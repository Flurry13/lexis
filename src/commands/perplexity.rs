use crate::analysis::lm::{NgramLM, Smoothing};
use crate::analysis::tokenizer;
use crate::output::ResultTable;
use crate::utils::format::format_num;
use anyhow::Result;

pub fn run(
    text: &str,
    source_name: &str,
    order: usize,
    smoothing_name: &str,
    k: f64,
) -> Result<ResultTable> {
    let words = tokenizer::words(text);
    let token_refs: Vec<&str> = words.iter().copied().collect();

    let lm = NgramLM::train(&token_refs, order);
    let stats = lm.stats();

    let smoothing = match smoothing_name {
        "none" => Smoothing::None,
        "laplace" => Smoothing::AddK(k),
        "backoff" => Smoothing::StupidBackoff(0.4),
        _ => Smoothing::AddK(k),
    };

    let pp = lm.perplexity(&token_refs, &smoothing);

    let smoothing_label = match &smoothing {
        Smoothing::None => "None (MLE)".to_string(),
        Smoothing::AddK(k) => format!("Add-k (k={k})"),
        Smoothing::StupidBackoff(a) => format!("Stupid Backoff (α={a})"),
    };

    let mut table = ResultTable::new(source_name, vec!["Metric", "Value"]);
    table.add_row(vec!["Order".into(), stats.order.to_string()]);
    table.add_row(vec!["Vocabulary Size".into(), format_num(stats.vocab_size)]);
    for (i, count) in stats.ngram_counts.iter().enumerate() {
        let label = match i {
            0 => "Unigrams".to_string(),
            1 => "Bigrams".to_string(),
            2 => "Trigrams".to_string(),
            n => format!("{}-grams", n + 1),
        };
        table.add_row(vec![label, format_num(*count)]);
    }
    table.add_row(vec!["Smoothing".into(), smoothing_label]);
    table.add_row(vec!["Perplexity".into(), format!("{:.4}", pp)]);

    Ok(table)
}
