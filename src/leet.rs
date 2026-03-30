use std::collections::{HashMap, HashSet};

/// Leet-speak character mapping - maps each character to its possible substitutions
pub fn get_leet_map() -> HashMap<char, Vec<char>> {
    let mut map = HashMap::new();

    // Common leet-speak substitutions
    map.insert('a', vec!['a', 'A', '@', '4']);
    map.insert('b', vec!['b', 'B', '8']);
    map.insert('c', vec!['c', 'C', '<', '(']);
    map.insert('e', vec!['e', 'E', '3']);
    map.insert('i', vec!['i', 'I', '1', '!']);
    map.insert('l', vec!['l', 'L', '1', '|']);
    map.insert('o', vec!['o', 'O', '0']);
    map.insert('s', vec!['s', 'S', '$', '5']);
    map.insert('t', vec!['t', 'T', '7']);

    // Numbers
    for c in '0'..='9' {
        map.insert(c, vec![c]);
    }

    // Default: keep character as-is (lowercase and uppercase)
    for c in 'd'..='z' {
        map.entry(c)
            .or_insert_with(|| vec![c, c.to_ascii_uppercase()]);
    }

    map
}

/// Apply leet-speak substitutions to generate all variations of a string
pub fn apply_leet_variations(word: &str, leet_map: &HashMap<char, Vec<char>>) -> HashSet<String> {
    let mut results = HashSet::new();

    // Generate leet-speak variations using recursive approach
    fn generate_variations(
        chars: &[char],
        leet_map: &HashMap<char, Vec<char>>,
        current: String,
        index: usize,
        results: &mut HashSet<String>,
    ) {
        if index == chars.len() {
            results.insert(current);
            return;
        }

        let c = chars[index];
        let lower_c = c.to_ascii_lowercase();

        // Get substitutions for this character
        let subs = leet_map.get(&lower_c).cloned().unwrap_or_else(|| vec![c]);

        for sub in subs {
            generate_variations(
                chars,
                leet_map,
                format!("{}{}", current, sub),
                index + 1,
                results,
            );
        }
    }

    // Generate variations for the lowercase version
    let lower_word = word.to_lowercase();
    let chars: Vec<char> = lower_word.chars().collect();
    generate_variations(&chars, leet_map, String::new(), 0, &mut results);

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_leet_map_contains_expected_mappings() {
        let leet_map = get_leet_map();

        // Test 'a' has expected substitutions
        let a_subs = leet_map.get(&'a').expect("expected mapping for 'a' exists");
        assert!(a_subs.contains(&'@'));
        assert!(a_subs.contains(&'4'));

        // Test 'e' has expected substitution
        let e_subs = leet_map.get(&'e').expect("expected mapping for 'e' exists");
        assert!(e_subs.contains(&'3'));

        // Test 's' has expected substitutions
        let s_subs = leet_map.get(&'s').expect("expected mapping for 's' exists");
        assert!(s_subs.contains(&'$'));
        assert!(s_subs.contains(&'5'));
    }

    #[test]
    fn test_apply_leet_variations_empty_string() {
        let leet_map = get_leet_map();
        let result = apply_leet_variations("", &leet_map);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&"".to_string()));
    }

    #[test]
    fn test_apply_leet_variations_simple_word() {
        let leet_map = get_leet_map();
        let result = apply_leet_variations("a", &leet_map);

        // 'a' can be: a, A, @, 4
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"A".to_string()));
        assert!(result.contains(&"@".to_string()));
        assert!(result.contains(&"4".to_string()));
    }

    #[test]
    fn test_apply_leet_variations_with_numbers() {
        let leet_map = get_leet_map();
        let result = apply_leet_variations("test", &leet_map);

        // 't' -> t, T, 7
        // 'e' -> e, E, 3
        // 's' -> s, S, $, 5
        // 't' -> t, T, 7

        // Should contain variations like "t3st", "t35t", etc.
        assert!(result.contains(&"t3st".to_string()));
        assert!(result.contains(&"73s7".to_string()));
    }

    #[test]
    fn test_leet_map_contains_all_letters() {
        let leet_map = get_leet_map();

        // Check that all lowercase letters are in the map
        for c in 'a'..='z' {
            assert!(leet_map.contains_key(&c), "Missing mapping for '{}'", c);
        }
    }

    #[test]
    fn test_apply_leet_variations_preserves_original() {
        let leet_map = get_leet_map();
        let result = apply_leet_variations("abc", &leet_map);

        // Original lowercase should be included
        assert!(result.contains(&"abc".to_string()));
    }
}
