use crate::analysis::tokenizer;
use crate::output::ResultTable;
use crate::utils::format::format_num;
use anyhow::Result;

pub fn run(text: &str, source_name: &str) -> Result<ResultTable> {
    let words = tokenizer::words(text);
    let sentences = tokenizer::sentence_count(text);
    let chars = tokenizer::char_count(text);

    let whitespace_tokens = words.len();

    let mut table = ResultTable::new(source_name, vec!["Tokenizer", "Tokens"]);
    table.add_row(vec!["Whitespace".into(), format_num(whitespace_tokens)]);
    table.add_row(vec!["Sentences".into(), format_num(sentences)]);
    table.add_row(vec!["Characters".into(), format_num(chars)]);

    Ok(table)
}
