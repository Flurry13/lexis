use crate::analysis::ngram::ngram_frequencies;
use rustc_hash::FxHashMap;

/// Smoothing strategy for n-gram probability estimation.
#[derive(Clone, Debug)]
pub enum Smoothing {
    /// Maximum likelihood estimation — zero for unseen n-grams.
    None,
    /// Add-k smoothing (Laplace when k=1.0).
    AddK(f64),
    /// Stupid Backoff with decay factor alpha (typically 0.4).
    StupidBackoff(f64),
}

/// Summary statistics for a trained language model.
pub struct LMStats {
    pub order: usize,
    pub vocab_size: usize,
    /// Number of distinct n-grams at each order (index 0 = unigrams).
    pub ngram_counts: Vec<usize>,
}

/// An n-gram language model trained on a token sequence.
pub struct NgramLM {
    order: usize,
    /// counts[i] holds (i+1)-gram frequencies. counts[0] = unigrams.
    counts: Vec<FxHashMap<String, usize>>,
    total_unigrams: usize,
    vocab_size: usize,
}

impl NgramLM {
    /// Train an n-gram LM of the given order from a token slice.
    pub fn train(tokens: &[&str], order: usize) -> Self {
        let order = order.max(1);
        let mut counts = Vec::with_capacity(order);
        for n in 1..=order {
            counts.push(ngram_frequencies(tokens, n));
        }
        let total_unigrams: usize = counts[0].values().sum();
        let vocab_size = counts[0].len();
        Self {
            order,
            counts,
            total_unigrams,
            vocab_size,
        }
    }

    /// Probability of `word` given `context` under the specified smoothing.
    pub fn prob(&self, word: &str, context: &[&str], smoothing: &Smoothing) -> f64 {
        match smoothing {
            Smoothing::None => self.prob_mle(word, context),
            Smoothing::AddK(k) => self.prob_add_k(word, context, *k),
            Smoothing::StupidBackoff(alpha) => self.prob_backoff(word, context, *alpha),
        }
    }

    /// Log2 probability of `word` given `context`.
    pub fn log_prob(&self, word: &str, context: &[&str], smoothing: &Smoothing) -> f64 {
        let p = self.prob(word, context, smoothing);
        if p <= 0.0 {
            f64::NEG_INFINITY
        } else {
            p.log2()
        }
    }

    /// Perplexity of a token sequence: PP = 2^(-1/N * sum(log2 P(w_i | context))).
    pub fn perplexity(&self, tokens: &[&str], smoothing: &Smoothing) -> f64 {
        if tokens.is_empty() {
            return f64::INFINITY;
        }

        let mut total_log_prob = 0.0;
        let ctx_len = self.order - 1;

        for i in 0..tokens.len() {
            let start = if i >= ctx_len { i - ctx_len } else { 0 };
            let context = &tokens[start..i];
            total_log_prob += self.log_prob(tokens[i], context, smoothing);
        }

        let avg = -total_log_prob / tokens.len() as f64;
        2.0_f64.powf(avg)
    }

    /// Summary statistics about this model.
    pub fn stats(&self) -> LMStats {
        LMStats {
            order: self.order,
            vocab_size: self.vocab_size,
            ngram_counts: self.counts.iter().map(|c| c.len()).collect(),
        }
    }

    // --- private helpers ---

    /// MLE: C(context word) / C(context). Returns 0 for unseen.
    fn prob_mle(&self, word: &str, context: &[&str]) -> f64 {
        let n = context.len() + 1;
        if n == 1 {
            // Unigram
            let count = self.counts[0].get(word).copied().unwrap_or(0);
            if self.total_unigrams == 0 {
                return 0.0;
            }
            return count as f64 / self.total_unigrams as f64;
        }

        let ngram_order = n.min(self.order);
        let ctx_start = if context.len() > ngram_order - 1 {
            context.len() - (ngram_order - 1)
        } else {
            0
        };
        let effective_ctx = &context[ctx_start..];

        // Build the full n-gram key
        let mut ngram_key = effective_ctx.join(" ");
        ngram_key.push(' ');
        ngram_key.push_str(word);

        let ngram_count = self.counts[effective_ctx.len()]
            .get(&ngram_key)
            .copied()
            .unwrap_or(0);

        // Context count
        let ctx_count = if effective_ctx.is_empty() {
            self.total_unigrams
        } else {
            let ctx_key = effective_ctx.join(" ");
            self.counts[effective_ctx.len() - 1]
                .get(&ctx_key)
                .copied()
                .unwrap_or(0)
        };

        if ctx_count == 0 {
            0.0
        } else {
            ngram_count as f64 / ctx_count as f64
        }
    }

    /// Add-k smoothing: (C(ngram) + k) / (C(context) + k * V).
    fn prob_add_k(&self, word: &str, context: &[&str], k: f64) -> f64 {
        let n = context.len() + 1;
        if n == 1 {
            let count = self.counts[0].get(word).copied().unwrap_or(0);
            return (count as f64 + k) / (self.total_unigrams as f64 + k * self.vocab_size as f64);
        }

        let ngram_order = n.min(self.order);
        let ctx_start = if context.len() > ngram_order - 1 {
            context.len() - (ngram_order - 1)
        } else {
            0
        };
        let effective_ctx = &context[ctx_start..];

        let mut ngram_key = effective_ctx.join(" ");
        ngram_key.push(' ');
        ngram_key.push_str(word);

        let ngram_count = self.counts[effective_ctx.len()]
            .get(&ngram_key)
            .copied()
            .unwrap_or(0);

        let ctx_count = if effective_ctx.is_empty() {
            self.total_unigrams
        } else {
            let ctx_key = effective_ctx.join(" ");
            self.counts[effective_ctx.len() - 1]
                .get(&ctx_key)
                .copied()
                .unwrap_or(0)
        };

        (ngram_count as f64 + k) / (ctx_count as f64 + k * self.vocab_size as f64)
    }

    /// Stupid Backoff: use highest-order MLE if available, else back off with alpha decay.
    fn prob_backoff(&self, word: &str, context: &[&str], alpha: f64) -> f64 {
        let max_ctx = context.len().min(self.order - 1);

        // Try from longest context down to unigram
        for ctx_len in (0..=max_ctx).rev() {
            let ctx_start = context.len() - ctx_len;
            let effective_ctx = &context[ctx_start..];

            let p = self.prob_mle(word, effective_ctx);
            if p > 0.0 {
                // Apply alpha penalty for each backoff step
                let backoff_steps = max_ctx - ctx_len;
                return p * alpha.powi(backoff_steps as i32);
            }
        }

        // Absolute fallback: uniform over vocab
        if self.vocab_size > 0 {
            let backoff_steps = max_ctx + 1;
            alpha.powi(backoff_steps as i32) / self.vocab_size as f64
        } else {
            0.0
        }
    }
}
