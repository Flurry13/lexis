use crate::analysis::tokenizer;

/// Precomputed text metrics for readability formulas.
pub struct TextMetrics {
    pub word_count: usize,
    pub sentence_count: usize,
    pub syllable_count: usize,
    pub char_count: usize,
    pub complex_word_count: usize,
}

/// Compute all metrics needed for readability formulas in a single pass.
pub fn compute_metrics(text: &str) -> TextMetrics {
    let words = tokenizer::words(text);
    let sentence_count = tokenizer::sentence_count(text);

    let mut syllable_total = 0;
    let mut char_total = 0;
    let mut complex_count = 0;

    for word in &words {
        let syls = tokenizer::syllable_count(word);
        syllable_total += syls;
        char_total += word.chars().filter(|c| c.is_alphabetic()).count();
        if syls >= 3 {
            complex_count += 1;
        }
    }

    TextMetrics {
        word_count: words.len(),
        sentence_count,
        syllable_count: syllable_total,
        char_count: char_total,
        complex_word_count: complex_count,
    }
}

/// Flesch-Kincaid Grade Level.
pub fn flesch_kincaid_grade(m: &TextMetrics) -> f64 {
    if m.word_count == 0 || m.sentence_count == 0 {
        return 0.0;
    }
    let asl = m.word_count as f64 / m.sentence_count as f64;
    let asw = m.syllable_count as f64 / m.word_count as f64;
    0.39 * asl + 11.8 * asw - 15.59
}

/// Flesch Reading Ease (0–100 scale, higher = easier).
pub fn flesch_reading_ease(m: &TextMetrics) -> f64 {
    if m.word_count == 0 || m.sentence_count == 0 {
        return 0.0;
    }
    let asl = m.word_count as f64 / m.sentence_count as f64;
    let asw = m.syllable_count as f64 / m.word_count as f64;
    206.835 - 1.015 * asl - 84.6 * asw
}

/// Coleman-Liau Index.
pub fn coleman_liau(m: &TextMetrics) -> f64 {
    if m.word_count == 0 {
        return 0.0;
    }
    let l = m.char_count as f64 / m.word_count as f64 * 100.0;
    let s = m.sentence_count as f64 / m.word_count as f64 * 100.0;
    0.0588 * l - 0.296 * s - 15.8
}

/// Gunning Fog Index.
pub fn gunning_fog(m: &TextMetrics) -> f64 {
    if m.word_count == 0 || m.sentence_count == 0 {
        return 0.0;
    }
    let asl = m.word_count as f64 / m.sentence_count as f64;
    let pcw = m.complex_word_count as f64 / m.word_count as f64 * 100.0;
    0.4 * (asl + pcw)
}

/// SMOG Index.
pub fn smog(m: &TextMetrics) -> f64 {
    if m.sentence_count == 0 {
        return 0.0;
    }
    let ratio = m.complex_word_count as f64 * 30.0 / m.sentence_count as f64;
    3.0 + ratio.sqrt()
}

/// Map a grade-level score to a human-readable label.
pub fn grade_label(score: f64) -> &'static str {
    if score < 6.0 {
        "Elementary"
    } else if score < 9.0 {
        "Middle School"
    } else if score < 13.0 {
        "High School"
    } else if score < 17.0 {
        "College"
    } else {
        "Graduate"
    }
}

/// Map a Flesch Reading Ease score to a descriptive label.
pub fn ease_label(score: f64) -> &'static str {
    if score >= 90.0 {
        "Very Easy"
    } else if score >= 80.0 {
        "Easy"
    } else if score >= 70.0 {
        "Fairly Easy"
    } else if score >= 60.0 {
        "Standard"
    } else if score >= 50.0 {
        "Fairly Difficult"
    } else if score >= 30.0 {
        "Difficult"
    } else {
        "Very Difficult"
    }
}
