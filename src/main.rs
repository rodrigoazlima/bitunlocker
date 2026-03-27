use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Month names in order for range generation
fn get_month_order() -> Vec<&'static str> {
    vec!["january", "february", "march", "april", "may", "june",
         "july", "august", "september", "october", "november", "december"]
}

/// Generate month range from begin to end (inclusive)
fn generate_month_range(begin: &str, end: &str) -> Vec<String> {
    let months = get_month_order();
    let begin_lower = begin.to_lowercase();
    let end_lower = end.to_lowercase();
    
    let start_idx = months.iter().position(|m| m == &begin_lower);
    let end_idx = months.iter().position(|m| m == &end_lower);
    
    if let (Some(s), Some(e)) = (start_idx, end_idx) {
        let mut range = Vec::new();
        for i in s..=e {
            range.push(months[i].to_string());
            // Also add capitalized version
            let cap: String = months[i].chars().next().unwrap().to_uppercase().collect::<String>() + &months[i][1..];
            if cap != months[i] {
                range.push(cap);
            }
        }
        range
    } else {
        // Fallback to all months if parsing fails
        let mut result = Vec::new();
        for m in months {
            result.push(m.to_string());
            let cap: String = m.chars().next().unwrap().to_uppercase().collect::<String>() + &m[1..];
            result.push(cap);
        }
        result
    }
}

/// Generate numbers from 0 to max_value with specified min/max sizes
fn generate_numbers(_min_size: usize, max_size: usize) -> Vec<String> {
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

/// Leet-speak character mapping - maps each character to its possible substitutions
fn get_leet_map() -> HashMap<char, Vec<char>> {
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
        if !map.contains_key(&c) {
            map.insert(c, vec![c, c.to_ascii_uppercase()]);
        }
    }
    
    map
}

/// Apply leet-speak substitutions to generate all variations of a string
fn apply_leet_variations(
    word: &str,
    leet_map: &HashMap<char, Vec<char>>,
) -> HashSet<String> {
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
            generate_variations(chars, leet_map, format!("{}{}", current, sub), index + 1, results);
        }
    }
    
    // Generate variations for the lowercase version
    let lower_word = word.to_lowercase();
    let chars: Vec<char> = lower_word.chars().collect();
    generate_variations(&chars, leet_map, String::new(), 0, &mut results);
    
    results
}

/// Generate case variations based on case parameter
fn generate_case_variations(word: &str, case_mode: &str) -> HashSet<String> {
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

/// Template part with all customizable properties
struct TemplatePart {
    kind: String,
    min_size: usize,
    max_size: usize,
    begin: Option<String>,
    end: Option<String>,
    leet_speak: bool,
    case_mode: String,
}

/// Parse template like {month,maxSize=5,minSize=3,begin=january,end=april,leetSpeak=false,case=all}
fn parse_template(template: &str) -> Vec<TemplatePart> {
    let mut parts = Vec::new();
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if chars[i] == '{' {
            // Find the closing brace
            let mut depth = 1;
            let mut j = i + 1;
            while j < chars.len() && depth > 0 {
                if chars[j] == '{' {
                    depth += 1;
                } else if chars[j] == '}' {
                    depth -= 1;
                }
                j += 1;
            }
            
            if depth == 0 {
                let placeholder = template[i..j].to_string();
                
                // Parse placeholder content
                let part = parse_placeholder(&placeholder);
                if !part.kind.is_empty() {
                    parts.push(part);
                }
            }
            
            i = j;
        } else {
            i += 1;
        }
    }
    
    parts
}

/// Parse a single placeholder like {month,maxSize=5,minSize=3,begin=january,end=april}
fn parse_placeholder(placeholder: &str) -> TemplatePart {
    let mut kind = String::new();
    let mut min_size = 1;
    let mut max_size = 5;
    let mut begin = None;
    let mut end = None;
    let mut leet_speak = false;
    let mut case_mode = "mixed".to_string();
    
    // Remove braces
    let content = placeholder.trim_start_matches('{').trim_end_matches('}');
    
    // Parse each key=value pair
    for part in content.split(',') {
        if let Some((key, value)) = part.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "word" | "month" => kind = key.to_string(),
                "minSize" => min_size = value.parse().unwrap_or(1),
                "maxSize" => max_size = value.parse().unwrap_or(5),
                "begin" => begin = Some(value.to_lowercase()),
                "end" => end = Some(value.to_lowercase()),
                "leetSpeak" => leet_speak = value.to_lowercase() == "true",
                "case" => case_mode = value.to_lowercase(),
                _ => {}
            }
        } else if part.trim().contains("word") || part.trim().contains("month") {
            kind = part.trim().to_string();
        }
    }
    
    TemplatePart {
        kind,
        min_size,
        max_size,
        begin,
        end,
        leet_speak,
        case_mode,
    }
}

/// Generate all password combinations
fn generate_passwords(
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

fn main() {
    let args: Vec<String> = args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <template>", args[0]);
        println!("\nTemplate format examples:");
        println!("{{month,maxSize=5,minSize=3,begin=january,end=april,leetSpeak=false,case=all}}Example{{number,maxSize=3}}");
        println!("{{word,maxSize=4,minSize=2,leetSpeak=true,case=mixed}}{{number,maxSize=2}}");
        std::process::exit(1);
    }
    
    let template = &args[1];
    let parts = parse_template(template);
    
    if parts.is_empty() {
        println!("No valid placeholders found in template");
        std::process::exit(1);
    }
    
    // Extract all parameters
    let mut word_min_size = 1;
    let mut word_max_size = 5;
    let mut begin_month = "january".to_string();
    let mut end_month = "december".to_string();
    let mut leet_speak_enabled = false;
    let mut case_mode = "mixed".to_string();
    let mut number_max_size = 3;
    
    for part in &parts {
        match part.kind.as_str() {
            "word" | "month" => {
                word_min_size = part.min_size;
                word_max_size = part.max_size;
                if let Some(b) = &part.begin {
                    begin_month = b.clone();
                }
                if let Some(e) = &part.end {
                    end_month = e.clone();
                }
                leet_speak_enabled = part.leet_speak;
                case_mode = part.case_mode.clone();
            }
            "number" => number_max_size = part.max_size,
            _ => {}
        }
    }
    
    // Generate month range
    let months = generate_month_range(&begin_month, &end_month);
    
    // Apply leet-speak to months if enabled
    let leet_map = get_leet_map();
    let mut word_variations = HashSet::new();
    
    for month in &months {
        if leet_speak_enabled {
            let variations = apply_leet_variations(month, &leet_map);
            for v in variations {
                if v.len() >= word_min_size && v.len() <= word_max_size {
                    word_variations.insert(v);
                }
            }
        } else {
            // Just add the month with case variations based on mode
            let cases = generate_case_variations(month, &case_mode);
            for v in cases {
                if v.len() >= word_min_size && v.len() <= word_max_size {
                    word_variations.insert(v);
                }
            }
        }
    }
    
    // Generate number values (0-999)
    let number_values = generate_numbers(1, number_max_size);
    
    // Generate passwords
    let passwords = generate_passwords(word_variations.into_iter().collect(), number_values);
    
    // Write to output file
    let output_file = "generated_passwords.txt";
    match File::create(output_file) {
        Ok(f) => {
            let mut writer = BufWriter::new(f);
            for pwd in &passwords {
                writeln!(writer, "{}", pwd).unwrap();
            }
            println!("Generated {} passwords to {}", passwords.len(), output_file);
        }
        Err(e) => {
            eprintln!("Error writing file: {}", e);
            std::process::exit(1);
        }
    }
}