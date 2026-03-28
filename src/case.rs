use std::collections::HashSet;

/// Generate case variations based on case parameter
pub fn generate_case_variations(word: &str, case_mode: &str) -> HashSet<String> {
    let mut variations = HashSet::new();
    variations.insert(word.to_lowercase());
    variations.insert(word.to_uppercase());
    variations.insert({
        let mut s = word.to_string();
        if !s.is_empty() {
            let first = s.chars().next().unwrap().to_uppercase().collect::<String>();
            s = first + &word[1..];
        }
        s
    });
    
    if case_mode == "all" {
        // Generate all combinations - for short words only (2^N possibilities)
        if word.len() <= 4 {
            fn gen_all_cases(
                chars: &[char],
                current: String,
                results: &mut HashSet<String>,
            ) {
                if chars.is_empty() {
                    results.insert(current);
                    return;
                }
                let c = chars[0];
                let lower_c = c.to_ascii_lowercase();
                let upper_c = c.to_ascii_uppercase();
                
                gen_all_cases(&chars[1..], format!("{}{}", current, lower_c), results);
                if lower_c != upper_c {
                    gen_all_cases(&chars[1..], format!("{}{}", current, upper_c), results);
                }
            }
            let chars: Vec<char> = word.chars().collect();
            gen_all_cases(&chars, String::new(), &mut variations);
        }
    }
    
    variations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_case_variations_mixed_mode() {
        let variations = generate_case_variations("test", "mixed");
        
        // Should include: lowercase, uppercase, titlecase
        assert!(variations.contains(&"test".to_string()));
        assert!(variations.contains(&"TEST".to_string()));
        assert!(variations.contains(&"Test".to_string()));
    }

    #[test]
    fn test_generate_case_variations_all_mode_short_word() {
        let variations = generate_case_variations("ab", "all");
        
        // For 2-character word with all mode: 2^2 = 4 combinations
        assert!(variations.contains(&"ab".to_string()));
        assert!(variations.contains(&"aB".to_string()));
        assert!(variations.contains(&"Ab".to_string()));
        assert!(variations.contains(&"AB".to_string()));
    }

    #[test]
    fn test_generate_case_variations_all_mode_long_word() {
        // For words longer than 4 characters, all mode should only do default cases
        let variations = generate_case_variations("hello", "all");
        
        assert!(variations.contains(&"hello".to_string()));
        assert!(variations.contains(&"HELLO".to_string()));
        assert!(variations.contains(&"Hello".to_string()));
    }

    #[test]
    fn test_generate_case_variations_all_mode_three_chars() {
        let variations = generate_case_variations("abc", "all");
        
        // 2^3 = 8 combinations for 3 character word
        assert_eq!(variations.len(), 8);
    }
}