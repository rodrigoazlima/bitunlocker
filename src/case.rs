use std::collections::HashSet;

/// Generate all possible case combinations for a word (2^N combinations)
fn gen_all_cases_recursive(chars: &[char], current: String, results: &mut HashSet<String>) {
    if chars.is_empty() {
        results.insert(current);
        return;
    }

    let c = chars[0];
    let lower_c = c.to_ascii_lowercase();
    let upper_c = c.to_ascii_uppercase();

    // Add lowercase version
    gen_all_cases_recursive(&chars[1..], format!("{}{}", current, lower_c), results);

    // Add uppercase version if different from lowercase
    if lower_c != upper_c {
        gen_all_cases_recursive(&chars[1..], format!("{}{}", current, upper_c), results);
    }
}

/// Generate case variations based on case parameter
pub fn generate_case_variations(word: &str, case_mode: &str) -> HashSet<String> {
    let mut variations = HashSet::new();

    if case_mode == "all" {
        // Generate all 2^N combinations for any word length
        gen_all_cases_recursive(
            &word.chars().collect::<Vec<_>>(),
            String::new(),
            &mut variations,
        );
    } else {
        // Default: lowercase, uppercase, titlecase
        variations.insert(word.to_lowercase());
        variations.insert(word.to_uppercase());

        let mut title_case = word.to_string();
        if !title_case.is_empty() {
            // Safety: We just checked that the string is not empty, so next() will return Some
            let first = title_case
                .chars()
                .next()
                .expect("title_case is not empty, chars.next() must return Some")
                .to_uppercase()
                .collect::<String>();
            title_case = first + &word[1..];
        }
        variations.insert(title_case);
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
    fn test_generate_case_variations_all_mode_february() {
        // For "february" (8 chars): 2^8 = 256 combinations
        let variations = generate_case_variations("february", "all");

        assert_eq!(variations.len(), 256);
        assert!(variations.contains(&"february".to_string()));
        assert!(variations.contains(&"FEBRUARY".to_string()));
        assert!(variations.contains(&"February".to_string()));
        assert!(variations.contains(&"FeBrUaRy".to_string()));
    }

    #[test]
    fn test_generate_case_variations_all_mode_three_chars() {
        let variations = generate_case_variations("abc", "all");

        // 2^3 = 8 combinations for 3 character word
        assert_eq!(variations.len(), 8);
    }
}
