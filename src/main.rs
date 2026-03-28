use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::{BufWriter, Write};

mod case;
mod leet;
mod months;
mod numbers;
mod passwords;
mod template;

use crate::{
    case::generate_case_variations,
    leet::{apply_leet_variations, get_leet_map},
    months::generate_month_range,
    numbers::generate_numbers,
    passwords::generate_passwords,
    template::parse_template,
};

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