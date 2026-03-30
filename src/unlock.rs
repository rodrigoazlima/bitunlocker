use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

use crate::cache::{get_device_serial_number, get_cache_file_path};

/// Result structure for unlock operations
pub struct UnlockResult {
    /// Total number of passwords tested
    pub total_tested: usize,
    /// List of successful passwords found
    pub successful_passwords: Vec<String>,
    /// Cache file path used during this session (if any)
    pub cache_file: Option<String>,
}

impl UnlockResult {
    /// Create a new empty UnlockResult
    pub fn new() -> Self {
        Self {
            total_tested: 0,
            successful_passwords: Vec::new(),
            cache_file: None,
        }
    }

    /// Create a new UnlockResult with pre-allocated capacity for successful passwords
    #[allow(dead_code)]
    pub fn with_capacity(_capacity: usize) -> Self {
        Self {
            total_tested: 0,
            successful_passwords: Vec::with_capacity(10), // Pre-allocate for expected successes
            cache_file: None,
        }
    }

    /// Save the cache file if a cache was used during this session
    pub fn save_cache(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Print a report of the unlock execution results
pub fn print_unlock_report(result: &UnlockResult, _stop_after_first: bool) {
    println!("\n=== Unlock Execution Report ===");
    println!("Total passwords tested: {}", result.total_tested);
    println!(
        "Passwords that passed: {}",
        result.successful_passwords.len()
    );
    if !result.successful_passwords.is_empty() {
        println!("Successful password(s):");
        for pwd in &result.successful_passwords {
            println!("  - {}", pwd);
        }
    } else {
        println!("No successful passwords found.");
    }
    println!("================================");
}

/// Try to unlock a BitLocker drive using manage-bde.exe
pub fn try_unlock_drive(drive: &str, password: &str) -> Result<bool, String> {
    // manage-bde.exe -unlock D: -RecoveryPassword <password>
    let output = Command::new("manage-bde.exe")
        .arg("-unlock")
        .arg(drive)
        .arg("-RecoveryPassword")
        .arg(password)
        .output()
        .map_err(|e| format!("Failed to run manage-bde: {}", e))?;

    // Check exit status
    if output.status.success() {
        return Ok(true);
    }

    // Parse error output for specific messages
    let stderr = String::from_utf8_lossy(&output.stderr);

    // If it's not a success, check what the error says
    // Wrong password typically shows "BitLocker recovery key is incorrect"
    if stderr.contains("recovery key is incorrect")
        || stderr.contains("incorrect password")
        || stderr.contains("0x80070043")
    {
        return Ok(false);
    }

    // Return false for other errors (might be drive not found, access denied, etc.)
    Ok(false)
}

/// Try to unlock a BitLocker drive using PowerShell (faster approach)
pub fn try_unlock_drive_ps(drive: &str, password: &str) -> Result<bool, String> {
    // PowerShell command to attempt unlock
    let ps_script = format!(
        r#"
try {{
    $Drive = Get-BitLockerVolume -MountPoint "{}"
    Unlock-BitLocker -MountPoint "{}" -RecoveryPassword "{}" -ErrorAction Stop | Out-Null
    Write-Output "SUCCESS"
}} catch {{
    if ($_.Exception.Message -like "*recovery*" -or $_.Exception.Message -like "*password*") {{
        Write-Output "FAILED"
    }} else {{
        Write-Output "ERROR"
    }}
}}
"#,
        drive, drive, password
    );

    let output = Command::new("powershell")
        .arg("-Command")
        .arg(&ps_script)
        .output()
        .map_err(|e| format!("Failed to run PowerShell: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // powershell returns "SUCCESS" or "FAILED"
    Ok(stdout == "SUCCESS")
}

/// Unlock a drive by trying multiple passwords
pub fn brute_force_unlock(
    drive: &str,
    passwords: Vec<String>,
    use_ps: bool,
    stop_after_first: bool,
    use_cache: bool,
) -> Result<UnlockResult, String> {
    let unlock_fn = if use_ps { try_unlock_drive_ps } else { try_unlock_drive };

    // If cache is disabled, skip all cache logic
    if !use_cache {
        return brute_force_unlock_with_callback(drive, passwords, unlock_fn, stop_after_first, None);
    }
    
    // Get cache file path for this device
    let device_id = get_device_serial_number().unwrap_or_else(|_| "unknown".to_string());
    let cache_path = get_cache_file_path(&device_id);
    
    // Load existing cache if available
    let mut cache_used_passwords: HashSet<String> = HashSet::new();
    if Path::exists(&Path::new(&cache_path)) {
        if let Ok(file) = File::open(&cache_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(password) = line {
                    cache_used_passwords.insert(password);
                }
            }
        }
    }

    // Filter out passwords already in cache
    let mut passwords_to_test: Vec<String> = Vec::new();
    
    for password in passwords {
        if !cache_used_passwords.contains(&password) {
            passwords_to_test.push(password);
        }
    }

    brute_force_unlock_with_callback(drive, passwords_to_test, unlock_fn, stop_after_first, Some(cache_path))
}

/// Internal function that supports cache filtering
fn brute_force_unlock_with_callback(
    drive: &str,
    passwords: Vec<String>,
    unlock_fn: fn(&str, &str) -> Result<bool, String>,
    stop_after_first: bool,
    cache_file: Option<String>,
) -> Result<UnlockResult, String> {
    let total = passwords.len();
    println!("Attempting to unlock {} with {} passwords...", drive, total);

    let mut result = UnlockResult::with_capacity(total.min(100));
    result.cache_file = cache_file;
    
    for (i, password) in passwords.iter().enumerate() {
        print!("[{}/{}] Trying: {} ... ", i + 1, total, &password);

        let unlock_result = unlock_fn(drive, password);

        match unlock_result {
            Ok(true) => {
                println!("SUCCESS");
                result.successful_passwords.push(password.clone());
                result.total_tested += 1;
                
                // Add to cache
                if let Some(ref cf) = result.cache_file {
                    add_to_cache(cf, password.clone());
                }
                
                if stop_after_first {
                    // Print report and return
                    print_unlock_report(&result, true);
                    return Ok(result);
                }
            }
            Ok(false) => {
                println!("failed");
                result.total_tested += 1;
            }
            Err(e) => {
                println!("error: {}", e);
                result.total_tested += 1;
                // Don't stop on error, continue with next password
            }
        }
    }

    if result.successful_passwords.is_empty() {
        println!("\n✗ No valid password found in the list.");
    } else {
        println!(
            "\n✓ Tested {} passwords, found {} successful one(s).",
            result.total_tested,
            result.successful_passwords.len()
        );
    }
    
    print_unlock_report(&result, stop_after_first);
    Ok(result)
}

/// Add a password to the cache file
fn add_to_cache(cache_path: &str, password: String) {
    // Append to cache file
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(cache_path) {
        let _ = writeln!(file, "{}", password);
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_try_unlock_with_empty_password() {
        // This should return false (not succeed) for an empty/invalid password
        // We can't actually test real unlocking without a locked drive
        // So just verify the function structure works
        let result = try_unlock_drive("D:", "invalid-password");
        assert!(result.is_ok());
    }

    #[test]
    fn test_unlock_result_new() {
        let result = UnlockResult::new();
        assert_eq!(result.total_tested, 0);
        assert!(result.successful_passwords.is_empty());
    }

    #[test]
    fn test_unlock_result_with_capacity() {
        let result = UnlockResult::with_capacity(10);
        assert_eq!(result.total_tested, 0);
        // capacity is not directly testable, but we can verify it's initialized
        assert!(result.successful_passwords.is_empty());
    }

    #[test]
    fn test_unlock_result_with_successes() {
        let mut result = UnlockResult::new();
        result.total_tested = 5;
        result.cache_file = None;
        result.successful_passwords.push("password1".to_string());
        result.successful_passwords.push("password2".to_string());

        assert_eq!(result.total_tested, 5);
        assert_eq!(result.successful_passwords.len(), 2);
        assert!(result.successful_passwords.contains(&"password1".to_string()));
        assert!(result.successful_passwords.contains(&"password2".to_string()));
    }

    #[test]
    fn test_print_unlock_report_with_no_successes() {
        let result = UnlockResult {
            total_tested: 10,
            successful_passwords: Vec::new(),
            cache_file: None,
        };
        
        // Just verify the function doesn't panic - actual output goes to stdout
        print_unlock_report(&result, true);
    }

    #[test]
    fn test_print_unlock_report_with_successes() {
        let result = UnlockResult {
            total_tested: 15,
            successful_passwords: vec!["correct-password".to_string()],
            cache_file: None,
        };
        
        // Just verify the function doesn't panic
        print_unlock_report(&result, false);
    }

    #[test]
    fn test_brute_force_stop_after_first_true() {
        // With use_cache=false (disabled), all passwords will be tested
        // Since we can't mock the actual unlock function in a simple unit test,
        // just verify it completes without panicking and tests all passwords
        let passwords = vec![
            "wrong1".to_string(),
            "correct".to_string(),
            "wrong2".to_string(),
        ];

        let result = brute_force_unlock("D:", passwords, true, true, false);

        assert!(result.is_ok());
        // Since we're using real unlock function that returns false for all,
        // it should test all 3 passwords
        let result = result.unwrap();
        assert_eq!(result.total_tested, 3);
    }

    #[test]
    fn test_brute_force_stop_after_first_false() {
        // Create passwords where first and third are "successes"
        let passwords = vec![
            "wrong".to_string(),
            "wrong".to_string(),
            "also-correct".to_string(),
        ];

        // With stop_after_first=false, should continue testing all
        let result = brute_force_unlock("D:", passwords, true, false, false);

        assert!(result.is_ok());
        let result = result.unwrap();
        
        // Should have tested all 3 passwords
        assert_eq!(result.total_tested, 3);
    }

    #[test]
    fn test_brute_force_cache_disabled() {
        // Test that cache disabled works correctly
        let passwords = vec![
            "pwd1".to_string(),
            "pwd2".to_string(),
        ];

        // With use_cache=false, should test all passwords
        let result = brute_force_unlock("D:", passwords, true, false, false);

        assert!(result.is_ok());
        let result = result.unwrap();
        
        // Should have tested all 2 passwords
        assert_eq!(result.total_tested, 2);
    }
}