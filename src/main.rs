use std::env::args;
use std::fs::File;
use std::io::{BufWriter, Write};

mod cache;
mod case;
mod leet;
mod months;
mod numbers;
mod template;
mod unlock;
mod words;

use crate::{
    template::parse_template,
    unlock::{brute_force_unlock, UnlockResult},
};

fn print_usage() {
    println!("Usage: bitunlocker <command> [options]");
    println!("\nCommands:");
    println!("  gen <template>        Generate passwords from template and save to generated_passwords.txt");
    println!(
        "                        (use --unlock D: to also attempt unlock, --test to validate)"
    );
    println!("  test <file>           Validate password file against expected patterns");
    println!("  unlock <drive>        Try to unlock using existing generated_passwords.txt");
    println!("                        (or use --passwords <file> for custom list)");
    println!("  help                  Show this help message");
    println!("\nTemplate placeholders:");
    println!("  {{number,min=X,max=Y}} - Number range with optional padding");
    println!("  {{word,min=X,max=Y}}   - Word value or range");
    println!("  {{month}}              - Month names (january-december)");
    println!("  {{shortened,min=L}}    - Shortened versions of a word (min=length to keep)");
    println!("  {{extended,max=L}}     - Extended versions of a word (max=total length limit)");
    println!("\nOptions:");
    println!("  --unlock D:           Attempt to unlock drive after generation");
    println!("  --passwords <file>    Use custom password file for unlock");
    println!("  --no-powershell       Use manage-bde.exe instead of PowerShell");
    println!("  --no-cache            Disable device-specific password cache (enabled by default)");
    println!("\nExamples:");
    println!("  bitunlocker gen \"pass{{number,min=001,max=333}}\" --unlock D:");
    println!("  bitunlocker gen \"{{word}}{{year,min=1990,max=2030}}\"");
    println!("  bitunlocker gen \"{{shortened,min=3,max=6}}\" - shortened versions with min length");
    println!("  bitunlocker gen \"{{extended,max=10}}\" - extended versions with max length");
    println!("  bitunlocker unlock D: --passwords my_passwords.txt");
    println!("  bitunlocker unlock D: --no-cache"); // Disable cache
}

