use std::collections::HashSet;

/// Generate shortened versions of a word by removing characters
/// For "august", generates all possible subsequences (removing 0 or more chars)
/// up to the original length
pub fn generate_shortened(word: &str, min_length: usize) -> HashSet<String> {
    let mut results = HashSet::new();
    let chars: Vec<char> = word.chars().collect();
    let n = chars.len();

    if n == 0 {
        return results;
    }

    // Generate all possible subsequences by removing characters
    // Use bitmask approach: for each possible subset of positions to keep
    // Start from empty (mask=0) and go up to full word (mask with all bits set)
    for mask in 0..(1 << n) {
        let mut current = String::new();
        for i in 0..n {
            if mask & (1 << i) != 0 {
                current.push(chars[i]);
            }
        }
        
        // Only include if length is at least min_length and at most original length
        let len = current.len();
        if len >= min_length && len <= n && !current.is_empty() {
            results.insert(current);
        }
    }

    // Also add empty string if min_length allows it
    if min_length == 0 {
        results.insert(String::new());
    }

    results
}

/// Generate extended versions of a word by adding characters (duplicates)
/// For "august":
/// - augst = without the second U (removed index 3)
/// - auust = without G (removed index 2), but with extra U inserted somewhere
/// max_length: maximum total length of the result
pub fn generate_extended(word: &str, max_length: usize) -> HashSet<String> {
    let mut results = HashSet::new();
    
    if word.is_empty() || max_length < 1 {
        return results;
    }

    let chars: Vec<char> = word.chars().collect();
    let n = chars.len();

    // Generate all subsequences (like shortened), but also include
    // extended versions where we duplicate characters
    
    // First, generate all possible subsequences with length > original
    // This means inserting characters between positions
    
    for mask in 0..(1 << n) {
        let mut current = String::new();
        
        for i in 0..n {
            if mask & (1 << i) != 0 {
                current.push(chars[i]);
            }
            // Also try inserting a character after each kept character
            // This is handled separately below
            
        }
        
        let len = current.len();
        if len > n && len <= max_length {
            results.insert(current.clone());
            
            // Also add case variations to results here
            for c in 'a'..='z' {
                current.push(c);
                if current.len() <= max_length {
                    results.insert(current.clone());
                }
                current.pop();
            }
        }
    }

    // Generate versions with character duplication (extended)
    for i in 0..n {
        let mut new_word = String::new();
        for (j, &ch) in chars.iter().enumerate() {
            new_word.push(ch);
            if j == i {
                // Duplicate this character
                new_word.push(ch);
            }
        }
        
        if new_word.len() <= max_length && new_word.len() > n {
            results.insert(new_word);
        }
    }

    // Generate versions with inserted characters at each position
    for pos in 0..=n {
        for c in 'a'..='z' {
            let mut new_word = String::new();
            for (i, &ch) in chars.iter().enumerate() {
                if i == pos {
                    new_word.push(c);
                }
                new_word.push(ch);
            }
            
            if new_word.len() <= max_length && new_word.len() > n {
                results.insert(new_word);
            }
        }
    }

    // Generate versions with 2 inserted characters
    for pos1 in 0..=n {
        for pos2 in pos1 + 1..=n + 1 {
            for c1 in 'a'..='z' {
                for c2 in 'a'..='z' {
                    let mut new_word = String::new();
                    for (i, &ch) in chars.iter().enumerate() {
                        if i == pos1 {
                            new_word.push(c1);
                        }
                        if i + 1 == pos2 {
                            new_word.push(c2);
                        }
                        new_word.push(ch);
                    }
                    
                    // Add final c2 if at end
                    if n == pos2 - 1 {
                        new_word.push(c2);
                    }
                    
                    if new_word.len() <= max_length && new_word.len() > n {
                        results.insert(new_word);
                    }
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_shortened_basic() {
        let result = generate_shortened("abc", 1);
        
        // Should include: a, b, c, ab, ac, bc, abc (7 total)
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"b".to_string()));
        assert!(result.contains(&"c".to_string()));
        assert!(result.contains(&"ab".to_string()));
        assert!(result.contains(&"ac".to_string()));
        assert!(result.contains(&"bc".to_string()));
        assert!(result.contains(&"abc".to_string()));
    }

    #[test]
    fn test_generate_shortened_with_min_length() {
        let result = generate_shortened("abcd", 3);
        
        // With min_length=3: abc, abd, acd, bcd, abcd (5 total)
        assert!(result.contains(&"abc".to_string()));
        assert!(result.contains(&"abd".to_string()));
        assert!(result.contains(&"acd".to_string()));
        assert!(result.contains(&"bcd".to_string()));
        assert!(result.contains(&"abcd".to_string()));
        
        // Should not include 2-char combinations
        assert!(!result.contains(&"ab".to_string()));
    }

    #[test]
    fn test_generate_shortened_august() {
        let result = generate_shortened("august", 3);
        
        // Should include aug, augu, augus, august, etc.
        assert!(result.contains(&"aug".to_string()));
        assert!(result.contains(&"augu".to_string()));
        assert!(result.contains(&"augus".to_string()));
        assert!(result.contains(&"august".to_string()));
    }

    #[test]
    fn test_generate_shortened_empty() {
        let result = generate_shortened("", 1);
        // Empty word produces no results (we skip mask=0 which is empty)
        assert!(result.is_empty());
    }

    #[test]
    fn test_generate_extended_basic() {
        let result = generate_extended("ab", 4);
        
        // Should include words longer than "ab" but <= 4 chars
        for word in &result {
            assert!(word.len() > 2 && word.len() <= 4);
        }
    }

    #[test]
    fn test_generate_extended_within_max() {
        let result = generate_extended("cat", 5);
        
        // All results should be > 3 and <= 5
        for word in &result {
            assert!(word.len() > 3 && word.len() <= 5);
        }
    }

    #[test]
    fn test_generate_extended_max_less_than_word() {
        let result = generate_extended("hello", 3);
        // max_length < word length means no extensions possible
        assert!(result.is_empty());
    }

    #[test]
    fn test_generate_extended_with_original() {
        let result = generate_extended("test", 8);  // Need higher max to get extensions
        
        // Extended should NOT include original (only longer words)
        // But it should include words like "tests", "ttest", etc.
        assert!(!result.contains(&"test".to_string()));
        
        // Should contain at least some extension
        let has_extensions = result.iter().any(|w| w.len() > 4);
        assert!(has_extensions, "Expected at least one extended word");
    }

    #[test]
    fn test_generate_shortened_august_all_variations() {
        // Test that all expected shortened variations of "august" are generated
        let result = generate_shortened("august", 3);
        
        // Check for the examples from user's requirements:
        // augst (removed second u), auust (removed g), augus, augu, aug
        assert!(result.contains(&"augst".to_string()), "augst not found");
        assert!(result.contains(&"auust".to_string()), "auust not found");
        assert!(result.contains(&"augus".to_string()), "augus not found");
        assert!(result.contains(&"augu".to_string()), "augu not found");
        assert!(result.contains(&"aug".to_string()), "aug not found");
        assert!(result.contains(&"auu".to_string()), "auu not found");
        assert!(result.contains(&"ust".to_string()), "ust not found");
    }

    #[test]
    fn test_generate_shortened_with_february() {
        let result = generate_shortened("february", 4);
        
        // Should include shorter versions
        assert!(result.iter().any(|v| v.len() >= 4 && v.len() <= 8));
    }

    #[test]
    fn test_generate_extended_with_august_max_8() {
        let result = generate_extended("august", 8);
        
        // Should produce words with 7-8 characters
        assert!(result.iter().any(|v| v.len() == 7 || v.len() == 8));
    }

    #[test]
    fn test_generate_shortened_with_june() {
        let result = generate_shortened("june", 2);
        
        // Should include all 2-4 character subsequences
        assert!(result.contains(&"jun".to_string()));
        assert!(result.contains(&"jue".to_string()));
        assert!(result.contains(&"jne".to_string()));
        assert!(result.contains(&"une".to_string()));
        assert!(result.contains(&"ue".to_string()));
    }
}
