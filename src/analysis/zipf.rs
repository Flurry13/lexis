/// Compute Zipf exponent (alpha) via least-squares regression on log-log data.
/// Returns (alpha, r_squared). Alpha is returned as positive.
/// Input: slice of (rank, frequency) pairs, must be sorted by rank ascending.
pub fn zipf_exponent(rank_freq: &[(usize, usize)]) -> (f64, f64) {
    let points: Vec<(f64, f64)> = rank_freq
        .iter()
        .filter(|&&(r, f)| r > 0 && f > 0)
        .map(|&(r, f)| ((r as f64).ln(), (f as f64).ln()))
        .collect();

    if points.len() < 2 {
        return (0.0, 0.0);
    }

    let n = points.len() as f64;
    let sum_x: f64 = points.iter().map(|(x, _)| x).sum();
    let sum_y: f64 = points.iter().map(|(_, y)| y).sum();
    let sum_xy: f64 = points.iter().map(|(x, y)| x * y).sum();
    let sum_x2: f64 = points.iter().map(|(x, _)| x * x).sum();

    let denom = n * sum_x2 - sum_x * sum_x;
    if denom.abs() < f64::EPSILON {
        return (0.0, 0.0);
    }

    let slope = (n * sum_xy - sum_x * sum_y) / denom;
    let alpha = -slope; // Zipf's law: freq ~ rank^(-alpha), so slope is negative

    // R-squared
    let mean_y = sum_y / n;
    let ss_tot: f64 = points.iter().map(|(_, y)| (y - mean_y).powi(2)).sum();
    let intercept = (sum_y - slope * sum_x) / n;
    let ss_res: f64 = points
        .iter()
        .map(|(x, y)| {
            let predicted = slope * x + intercept;
            (y - predicted).powi(2)
        })
        .sum();

    let r_squared = if ss_tot > 0.0 {
        1.0 - ss_res / ss_tot
    } else {
        0.0
    };

    (alpha, r_squared)
}

/// Render a sparkline from frequency values using Unicode block characters.
pub fn sparkline(values: &[usize], width: usize) -> String {
    if values.is_empty() || width == 0 {
        return String::new();
    }

    let blocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    // Bucket values into `width` bins
    let buckets = if values.len() <= width {
        values.to_vec()
    } else {
        let chunk_size = values.len() as f64 / width as f64;
        (0..width)
            .map(|i| {
                let start = (i as f64 * chunk_size) as usize;
                let end = (((i + 1) as f64 * chunk_size) as usize).min(values.len());
                values[start..end].iter().sum::<usize>() / (end - start).max(1)
            })
            .collect()
    };

    let max = *buckets.iter().max().unwrap_or(&1);
    if max == 0 {
        return blocks[0].to_string().repeat(buckets.len());
    }

    buckets
        .iter()
        .map(|&v| {
            let idx = (v as f64 / max as f64 * 7.0).round() as usize;
            blocks[idx.min(7)]
        })
        .collect()
}
