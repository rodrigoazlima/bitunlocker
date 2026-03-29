/// Generate passwords from parsed template parts
/// This is a generic function that works with any template pattern
pub fn generate_passwords_from_parts(
    parts: &[crate::template::TemplatePart],
) -> Vec<String> {
    // Collect all values for each placeholder
    let mut values_by_part = Vec::new();
    
    for part in parts {
        let values = get_values_for_part(part);
        values_by_part.push(values);
    }
    
    // Generate Cartesian product of all values
    generate_combinations(&values_by_part)
}

/// Get the list of values for a template part
fn get_values_for_part(part: &crate::template::TemplatePart) -> Vec<String> {
    match part.kind.as_str() {
        "number" => {
            if let (Some(begin), Some(end)) = (&part.min_value, &part.max_value) {
                crate::numbers::generate_number_range(begin, end)
            } else {
                // Default: generate 0-99
                crate::numbers::generate_number_range("0", "99")
            }
        }
        "word" => {
            if let (Some(_begin), Some(_end)) = (&part.min_value, &part.max_value) {
                // Generate a range of words (alphabetical or numeric)
                vec!["".to_string()]
            } else {
                vec!["".to_string()]
            }
        }
        "month" => {
            let months = crate::months::get_month_order();
            let mut results = Vec::new();
            
            for month in &months {
                let cases = crate::case::generate_case_variations(month, &part.case_mode);
                for case in cases {
                    if part.leet_speak {
                        let leet_cases = crate::leet::apply_leet_variations(&case, &crate::leet::get_leet_map());
                        results.extend(leet_cases);
                    } else {
                        results.push(case);
                    }
                }
            }
            results
        }
        _ => {
            // Default: empty string for unknown types
            vec!["".to_string()]
        }
    }
}

/// Generate all combinations (Cartesian product) from lists of values
fn generate_combinations(values: &[Vec<String>]) -> Vec<String> {
    if values.is_empty() {
        return vec!["".to_string()];
    }
    
    let mut results = Vec::new();
    let mut current = Vec::with_capacity(values.len());
    
    fn backtrack(
        values: &[Vec<String>],
        index: usize,
        current: &mut Vec<String>,
        results: &mut Vec<String>,
    ) {
        if index == values.len() {
            results.push(current.concat());
            return;
        }
        
        for value in &values[index] {
            current.push(value.clone());
            backtrack(values, index + 1, current, results);
            current.pop();
        }
    }
    
    backtrack(values, 0, &mut current, &mut results);
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_combinations_two_parts() {
        let values = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ];
        let results = generate_combinations(&values);
        
        assert!(results.contains(&"a1".to_string()));
        assert!(results.contains(&"a2".to_string()));
        assert!(results.contains(&"b1".to_string()));
        assert!(results.contains(&"b2".to_string()));
    }
}