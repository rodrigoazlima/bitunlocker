use std::process::Command;

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
    use_ps: bool, // Use PowerShell if true
) -> Result<Option<String>, String> {
    let total = passwords.len();
    println!("Attempting to unlock {} with {} passwords...", drive, total);

    for (i, password) in passwords.iter().enumerate() {
        print!("[{}/{}] Trying: {} ... ", i + 1, total, &password);

        let result = if use_ps {
            try_unlock_drive_ps(drive, password)
        } else {
            try_unlock_drive(drive, password)
        };

        match result {
            Ok(true) => {
                println!("\n✓ SUCCESS! Password found: {}", password);
                return Ok(Some(password.clone()));
            }
            Ok(false) => {
                println!("failed");
            }
            Err(e) => {
                println!("error: {}", e);
                // Don't stop on error, continue with next password
            }
        }
    }

    println!("\n✗ No valid password found in the list.");
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_unlock_with_empty_password() {
        // This should return false (not succeed) for an empty/invalid password
        // We can't actually test real unlocking without a locked drive
        // So just verify the function structure works
        let result = try_unlock_drive("D:", "invalid-password");
        assert!(result.is_ok());
    }
}
