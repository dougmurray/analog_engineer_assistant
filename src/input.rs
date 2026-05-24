/// Parse a value string that may include SI prefixes or scientific notation.
/// Examples: "1k" → 1000.0, "4.7u" → 4.7e-6, "1e-9" → 1e-9, "100m" → 0.1
pub fn parse_value(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // Check for trailing SI prefix (only if last char isn't part of exponent)
    let (num_str, multiplier) = if let Some(last) = s.chars().last() {
        // Don't treat 'e'/'E' suffix as SI — it's part of sci notation
        let has_exp = s.to_lowercase().contains('e');
        match last {
            'p' if !has_exp => (&s[..s.len()-1], 1e-12),
            'n' if !has_exp => (&s[..s.len()-1], 1e-9),
            'u' if !has_exp => (&s[..s.len()-1], 1e-6),
            'm' if !has_exp => (&s[..s.len()-1], 1e-3),
            'k' | 'K' if !has_exp => (&s[..s.len()-1], 1e3),
            'M' if !has_exp => (&s[..s.len()-1], 1e6),
            'G' if !has_exp => (&s[..s.len()-1], 1e9),
            'T' if !has_exp => (&s[..s.len()-1], 1e12),
            _ => (s, 1.0),
        }
    } else {
        return None;
    };

    num_str.parse::<f64>().ok().map(|v| v * multiplier)
}

pub fn format_eng(value: f64) -> String {
    if !value.is_finite() {
        return format!("{}", value);
    }
    let abs = value.abs();
    if abs == 0.0 {
        return "0".to_string();
    }

    let (scaled, suffix) = if abs >= 1e12 {
        (value / 1e12, "T")
    } else if abs >= 1e9 {
        (value / 1e9, "G")
    } else if abs >= 1e6 {
        (value / 1e6, "M")
    } else if abs >= 1e3 {
        (value / 1e3, "k")
    } else if abs >= 1.0 {
        (value, "")
    } else if abs >= 1e-3 {
        (value / 1e-3, "m")
    } else if abs >= 1e-6 {
        (value / 1e-6, "μ")
    } else if abs >= 1e-9 {
        (value / 1e-9, "n")
    } else if abs >= 1e-12 {
        (value / 1e-12, "p")
    } else {
        (value, "")
    };

    // Trim trailing zeros after decimal
    let s = format!("{:.4}", scaled);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    format!("{}{}", s, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_si_prefixes() {
        assert!((parse_value("1k").unwrap() - 1000.0).abs() < 1e-9);
        assert!((parse_value("4.7u").unwrap() - 4.7e-6).abs() < 1e-18);
        assert!((parse_value("100n").unwrap() - 100e-9).abs() < 1e-20);
        assert!((parse_value("1e-3").unwrap() - 1e-3).abs() < 1e-15);
        assert!((parse_value("3.3").unwrap() - 3.3).abs() < 1e-9);
    }
}
