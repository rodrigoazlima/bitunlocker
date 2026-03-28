use std::collections::HashSet;

/// Generate numbers from 0 to max_value with specified min/max sizes
pub fn generate_numbers(_min_size: usize, max_size: usize) -> Vec<String> {
    let mut numbers = Vec::new();
    
    // Always use 3-digit padding for consistency (000-999)
    match max_size {
        1 => for i in 0..=9 { numbers.push(format!("{:03}", i)); },
        2 => for i in 0..=99 { numbers.push(format!("{:03}", i)); },
        _ => for i in 0..=999 { numbers.push(format!("{:03}", i)); },
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
    fn test_generate_numbers_max_size_1() {
        let numbers = generate_numbers(1, 1);
        assert_eq!(numbers.len(), 10);
        assert!(numbers.contains(&"000".to_string()));
        assert!(numbers.contains(&"009".to_string()));
    }

    #[test]
    fn test_generate_numbers_max_size_2() {
        let numbers = generate_numbers(1, 2);
        assert_eq!(numbers.len(), 100);
        assert!(numbers.contains(&"000".to_string()));
        assert!(numbers.contains(&"099".to_string()));
    }

    #[test]
    fn test_generate_numbers_max_size_3() {
        let numbers = generate_numbers(1, 3);
        assert_eq!(numbers.len(), 1000);
        assert!(numbers.contains(&"000".to_string()));
        assert!(numbers.contains(&"999".to_string()));
    }
}