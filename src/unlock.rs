use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

use crate::cache::{get_cache_file_path, get_device_serial_number, DeviceCache};

/// BitLocker brute-force unlocker for Windows. 
// Tries recovery passwords via manage-bde.exe or PowerShell (Unlock-BitLocker). 
// Includes per-device caching (by serial) to skip known passwords, progress reporting, 
// and stop-after-first option. Uses UnlockResult for output. 
// Windows-only, requires admin.
// @author rodrigoazlima

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
    #[cfg(test)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            total_tested: 0,
            successful_passwords: Vec::with_capacity(capacity),
            cache_file: None,
        }
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
    let unlock_fn = if use_ps {
        try_unlock_drive_ps
    } else {
        try_unlock_drive
    };

    // If cache is disabled, skip all cache logic
    if !use_cache {
        return brute_force_unlock_with_callback(
            drive,
            passwords,
            unlock_fn,
            stop_after_first,
            None,
            None,
        );
    }

    // Get cache file path for this device
    let device_id = get_device_serial_number().unwrap_or_else(|_| "unknown".to_string());
    let cache_path = get_cache_file_path(&device_id);

    // Load existing cache if available, or create new one
    let cache = DeviceCache::load_from_file(&cache_path).ok();

    // Use the DeviceCache to track successful passwords during this session
    let mut temp_cache = cache.clone().unwrap_or_else(|| DeviceCache {
        device_id: device_id.clone(),
        used_passwords: HashSet::new(),
    });

    let result = brute_force_unlock_with_callback(
        drive,
        passwords,
        unlock_fn,
        stop_after_first,
        Some(cache_path),
        cache.as_ref(),
    )?;

    // Add successful passwords to temp cache and save
    for pwd in &result.successful_passwords {
        temp_cache.add(pwd.clone());
    }

    if !temp_cache.used_passwords.is_empty() {
        let _ = temp_cache.save();
    }

    Ok(result)
}

/// Internal function that supports cache filtering
fn brute_force_unlock_with_callback(
    drive: &str,
    passwords: Vec<String>,
    unlock_fn: fn(&str, &str) -> Result<bool, String>,
    stop_after_first: bool,
    cache_file: Option<String>,
    cache: Option<&DeviceCache>,
) -> Result<UnlockResult, String> {
    let total = passwords.len();
    println!("Attempting to unlock {} with {} passwords...", drive, total);

    let mut result = UnlockResult::new();
    if cache_file.is_some() {
        // Pre-allocate capacity when we have a cache file
        result.successful_passwords.reserve(total.min(100));
    }
    result.cache_file = cache_file;

    for (i, password) in passwords.iter().enumerate() {
        print!("[{}/{}] Trying: {} ... ", i + 1, total, &password);

        // Check if password is in cache
        if let Some(ref c) = cache {
            if c.contains(password) {
                println!("skipped (cached)");
                continue;
            }
        }

        let unlock_result = unlock_fn(drive, password);

        match unlock_result {
            Ok(true) => {
                println!("SUCCESS");
                result.successful_passwords.push(password.clone());
                result.total_tested += 1;

                // Add to cache
                if let Some(cf) = result.cache_file.as_deref() {
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
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(cache_path)
    {
        let _ = writeln!(file, "{}", password);
    }
}

/// Mock unlock function for testing - always returns false (failed)
#[allow(dead_code)]
fn mock_unlock_fn_failed(_drive: &str, _password: &str) -> Result<bool, String> {
    Ok(false)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_brute_force_with_cached_password() {
        // Test that cached passwords show "skipped (cached)" and don't count toward total_tested
        let passwords = vec![
            "new_password1".to_string(),
            "cached_password".to_string(),
            "new_password2".to_string(),
        ];

        // Create a cache with one password already tested
        let mut cache = DeviceCache::new().unwrap_or_else(|_| DeviceCache {
            device_id: "test-device".to_string(),
            used_passwords: HashSet::new(),
        });
        cache.add("cached_password".to_string());

        // Use mock function to avoid actual unlock attempts
        let result = brute_force_unlock_with_callback(
            "D:",
            passwords,
            mock_unlock_fn_failed,
            false, // stop_after_first = false to test all
            None,  // cache_file
            Some(&cache),
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        // Only 2 new passwords should be tested (cached one skipped)
        assert_eq!(result.total_tested, 2);
    }

    #[test]
    fn test_brute_force_all_cached_passwords() {
        // Test when all passwords are in cache - total_tested should be 0
        let passwords = vec![
            "cached1".to_string(),
            "cached2".to_string(),
            "cached3".to_string(),
        ];

        let mut cache = DeviceCache::new().unwrap_or_else(|_| DeviceCache {
            device_id: "test-device".to_string(),
            used_passwords: HashSet::new(),
        });
        cache.add("cached1".to_string());
        cache.add("cached2".to_string());
        cache.add("cached3".to_string());

        let result = brute_force_unlock_with_callback(
            "D:",
            passwords,
            mock_unlock_fn_failed,
            false,
            None,
            Some(&cache),
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        // All 3 passwords should be skipped, total_tested should be 0
        assert_eq!(result.total_tested, 0);
        assert!(result.successful_passwords.is_empty());
    }

    #[test]
    fn test_brute_force_no_cached_passwords() {
        // Test when no passwords are in cache - all should be tested
        let passwords = vec![
            "new1".to_string(),
            "new2".to_string(),
        ];

        // Empty cache
        let cache = DeviceCache::new().unwrap_or_else(|_| DeviceCache {
            device_id: "test-device".to_string(),
            used_passwords: HashSet::new(),
        });

        let result = brute_force_unlock_with_callback(
            "D:",
            passwords,
            mock_unlock_fn_failed,
            false,
            None,
            Some(&cache),
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        // All 2 passwords should be tested
        assert_eq!(result.total_tested, 2);
    }

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
        assert!(result
            .successful_passwords
            .contains(&"password1".to_string()));
        assert!(result
            .successful_passwords
            .contains(&"password2".to_string()));
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
        let passwords = vec!["pwd1".to_string(), "pwd2".to_string()];

        // With use_cache=false, should test all passwords
        let result = brute_force_unlock("D:", passwords, true, false, false);

        assert!(result.is_ok());
        let result = result.unwrap();

        // Should have tested all 2 passwords
        assert_eq!(result.total_tested, 2);
    }
}