pub fn generate_and_save_passwords(template: &str, output_file: &str) {
    let parts = parse_template(template);

    if parts.is_empty() {
        // No placeholders - just one literal password
        match File::create(output_file) {
            Ok(f) => {
                let mut writer = BufWriter::new(f);
                // Panic message if file write fails (should not happen since we already created the file)
                writeln!(writer, "{}", template).expect("failed to write single password to file");
                println!("Generated 1 password to {}", output_file);
            }
            Err(e) => {
                eprintln!("Error writing file: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Extract literal text segments between placeholders
    let mut literal_segments = Vec::new();
    let mut current_pos = 0;

    for _part in &parts {
        if let Some(start) = template[current_pos..].find('{') {
            // Literal text before this placeholder
            if start > 0 {
                literal_segments.push(template[current_pos..current_pos + start].to_string());
            }
            // Find the end of the placeholder
            if let Some(end_brace) = template[current_pos + start..].find('}') {
                current_pos = current_pos + start + end_brace + 1;
            } else {
                break;
            }
        } else {
            // No more placeholders, add remaining text
            literal_segments.push(template[current_pos..].to_string());
            break;
        }
    }

    // Collect values for each placeholder
    let mut values_by_part = Vec::new();

    for _part in &parts {
        let values = get_values_for_part(_part);
        values_by_part.push(values);
    }

    // Generate passwords using Cartesian product with literals
    let passwords = generate_combinations_with_literals(&literal_segments, &values_by_part);

    // Remove duplicates and sort
    let mut unique_passwords: Vec<String> = passwords.into_iter().collect();
    unique_passwords.sort();
    unique_passwords.dedup();

    // Write to output file
    match File::create(output_file) {
        Ok(f) => {
            let mut writer = BufWriter::new(f);
            for pwd in &unique_passwords {
                // Panic message if file write fails (should not happen since we already created the file)
                writeln!(writer, "{}", pwd).expect("failed to write password to file");
            }
            println!(
                "Generated {} passwords to {}",
                unique_passwords.len(),
                output_file
            );
        }
        Err(e) => {
            eprintln!("Error writing file: {}", e);
            std::process::exit(1);
        }
    }
}

/// Generate passwords with literal prefixes/suffixes
pub fn generate_combinations_with_literals(
    literals: &[String],
    values_by_part: &[Vec<String>],
) -> Vec<String> {
    if values_by_part.is_empty() {
        return vec![literals.join("")];
    }

    let mut results = Vec::new();

    // For each value in the first placeholder
    for value in &values_by_part[0] {
        // Combine literals with this value and recursively process remaining
        let mut prefix = String::new();

        if !literals.is_empty() && !values_by_part.is_empty() {
            // Add literal before first placeholder
            prefix.push_str(&literals[0]);
        }

        prefix.push_str(value);

        if literals.len() > 1 && values_by_part.len() > 1 {
            // Add literal between placeholders
            prefix.push_str(&literals[1..].join(""));
        }

        let remaining_values: Vec<Vec<String>> = values_by_part.iter().skip(1).cloned().collect();

        if remaining_values.is_empty() {
            results.push(prefix);
        } else {
            // Recursively generate the rest
            let suffixes = generate_combinations_with_literals(&[], &remaining_values);
            for suffix in &suffixes {
                results.push(format!("{}{}", prefix, suffix));
            }
        }
    }

    results
}

pub fn get_values_for_part(part: &crate::template::TemplatePart) -> Vec<String> {
    // If min/max are provided and kind is not word/month, treat as number range
    if let (Some(_begin), Some(_end)) = (&part.min_value, &part.max_value) {
        // Safety: We already checked that min_value and max_value are Some in the pattern match
        return crate::numbers::generate_number_range(
            part.min_value
                .as_ref()
                .expect("min_value is Some after pattern match"),
            part.max_value
                .as_ref()
                .expect("max_value is Some after pattern match"),
        );
    }

    match part.kind.as_str() {
        "number" => {
            // Check if shortened is requested - number doesn't support shortened
            let has_shortened = part.has_shortened_flag;
            
            if has_shortened {
                return vec!["Error: 'number' placeholder does not support 'shortened' modifier".to_string()];
            }
            
            // Default: generate 0-99
            crate::numbers::generate_number_range("0", "99")
        }
        "word" => {
            vec!["".to_string()]
        }
        "shortened" => {
            // Get the source word from begin_value or min_value (as alphabetic word)
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
        "extended" => {
            // Get the source word from min_value or default to empty string
            let source_word = part.min_value.as_deref().unwrap_or("");
            
            // Determine maximum length (default to source_word.len() + 2)
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
        "month" => {
            let all_months = crate::months::get_month_order();

            // Get the begin and end indices for filtering
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

            // If begin/end not found or invalid range, return all months with case variations
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

            // Remove duplicates
            let mut unique_results: Vec<String> = results.into_iter().collect();
            unique_results.sort();
            unique_results.dedup();
            
            unique_results
        }
        _ => {
            // Default: empty string for unknown types
            vec!["".to_string()]
        }
    }
}

fn unlock_drive_from_file(
    drive: &str,
    passwords_file: Option<&str>,
    use_ps: bool,
    stop_after_first: bool,
    use_cache: bool,
) -> UnlockResult {
    let passwords = if let Some(file) = passwords_file {
        match std::fs::read_to_string(file) {
            Ok(content) => content.lines().map(|s| s.to_string()).collect(),
            Err(_e) => {
                eprintln!("Error reading password file '{}': {}", file, _e);
                std::process::exit(1);
            }
        }
    } else {
        match std::fs::read_to_string("generated_passwords.txt") {
            Ok(content) => content.lines().map(|s| s.to_string()).collect(),
            Err(_e) => {
                eprintln!("Error: 'generated_passwords.txt' not found. Generate passwords first.");
                eprintln!("  bitunlocker gen \"pass{{number,min=001,max=333}}\" --unlock D:");
                std::process::exit(1);
            }
        }
    };

    match brute_force_unlock(drive, passwords, use_ps, stop_after_first, use_cache) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error during unlock attempt: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "gen" | "generate" => {
            if args.len() < 3 {
                eprintln!("Error: No template provided");
                println!("\nUsage: bitunlocker gen <template>");
                std::process::exit(1);
            }

            let template = &args[2];
            let mut unlock_drive = None;
            let mut use_cache = true; // Cache is enabled by default

            // Parse additional arguments for --unlock and other options
            let mut i = 3;
            while i < args.len() {
                if args[i] == "--unlock" && i + 1 < args.len() {
                    unlock_drive = Some(&args[i + 1]);
                    i += 2;
                } else if args[i] == "--passwords" && i + 1 < args.len() {
                    // Handle --passwords for custom file (not used with gen, but allowed)
                    i += 2;
                } else if args[i] == "--no-powershell" {
                    // Note: This flag is not used in gen mode, only unlock mode
                    i += 1;
                } else if args[i] == "--no-cache" {
                    use_cache = false;
                    i += 1;
                } else {
                    i += 1;
                }
            }

            generate_and_save_passwords(template, "generated_passwords.txt");

            // If --unlock was specified, run unlock after generation (test all passwords)
            if let Some(drive) = unlock_drive {
                unlock_drive_from_file(drive, None, true, false, use_cache); // stop_after_first = false to test all
            }
        }

        "unlock" => {
            if args.len() < 3 {
                eprintln!("Error: No drive specified");
                println!("\nUsage: bitunlocker unlock D:");
                std::process::exit(1);
            }

            let mut drive = &args[2];
            let mut passwords_file = None;
            let mut use_ps = true;
            let mut use_cache = true; // Cache is enabled by default

            let mut i = 3;
            while i < args.len() {
                if args[i] == "--passwords" && i + 1 < args.len() {
                    passwords_file = Some(&args[i + 1]);
                    i += 2;
                } else if args[i] == "--no-powershell" {
                    use_ps = false;
                    i += 1;
                } else if args[i] == "--no-cache" {
                    use_cache = false;
                    i += 1;
                } else if !args[i].starts_with('-') {
                    drive = &args[i];
                    i += 1;
                } else {
                    i += 1;
                }
            }

            unlock_drive_from_file(
                drive,
                passwords_file.map(|x| x.as_str()),
                use_ps,
                true,
                use_cache,
            ); // stop_after_first = true for default unlock
        }

        "help" | "-h" | "--help" => {
            print_usage();
        }

        _ => {
            println!("Unknown command: {}", command);
            print_usage();
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests for main.rs functionality are minimal as core logic is in lib
}
