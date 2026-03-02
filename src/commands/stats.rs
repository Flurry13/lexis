use crate::analysis::{counter, tokenizer};
use crate::output::ResultTable;
use crate::utils::format::format_num;
use crate::utils::stopwords;
use anyhow::Result;
use rustc_hash::FxHashSet;

pub fn run(text: &str, source_name: &str, sw: Option<&FxHashSet<String>>) -> Result<ResultTable> {
    let freqs = counter::word_frequencies(text);
    let tokens = counter::token_count(&freqs);
    let sentences = tokenizer::sentence_count(text);
    let chars = tokenizer::char_count(text);

    // Apply stopword filtering if requested
    let (display_freqs, stopwords_removed) = if let Some(stopwords) = sw {
        let words = tokenizer::words(text);
        let filtered = stopwords::filter_words(&words, stopwords);
        let removed = words.len() - filtered.len();
        let mut ffreqs = rustc_hash::FxHashMap::default();
        for w in filtered {
            *ffreqs.entry(w.to_string()).or_insert(0) += 1;
        }
        (ffreqs, Some(removed))
    } else {
        (freqs, None)
    };

    let types = counter::type_count(&display_freqs);
    let hapax = counter::hapax_count(&display_freqs);
    let ttr = counter::type_token_ratio(&display_freqs);

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
        format!(
            "{} ({:.1}%)",
            format_num(hapax),
            if types > 0 {
                hapax as f64 / types as f64 * 100.0
            } else {
                0.0
            }
        ),
    ]);
    table.add_row(vec![
        "Avg Sentence Length".into(),
        format!("{:.1} words", avg_sentence_len),
    ]);

    if let Some(removed) = stopwords_removed {
        table.add_row(vec!["Stopwords Removed".into(), format_num(removed)]);
    }

    Ok(table)
}
