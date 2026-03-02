use crate::analysis::{counter, entropy, ngram, tokenizer};
use crate::output::ResultTable;
use crate::utils::format::format_num;
use anyhow::Result;

pub fn run(text: &str, source_name: &str) -> Result<ResultTable> {
    let unigram_freqs = counter::word_frequencies(text);
    let words = tokenizer::words(text);
    let bigram_freqs = ngram::ngram_frequencies(&words, 2);
    let trigram_freqs = ngram::ngram_frequencies(&words, 3);

    let h1 = entropy::shannon_entropy(&unigram_freqs);
    let h2 = entropy::shannon_entropy(&bigram_freqs);
    let h3 = entropy::shannon_entropy(&trigram_freqs);
    let rate = entropy::entropy_rate(h2, h3);
    let vocab_size = unigram_freqs.len();
    let redund = entropy::redundancy(rate, vocab_size);

    let mut table = ResultTable::new(source_name, vec!["Metric", "Value"]);
    table.add_row(vec!["H1 (Unigram Entropy)".into(), format!("{:.4}", h1)]);
    table.add_row(vec!["H2 (Bigram Entropy)".into(), format!("{:.4}", h2)]);
    table.add_row(vec!["H3 (Trigram Entropy)".into(), format!("{:.4}", h3)]);
    table.add_row(vec!["Entropy Rate".into(), format!("{:.4}", rate)]);
    table.add_row(vec!["Vocabulary Size".into(), format_num(vocab_size)]);
    table.add_row(vec!["Redundancy".into(), format!("{:.4}", redund)]);

    Ok(table)
}
