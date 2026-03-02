use crate::analysis::readability;
use crate::output::ResultTable;
use anyhow::Result;

pub fn run(text: &str, source_name: &str) -> Result<ResultTable> {
    let m = readability::compute_metrics(text);

    let fk_grade = readability::flesch_kincaid_grade(&m);
    let fre = readability::flesch_reading_ease(&m);
    let cl = readability::coleman_liau(&m);
    let fog = readability::gunning_fog(&m);
    let smog = readability::smog(&m);

    let mut table = ResultTable::new(source_name, vec!["Metric", "Score", "Grade"]);

    table.add_row(vec![
        "Flesch-Kincaid Grade".into(),
        format!("{:.2}", fk_grade),
        readability::grade_label(fk_grade).into(),
    ]);
    table.add_row(vec![
        "Flesch Reading Ease".into(),
        format!("{:.2}", fre),
        readability::ease_label(fre).into(),
    ]);
    table.add_row(vec![
        "Coleman-Liau Index".into(),
        format!("{:.2}", cl),
        readability::grade_label(cl).into(),
    ]);
    table.add_row(vec![
        "Gunning Fog Index".into(),
        format!("{:.2}", fog),
        readability::grade_label(fog).into(),
    ]);
    table.add_row(vec![
        "SMOG Index".into(),
        format!("{:.2}", smog),
        readability::grade_label(smog).into(),
    ]);

    Ok(table)
}
