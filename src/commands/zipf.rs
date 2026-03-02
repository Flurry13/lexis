use crate::analysis::{counter, zipf};
use crate::output::ResultTable;
use crate::utils::format::format_num;
use anyhow::Result;

pub fn run(text: &str, source_name: &str, top: usize, plot: bool) -> Result<ResultTable> {
    let freqs = counter::word_frequencies(text);
    let sorted = counter::top_n(&freqs, freqs.len());

    let rank_freq: Vec<(usize, usize)> = sorted
        .iter()
        .enumerate()
        .map(|(i, &(_, f))| (i + 1, f))
        .collect();

    let (alpha, r_squared) = zipf::zipf_exponent(&rank_freq);

    if plot {
        let values: Vec<usize> = sorted.iter().take(top).map(|&(_, f)| f).collect();
        let spark = zipf::sparkline(&values, 40);

        let mut table = ResultTable::new(source_name, vec!["Metric", "Value"]);
        table.add_row(vec!["Zipf Exponent (α)".into(), format!("{:.4}", alpha)]);
        table.add_row(vec!["R²".into(), format!("{:.4}", r_squared)]);
        table.add_row(vec!["Vocabulary Size".into(), format_num(freqs.len())]);
        table.add_row(vec!["Distribution".into(), spark]);
        Ok(table)
    } else {
        let display_count = top.min(sorted.len());
        let mut table = ResultTable::new(source_name, vec!["Rank", "Word", "Frequency"]);
        for (i, &(word, freq)) in sorted.iter().take(display_count).enumerate() {
            table.add_row(vec![
                format_num(i + 1),
                word.to_string(),
                format_num(freq),
            ]);
        }
        table.add_row(vec![
            "—".into(),
            format!("α={:.4}, R²={:.4}", alpha, r_squared),
            format_num(freqs.len()) + " types",
        ]);
        Ok(table)
    }
}
