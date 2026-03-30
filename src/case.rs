use std::collections::HashSet;

/// Generate camelCase variation (first lowercase, rest uppercase)
fn gen_camel_case(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    // First character is lowercase
    // Safety: We already checked that the string is not empty above, so next() returns Some
    result.push(
        word.chars()
            .next()
            .expect("camel_case word is not empty")
            .to_ascii_lowercase(),
    );

    // Rest of the characters are uppercase
    for c in word.chars().skip(1) {
        result.push(c.to_ascii_uppercase());
    }

    result
}

/// Generate snake_case variation (letters separated by underscores, all lowercase)
fn gen_snake_case(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    for c in word.chars() {
        if !result.is_empty() {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }

    result
}

/// Generate kebab-case variation (letters separated by hyphens, all lowercase)
fn gen_kebab_case(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    for c in word.chars() {
        if !result.is_empty() {
            result.push('-');
        }
        result.push(c.to_ascii_lowercase());
    }

    result
}

/// Generate SCREAM_SNAKE_CASE variation (letters separated by underscores, all uppercase)
fn gen_scream_case(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    for c in word.chars() {
        if !result.is_empty() {
            result.push('_');
        }
        result.push(c.to_ascii_uppercase());
    }

    result
}

/// Generate PascalCase variation (each word starts with uppercase)
fn gen_pascal_case(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    // Safety: We just checked that the string is not empty, so next() will return Some
    let first_char = word
        .chars()
        .next()
        .expect("pascal_case word is not empty")
        .to_uppercase()
        .collect::<String>();

    result.push_str(&first_char);

    // Rest of characters in lowercase
    for c in word.chars().skip(1) {
        result.push(c.to_ascii_lowercase());
    }

    result
}

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

    match case_mode {
        "all" => {
            // Generate all 2^N combinations for any word length
            gen_all_cases_recursive(
                &word.chars().collect::<Vec<_>>(),
                String::new(),
                &mut variations,
            );
        }
        "camel" => {
            variations.insert(gen_camel_case(word));
        }
        "snake" => {
            variations.insert(gen_snake_case(word));
        }
        "kebab" => {
            variations.insert(gen_kebab_case(word));
        }
        "scream" => {
            variations.insert(gen_scream_case(word));
        }
        "pascal" => {
            variations.insert(gen_pascal_case(word));
        }
        _ => {
            // Default: lowercase, uppercase, titlecase (mixed)
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

    #[test]
    fn test_camel_case_single_word() {
        let camel = gen_camel_case("hello");
        assert_eq!(camel, "hELLO");

        let camel = gen_camel_case("world");
        assert_eq!(camel, "wORLD");
    }

    #[test]
    fn test_snake_case_single_word() {
        let snake = gen_snake_case("hello");
        assert_eq!(snake, "h_e_l_l_o");

        let snake = gen_snake_case("world");
        assert_eq!(snake, "w_o_r_l_d");
    }

    #[test]
    fn test_kebab_case_single_word() {
        let kebab = gen_kebab_case("hello");
        assert_eq!(kebab, "h-e-l-l-o");

        let kebab = gen_kebab_case("world");
        assert_eq!(kebab, "w-o-r-l-d");
    }

    #[test]
    fn test_scream_case_single_word() {
        let scream = gen_scream_case("hello");
        assert_eq!(scream, "H_E_L_L_O");

        let scream = gen_scream_case("world");
        assert_eq!(scream, "W_O_R_L_D");
    }

    #[test]
    fn test_pascal_case_single_word() {
        let pascal = gen_pascal_case("hello");
        assert_eq!(pascal, "Hello");

        let pascal = gen_pascal_case("world");
        assert_eq!(pascal, "World");
    }

    #[test]
    fn test_generate_case_variations_camel_mode() {
        let variations = generate_case_variations("hello", "camel");
        assert_eq!(variations.len(), 1);
        assert!(variations.contains(&"hELLO".to_string()));
    }

    #[test]
    fn test_generate_case_variations_snake_mode() {
        let variations = generate_case_variations("hello", "snake");
        assert_eq!(variations.len(), 1);
        assert!(variations.contains(&"h_e_l_l_o".to_string()));
    }

    #[test]
    fn test_generate_case_variations_kebab_mode() {
        let variations = generate_case_variations("hello", "kebab");
        assert_eq!(variations.len(), 1);
        assert!(variations.contains(&"h-e-l-l-o".to_string()));
    }

    #[test]
    fn test_generate_case_variations_scream_mode() {
        let variations = generate_case_variations("hello", "scream");
        assert_eq!(variations.len(), 1);
        assert!(variations.contains(&"H_E_L_L_O".to_string()));
    }

    #[test]
    fn test_generate_case_variations_pascal_mode() {
        let variations = generate_case_variations("hello", "pascal");
        assert_eq!(variations.len(), 1);
        assert!(variations.contains(&"Hello".to_string()));
    }
}
