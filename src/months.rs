/// Month names in order for range generation
pub fn get_month_order() -> Vec<&'static str> {
    vec!["january", "february", "march", "april", "may", "june",
         "july", "august", "september", "october", "november", "december"]
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