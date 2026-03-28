use std::collections::HashSet;

/// Generate all password combinations
pub fn generate_passwords(
    word_variations: Vec<String>,
    number_values: Vec<String>,
) -> Vec<String> {
    let mut passwords = Vec::new();
    
    // Combine each word variation with "Example" and each number
    for word in &word_variations {
        for num in &number_values {
            let pwd = format!("{}Example{}", word, num);
            passwords.push(pwd);
            
            // Also add lowercase variant if different
            let lower_pwd = format!("{}Example{}", word.to_lowercase(), num);
            if !passwords.contains(&lower_pwd) && word != &word.to_lowercase() {
                passwords.push(lower_pwd);
            }
        }
    }
    
    // Sort by length, then alphabetically
    passwords.sort_by(|a, b| {
        a.len().cmp(&b.len()).then_with(|| a.cmp(b))
    });
    
    // Remove duplicates
    let mut seen = HashSet::new();
    passwords.retain(|p| seen.insert(p.clone()));
    
    passwords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_passwords_basic() {
        let word_variations = vec!["Jan".to_string()];
        let number_values = vec!["001".to_string(), "002".to_string()];
        
        let passwords = generate_passwords(word_variations, number_values);
        
        assert!(passwords.contains(&"JanExample001".to_string()));
        assert!(passwords.contains(&"JanExample002".to_string()));
    }

    #[test]
    fn test_generate_passwords_sorted_by_length() {
        let word_variations = vec!["A".to_string(), "ABC".to_string()];
        let number_values = vec!["01".to_string()];
        
        let passwords = generate_passwords(word_variations, number_values);
        
        // Check that shorter passwords come first
        for i in 0..passwords.len() - 1 {
            assert!(passwords[i].len() <= passwords[i + 1].len());
        }
    }

    #[test]
    fn test_generate_passwords_removes_duplicates() {
        let word_variations = vec!["Test".to_string(), "test".to_string()];
        let number_values = vec!["001".to_string()];
        
        let passwords = generate_passwords(word_variations, number_values);
        
        // Count occurrences of "testExample001"
        let count = passwords.iter().filter(|&p| p == "testExample001").count();
        assert_eq!(count, 1); // Should only appear once
    }

    #[test]
    fn test_full_workflow_month_range() {
        let months = super::super::months::generate_month_range("january", "march");
        assert_eq!(months.len(), 6); // jan, Jan, feb, Feb, mar, Mar
        
        for month in &months {
            assert!(month == "january" || month == "January"
                || month == "february" || month == "February"
                || month == "march" || month == "March");
        }
    }
}