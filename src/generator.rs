/// Generate passwords from parsed template parts
/// This is a generic function that works with any template pattern
pub fn generate_passwords_from_parts(parts: &[crate::template::TemplatePart]) -> Vec<String> {
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
            // Check if shortened is requested - number doesn't support shortened
            let has_shortened = part.has_shortened_flag;
            
            if has_shortened {
                return vec!["Error: 'number' placeholder does not support 'shortened' modifier".to_string()];
            }
            
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
            let all_months = crate::months::get_month_order();
            
            // Get begin and end indices for filtering
            let mut start_idx = 0;
            let mut end_idx = all_months.len() - 1;

            if let Some(begin) = &part.begin_value {
                if let Some(idx) = all_months
                    .iter()
                    .position(|m| m.to_lowercase() == begin.to_lowercase())
                {
                    start_idx = idx;
                }
            }

            if let Some(end) = &part.end_value {
                if let Some(idx) = all_months
                    .iter()
                    .position(|m| m.to_lowercase() == end.to_lowercase())
                {
                    end_idx = idx;
                }
            }

            // Generate months in the range
            let mut results = Vec::new();

            for month in &all_months[start_idx..=end_idx] {
                // Check if shortened is requested - generate all subsequences of the month name
                let has_shortened = part.has_shortened_flag;
                
                if has_shortened {
                    // Generate all shortened versions using bitmask approach
                    let chars: Vec<char> = month.chars().collect();
                    let n = chars.len();
                    
                    for mask in 1..(1 << n) {  // Skip mask=0 (empty string)
                        let mut current = String::new();
                        for i in 0..n {
                            if mask & (1 << i) != 0 {
                                current.push(chars[i]);
                            }
                        }
                        
                        // Apply case variations
                        let cases = crate::case::generate_case_variations(&current, &part.case_mode);
                        if part.leet_speak {
                            for case in cases {
                                let leet_cases =
                                    crate::leet::apply_leet_variations(&case, &crate::leet::get_leet_map());
                                results.extend(leet_cases);
                            }
                        } else {
                            results.extend(cases);
                        }
                    }
                } else {
                    // Regular month generation without shortened
                    let cases = crate::case::generate_case_variations(month, &part.case_mode);
                    
                    for case in cases {
                        if part.leet_speak {
                            let leet_cases =
                                crate::leet::apply_leet_variations(&case, &crate::leet::get_leet_map());
                            results.extend(leet_cases);
                        } else {
                            results.push(case);
                        }
                    }
                }
            }

            // If begin/end not found or invalid range, return all months
            if results.is_empty() {
                for month in &all_months {
                    let cases = crate::case::generate_case_variations(month, &part.case_mode);
                    
                    for case in cases {
                        if part.leet_speak {
                            let leet_cases = crate::leet::apply_leet_variations(
                                &case,
                                &crate::leet::get_leet_map(),
                            );
                            results.extend(leet_cases);
                        } else {
                            results.push(case);
                        }
                    }
                }
            }

            // Remove duplicates using HashSet (O(1) lookup)
            let mut unique_results: Vec<String> = results.into_iter().collect();
            unique_results.sort();
            unique_results.dedup();
            
            unique_results
        }
        "shortened" => {
            // Get the source word from begin_value or min_value (alphabetic)
            let source_word = part.begin_value.as_deref()
                .or_else(|| {
                    part.min_value.as_deref().filter(|v| v.chars().all(|c| c.is_alphabetic()))
                })
                .unwrap_or("");
            
            if source_word.is_empty() {
                return vec![];
            }
            
            // Determine minimum length from min_value or default to 1
            let min_length = part.min_value.as_ref()
                .and_then(|v| v.parse::<usize>().ok())
                .map(|min_len| min_len.min(source_word.len()))
                .unwrap_or(1);

            let shortened = crate::words::generate_shortened(source_word, min_length);
            
            // Apply case variations to each shortened word
            let mut results = Vec::new();
            for word in &shortened {
                let cases = crate::case::generate_case_variations(word, &part.case_mode);
                if part.leet_speak {
                    for case in cases {
                        let leet_cases =
                            crate::leet::apply_leet_variations(&case, &crate::leet::get_leet_map());
                        results.extend(leet_cases);
                    }
                } else {
                    results.extend(cases);
                }
            }
            results
        }
        "extended" => {
            // Get the source word from begin_value or min_value (alphabetic)
            let source_word = part.begin_value.as_deref()
                .or_else(|| {
                    part.min_value.as_deref().filter(|v| v.chars().all(|c| c.is_alphabetic()))
                })
                .unwrap_or("");
            
            if source_word.is_empty() {
                return vec![];
            }
            
            // Determine maximum length from max_value or default to source_word.len() + 2
            let max_length = part.max_value.as_ref()
                .and_then(|v| v.parse::<usize>().ok())
                .map(|max_len| max_len.max(source_word.len()))
                .unwrap_or_else(|| {
                    // Default: extend up to 10 chars or source length + 2, whichever is larger
                    source_word.len().saturating_add(2).max(10)
                });

            let extended = crate::words::generate_extended(source_word, max_length);
            
            // Apply case variations to each extended word
            let mut results = Vec::new();
            for word in &extended {
                let cases = crate::case::generate_case_variations(word, &part.case_mode);
                if part.leet_speak {
                    for case_str in cases {
                        let leet_cases =
                            crate::leet::apply_leet_variations(&case_str, &crate::leet::get_leet_map());
                        results.extend(leet_cases);
                    }
                } else {
                    results.extend(cases);
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
pub fn generate_combinations(values: &[Vec<String>]) -> Vec<String> {
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

    #[test]
    fn test_get_values_for_part_shortened_with_august() {
        let part = crate::template::TemplatePart {
            kind: "shortened".to_string(),
            begin_value: Some("august".to_string()),
            end_value: None,
            min_value: Some("3".to_string()),
            max_value: None,
            leet_speak: false,
            case_mode: "mixed".to_string(),
            has_shortened_flag: false,
        };
        
        let values = get_values_for_part(&part);
        
        // Should include shortened versions of "august" with at least 3 chars
        assert!(values.contains(&"aug".to_string()));
        assert!(values.contains(&"augu".to_string()));
        assert!(values.contains(&"augus".to_string()));
        assert!(values.contains(&"august".to_string()));
    }

    #[test]
    fn test_get_values_for_part_extended_with_august() {
        let part = crate::template::TemplatePart {
            kind: "extended".to_string(),
            begin_value: Some("august".to_string()),
            end_value: None,
            min_value: None,
            max_value: Some("10".to_string()),
            leet_speak: false,
            case_mode: "mixed".to_string(),
            has_shortened_flag: false,
        };
        
        let values = get_values_for_part(&part);
        
        // Should include extended versions of "august" (more than 6 chars, up to 10)
        assert!(values.iter().any(|v| v.len() > 6 && v.len() <= 10));
    }

    #[test]
    fn test_get_values_for_part_shortened_with_case_all() {
        let part = crate::template::TemplatePart {
            kind: "shortened".to_string(),
            begin_value: Some("august".to_string()),
            end_value: None,
            min_value: Some("3".to_string()),
            max_value: None,
            leet_speak: false,
            case_mode: "all".to_string(),
            has_shortened_flag: false,
        };
        
        let values = get_values_for_part(&part);
        
        // With case=all, should have all 2^N combinations
        // For a 6-char word with at least 3 chars: should include many variations
        assert!(values.iter().any(|v| v.contains("aug")));
    }

    #[test]
    fn test_get_values_for_part_extended_with_case_all() {
        let part = crate::template::TemplatePart {
            kind: "extended".to_string(),
            begin_value: Some("cat".to_string()),
            end_value: None,
            min_value: None,
            max_value: Some("6".to_string()),
            leet_speak: false,
            case_mode: "all".to_string(),
            has_shortened_flag: false,
        };
        
        let values = get_values_for_part(&part);
        
        // Should include extended versions with all case combinations
        assert!(values.iter().any(|v| v.len() > 3 && v.len() <= 6));
    }

    #[test]
    fn test_get_values_for_part_shortened_with_leet_speak() {
        let part = crate::template::TemplatePart {
            kind: "shortened".to_string(),
            begin_value: Some("cat".to_string()),
            end_value: None,
            min_value: Some("2".to_string()),
            max_value: None,
            leet_speak: true,
            case_mode: "mixed".to_string(),
            has_shortened_flag: false,
        };
        
        let values = get_values_for_part(&part);
        
        // With leetSpeak, should include variations like "c@", "@t", etc.
        assert!(values.iter().any(|v| v.contains("@")));
    }

    #[test]
    fn test_get_values_for_part_month_with_shortened_lower() {
        // Test the exact scenario from the failing e2e test: {month,begin=may,end=may,case=lower,shortened}
        let part = crate::template::TemplatePart {
            kind: "month".to_string(),
            begin_value: Some("may".to_string()),
            end_value: Some("may".to_string()),
            min_value: None,
            max_value: None,
            leet_speak: false,
            case_mode: "lower".to_string(),
            has_shortened_flag: true,
        };
        
        let values = get_values_for_part(&part);
        
        println!("Generated {} values for may with shortened:", values.len());
        for v in &values {
            println!("  - {}", v);
        }
        
        // For "may" (3 chars) with case=lower and shortened, we should have:
        // m, a, y, ma, my, ay, may = 7 base forms, each lowercased
        assert!(values.contains(&"m".to_string()), "Single char 'm' should be present");
        assert!(values.contains(&"a".to_string()), "Single char 'a' should be present");
        assert!(values.contains(&"y".to_string()), "Single char 'y' should be present");
        assert!(values.contains(&"ma".to_string()), "Two chars 'ma' should be present");
        assert!(values.contains(&"my".to_string()), "Two chars 'my' should be present - THIS IS THE FAILING CHECK");
        assert!(values.contains(&"ay".to_string()), "Two chars 'ay' should be present");
        assert!(values.contains(&"may".to_string()), "Full word 'may' should be present");
    }

    #[test]
    fn test_get_values_for_part_number_with_shortened_should_error() {
        let part = crate::template::TemplatePart {
            kind: "number".to_string(),
            begin_value: None,
            end_value: None,
            min_value: Some("0".to_string()),
            max_value: Some("10".to_string()),
            leet_speak: false,
            case_mode: "mixed".to_string(),
            has_shortened_flag: true,  // This should trigger the error
        };
        
        let values = get_values_for_part(&part);
        
        // Should return error message when number has shortened flag
        assert!(values.len() == 1);
        assert!(values[0].contains("Error"));
    }

    #[test]
    fn test_get_values_for_part_extended_with_leet_speak() {
        let part = crate::template::TemplatePart {
            kind: "extended".to_string(),
            begin_value: Some("cat".to_string()),
            end_value: None,
            min_value: None,
            max_value: Some("6".to_string()),
            leet_speak: true,
            case_mode: "mixed".to_string(),
            has_shortened_flag: false,
        };
        
        let values = get_values_for_part(&part);
        
        // Should include extended versions with leet-speak
        assert!(values.iter().any(|v| v.len() > 3 && (v.contains("@") || v.contains("!"))));
    }
}
