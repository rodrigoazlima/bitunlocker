use std::collections::HashSet;

/// Generate a range of numbers from begin to end (inclusive)
/// Supports zero-padded numbers like "001" to "333"
pub fn generate_number_range(begin: &str, end: &str) -> Vec<String> {
    let begin_val = begin.parse::<i64>().unwrap_or(0);
    let end_val = end.parse::<i64>().unwrap_or(0);

    let mut numbers = Vec::new();

    // Determine padding from the begin value (count digits including leading zeros)
    let padding = begin.len();

    for i in begin_val..=end_val {
        let num_str = if padding > 0 {
            format!("{:0padding$}", i, padding = padding)
        } else {
            i.to_string()
        };
        numbers.push(num_str);
    }

    // Remove duplicates
    let mut seen = HashSet::new();
    numbers.retain(|n| seen.insert(n.clone()));

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_number_range_basic() {
        let nums = generate_number_range("1", "5");
        assert_eq!(nums.len(), 5);
        assert_eq!(nums[0], "1");
        assert_eq!(nums[4], "5");
    }

    #[test]
    fn test_generate_number_range_with_padding() {
        let nums = generate_number_range("001", "005");
        assert_eq!(nums.len(), 5);
        assert_eq!(nums[0], "001");
        assert_eq!(nums[4], "005");
    }

    #[test]
    fn test_generate_number_range_large() {
        let nums = generate_number_range("1991", "2000");
        assert_eq!(nums.len(), 10);
        assert_eq!(nums[0], "1991");
        assert_eq!(nums[4], "1995");
        assert_eq!(nums[9], "2000");
    }
}
