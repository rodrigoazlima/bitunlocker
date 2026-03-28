//! Password Generator Library
//! 
//! This library provides password generation functionality based on templates.

pub mod case;
pub mod leet;
pub mod months;
pub mod numbers;
pub mod passwords;
pub mod template;

// Re-export commonly used functions from the main module
pub use months::{get_month_order, generate_month_range};
pub use numbers::generate_numbers;
pub use leet::{get_leet_map, apply_leet_variations};
pub use case::generate_case_variations;
pub use template::{TemplatePart, parse_template, parse_placeholder};
pub use passwords::generate_passwords;