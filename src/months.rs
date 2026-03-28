/// Month names in order for range generation
pub fn get_month_order() -> Vec<&'static str> {
    vec!["january", "february", "march", "april", "may", "june",
         "july", "august", "september", "october", "november", "december"]
}

/// Generate month range from begin to end (inclusive)
pub fn generate_month_range(begin: &str, end: &str) -> Vec<String> {
    let months = get_month_order();
    let begin_lower = begin.to_lowercase();
    let end_lower = end.to_lowercase();
    
    let start_idx = months.iter().position(|m| m == &begin_lower);
    let end_idx = months.iter().position(|m| m == &end_lower);
    
    if let (Some(s), Some(e)) = (start_idx, end_idx) {
        let mut range = Vec::new();
        for i in s..=e {
            range.push(months[i].to_string());
            // Also add capitalized version
            let cap: String = months[i].chars().next().unwrap().to_uppercase().collect::<String>() + &months[i][1..];
            if cap != months[i] {
                range.push(cap);
            }
        }
        range
    } else {
        // Fallback to all months if parsing fails
        let mut result = Vec::new();
        for m in months {
            result.push(m.to_string());
            let cap: String = m.chars().next().unwrap().to_uppercase().collect::<String>() + &m[1..];
            result.push(cap);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_month_order_returns_twelve_months() {
        let months = get_month_order();
        assert_eq!(months.len(), 12);
        assert_eq!(months[0], "january");
        assert_eq!(months[11], "december");
    }

    #[test]
    fn test_get_month_order_in_correct_order() {
        let months = get_month_order();
        let expected = vec![
            "january", "february", "march", "april", "may", "june",
            "july", "august", "september", "october", "november", "december"
        ];
        assert_eq!(months, expected);
    }

    #[test]
    fn test_generate_month_range_valid() {
        let range = generate_month_range("january", "march");
        // january, January, february, February, march, March
        assert!(range.contains(&"january".to_string()));
        assert!(range.contains(&"January".to_string()));
        assert!(range.contains(&"february".to_string()));
        assert!(range.contains(&"February".to_string()));
        assert!(range.contains(&"march".to_string()));
        assert!(range.contains(&"March".to_string()));
    }

    #[test]
    fn test_generate_month_range_full_year() {
        let range = generate_month_range("january", "december");
        assert!(range.contains(&"january".to_string()));
        assert!(range.contains(&"december".to_string()));
    }

    #[test]
    fn test_generate_month_range_invalid_fallback() {
        // Invalid month names should fallback to all months
        let range = generate_month_range("invalid", "monday");
        assert_eq!(range.len(), 24); // 12 months * 2 (lowercase + capitalized)
    }
}