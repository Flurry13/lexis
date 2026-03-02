use crate::analysis::{ngram, tokenizer};
use crate::output::ResultTable;
use crate::utils::format::format_num;
use crate::utils::stopwords as sw_util;
use anyhow::Result;
use rustc_hash::FxHashSet;

pub fn run(
    text: &str,
    source_name: &str,
    n: usize,
    top: usize,
    min_freq: Option<usize>,
    case_insensitive: bool,
    stopwords: Option<&FxHashSet<String>>,
) -> Result<ResultTable> {
    anyhow::ensure!(n >= 1, "n-gram size must be at least 1");
    let words = tokenizer::words(text);

    // Apply stopword filtering before n-gram extraction
    let filtered: Vec<&str>;
    let words_ref = if let Some(sw) = stopwords {
        filtered = sw_util::filter_words(&words, sw);
        &filtered
    } else {
        &words
    };

    let freqs = if case_insensitive {
        let lowered: Vec<String> = words_ref.iter().map(|w| w.to_lowercase()).collect();
        let refs: Vec<&str> = lowered.iter().map(|s| s.as_str()).collect();
        ngram::ngram_frequencies(&refs, n)
    } else {
        ngram::ngram_frequencies(words_ref, n)
    };

    let total: usize = freqs.values().sum();

    let mut entries: Vec<(&str, usize)> = freqs.iter().map(|(k, &v)| (k.as_str(), v)).collect();

    if let Some(min) = min_freq {
        entries.retain(|&(_, freq)| freq >= min);
    }

    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    entries.truncate(top);

    let n_label = match n {
        1 => "Unigram",
        2 => "Bigram",
        3 => "Trigram",
        _ => "N-gram",
    };

    let mut table = ResultTable::new(source_name, vec![n_label, "Freq", "Rel %"]);
    for (ngram_str, freq) in entries {
        let pct = if total > 0 {
            freq as f64 / total as f64 * 100.0
        } else {
            0.0
        };
        table.add_row(vec![
            format!("\"{}\"", ngram_str),
            format_num(freq),
            format!("{:.2}%", pct),
        ]);
    }

    Ok(table)
}
