/// Month names in order for range generation
pub fn get_month_order() -> Vec<&'static str> {
    vec![
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ]
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
            "january",
            "february",
            "march",
            "april",
            "may",
            "june",
            "july",
            "august",
            "september",
            "october",
            "november",
            "december",
        ];
        assert_eq!(months, expected);
    }
}
