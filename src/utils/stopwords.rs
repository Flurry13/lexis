use rustc_hash::FxHashSet;
use std::path::Path;

/// Load stopwords from a file. One word per line, # comments and blank lines skipped.
pub fn load_stopwords(path: &Path) -> std::io::Result<FxHashSet<String>> {
    let content = std::fs::read_to_string(path)?;
    Ok(parse_stopwords(&content))
}

/// Parse stopword content into a set.
fn parse_stopwords(content: &str) -> FxHashSet<String> {
    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|word| word.to_lowercase())
        .collect()
}

/// Default English stopwords bundled with the binary.
pub fn default_english() -> FxHashSet<String> {
    parse_stopwords(include_str!("../../data/stopwords/english.txt"))
}

/// Filter words, removing any that appear in the stopword set (case-insensitive).
pub fn filter_words<'a>(words: &[&'a str], stopwords: &FxHashSet<String>) -> Vec<&'a str> {
    words
        .iter()
        .filter(|w| !stopwords.contains(&w.to_lowercase()))
        .copied()
        .collect()
}
