//! Password Generator Library
//!
//! This library provides password generation functionality based on templates.

pub mod cache;
pub mod case;
pub mod generator;
pub mod leet;
pub mod months;
pub mod numbers;
pub mod template;
pub mod unlock;

// Re-export commonly used functions from the main module
// Removed generate_month_range re-export as it's unused
pub use cache::{get_cache_file_path, DeviceCache};
pub use case::generate_case_variations;
pub use generator::{generate_combinations, generate_passwords_from_parts};
pub use leet::{apply_leet_variations, get_leet_map};
pub use numbers::generate_number_range;
pub use template::{parse_placeholder, parse_template, TemplatePart};
pub use unlock::{
    brute_force_unlock, print_unlock_report, try_unlock_drive, try_unlock_drive_ps, UnlockResult,
};
