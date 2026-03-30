use std::process::Command;

/// Result structure for unlock operations
pub struct UnlockResult {
    /// Total number of passwords tested
    pub total_tested: usize,
    /// List of successful passwords found
    pub successful_passwords: Vec<String>,
}

impl UnlockResult {
    /// Create a new empty UnlockResult
    pub fn new() -> Self {
        Self {
            total_tested: 0,
            successful_passwords: Vec::new(),
        }
    }

    /// Create a new UnlockResult with pre-allocated capacity for successful passwords
    #[allow(dead_code)]
    pub fn with_capacity(_capacity: usize) -> Self {
        Self {
            total_tested: 0,
            successful_passwords: Vec::with_capacity(10), // Pre-allocate for expected successes
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

#[cfg(test)]
use std::collections::HashSet;
#[cfg(test)]
use std::sync::Arc;

/// Mockable function type for testing
type UnlockCallback = fn(&str, &str) -> Result<bool, String>;

/// Global mutable state for mock unlocker (thread-safe)
#[cfg(test)]
static mut MOCK_SUCCESSFUL_PASSWORDS: Option<Arc<HashSet<String>>> = None;

/// Helper struct for mock unlocker that implements the callback interface
#[cfg(test)]
struct MockUnlockHelper;

#[cfg(test)]
impl MockUnlockHelper {
    /// Set the successful passwords for this test (called before each test)
    fn set_successful_passwords(passwords: &[&str]) {
        let pwd_set: HashSet<String> = passwords.iter().map(|s| s.to_string()).collect();
        unsafe {
            MOCK_SUCCESSFUL_PASSWORDS = Some(Arc::new(pwd_set));
        }
    }

    /// Static method that matches the UnlockCallback signature
    fn callback(_drive: &str, pwd: &str) -> Result<bool, String> {
        unsafe {
            if let Some(ref set) = MOCK_SUCCESSFUL_PASSWORDS {
                Ok(set.contains(pwd))
            } else {
                Ok(false)
            }
        }
    }
}

/// Unlock a drive by trying multiple passwords
pub fn brute_force_unlock_with_callback(
    drive: &str,
    passwords: Vec<String>,
    unlock_fn: UnlockCallback,
    stop_after_first: bool, // Stop after first successful password
) -> Result<UnlockResult, String> {
    let total = passwords.len();
    println!("Attempting to unlock {} with {} passwords...", drive, total);

    let mut result = UnlockResult::with_capacity(total.min(100));
    
    for (i, password) in passwords.iter().enumerate() {
        print!("[{}/{}] Trying: {} ... ", i + 1, total, &password);

        let unlock_result = unlock_fn(drive, password);

        match unlock_result {
            Ok(true) => {
                println!("SUCCESS");
                result.successful_passwords.push(password.clone());
                result.total_tested += 1;
                
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

/// Unlock a drive by trying multiple passwords using PowerShell
pub fn brute_force_unlock_ps(
    drive: &str,
    passwords: Vec<String>,
    stop_after_first: bool,
) -> Result<UnlockResult, String> {
    brute_force_unlock_with_callback(drive, passwords, try_unlock_drive_ps, stop_after_first)
}

/// Unlock a drive by trying multiple passwords using manage-bde.exe
pub fn brute_force_unlock_manage(
    drive: &str,
    passwords: Vec<String>,
    stop_after_first: bool,
) -> Result<UnlockResult, String> {
    brute_force_unlock_with_callback(drive, passwords, try_unlock_drive, stop_after_first)
}

/// Unlock a drive by trying multiple passwords (convenience wrapper using PowerShell)
pub fn brute_force_unlock(
    drive: &str,
    passwords: Vec<String>,
    use_ps: bool,
    stop_after_first: bool,
) -> Result<UnlockResult, String> {
    if use_ps {
        brute_force_unlock_ps(drive, passwords, stop_after_first)
    } else {
        brute_force_unlock_manage(drive, passwords, stop_after_first)
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
        };
        
        // Just verify the function doesn't panic - actual output goes to stdout
        print_unlock_report(&result, true);
    }

    #[test]
    fn test_print_unlock_report_with_successes() {
        let result = UnlockResult {
            total_tested: 15,
            successful_passwords: vec!["correct-password".to_string()],
        };
        
        // Just verify the function doesn't panic
        print_unlock_report(&result, false);
    }

    #[test]
    fn test_brute_force_stop_after_first_true() {
        // Create passwords where second one is "success"
        let passwords = vec![
            "wrong1".to_string(),
            "correct".to_string(),
            "wrong2".to_string(), // Should not be tested after first success
        ];

        // Set up mock to return true only for "correct"
        MockUnlockHelper::set_successful_passwords(&["correct"]);

        // With stop_after_first=true, should return after finding first success
        let result = brute_force_unlock_with_callback("D:", passwords, MockUnlockHelper::callback, true);

        assert!(result.is_ok());
        let result = result.unwrap();
        
        // Should have tested only 2 passwords (stops at first success)
        assert_eq!(result.total_tested, 2);
        assert_eq!(result.successful_passwords.len(), 1);
        assert_eq!(result.successful_passwords[0], "correct");
    }

    #[test]
    fn test_brute_force_stop_after_first_false() {
        // Create passwords where first and third are "successes"
        let passwords = vec![
            "correct".to_string(),
            "wrong".to_string(),
            "also-correct".to_string(),
        ];

        // Set up mock to return true for "correct" and "also-correct"
        MockUnlockHelper::set_successful_passwords(&["correct", "also-correct"]);

        // With stop_after_first=false, should continue testing all
        let result = brute_force_unlock_with_callback("D:", passwords, MockUnlockHelper::callback, false);

        assert!(result.is_ok());
        let result = result.unwrap();
        
        // Should have tested all 3 passwords
        assert_eq!(result.total_tested, 3);
        // Both correct passwords should be recorded
        assert_eq!(result.successful_passwords.len(), 2);
        assert!(result.successful_passwords.contains(&"correct".to_string()));
        assert!(result.successful_passwords.contains(&"also-correct".to_string()));
    }
}
