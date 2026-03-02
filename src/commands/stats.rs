use crate::analysis::{counter, tokenizer};
use crate::output::ResultTable;
use crate::utils::format::format_num;
use anyhow::Result;

pub fn run(text: &str, source_name: &str) -> Result<ResultTable> {
    let freqs = counter::word_frequencies(text);
    let tokens = counter::token_count(&freqs);
    let types = counter::type_count(&freqs);
    let hapax = counter::hapax_count(&freqs);
    let ttr = counter::type_token_ratio(&freqs);
    let sentences = tokenizer::sentence_count(text);
    let chars = tokenizer::char_count(text);

    let avg_sentence_len = if sentences > 0 {
        tokens as f64 / sentences as f64
    } else {
        0.0
    };

    let mut table = ResultTable::new(source_name, vec!["Metric", "Value"]);
    table.add_row(vec!["Tokens (words)".into(), format_num(tokens)]);
    table.add_row(vec!["Types (unique)".into(), format_num(types)]);
    table.add_row(vec!["Characters".into(), format_num(chars)]);
    table.add_row(vec!["Sentences".into(), format_num(sentences)]);
    table.add_row(vec!["Type-Token Ratio".into(), format!("{:.4}", ttr)]);
    table.add_row(vec![
        "Hapax Legomena".into(),
        format!("{} ({:.1}%)", format_num(hapax), if types > 0 { hapax as f64 / types as f64 * 100.0 } else { 0.0 }),
    ]);
    table.add_row(vec![
        "Avg Sentence Length".into(),
        format!("{:.1} words", avg_sentence_len),
    ]);

    Ok(table)
}
