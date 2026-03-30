use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Cache entry for a device
#[derive(Clone)]
pub struct DeviceCache {
    pub device_id: String,
    pub used_passwords: HashSet<String>,
}

impl DeviceCache {
    /// Create a new empty cache for the current device
    pub fn new() -> Result<Self, String> {
        let device_id = get_device_serial_number()?;
        Ok(DeviceCache {
            device_id,
            used_passwords: HashSet::new(),
        })
    }

    /// Load cache from file if it exists
    pub fn load(device_id: &str) -> Result<Self, String> {
        let cache_path = get_cache_file_path(device_id);

        let mut used_passwords = HashSet::new();

        if Path::exists(&Path::new(&cache_path)) {
            let file = File::open(&cache_path).map_err(|e| format!("Failed to open cache: {}", e))?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                if let Ok(password) = line {
                    used_passwords.insert(password);
                }
            }
        }

        Ok(DeviceCache {
            device_id: device_id.to_string(),
            used_passwords,
        })
    }

    /// Load cache from a specific file path
    pub fn load_from_file(cache_path: &str) -> Result<Self, String> {
        let mut used_passwords = HashSet::new();

        if Path::exists(&Path::new(cache_path)) {
            let file = File::open(cache_path).map_err(|e| format!("Failed to open cache: {}", e))?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                if let Ok(password) = line {
                    used_passwords.insert(password);
                }
            }
        }

        Ok(DeviceCache {
            device_id: "unknown".to_string(),
            used_passwords,
        })
    }

    /// Save cache to file
    pub fn save(&self) -> Result<(), String> {
        let cache_path = get_cache_file_path(&self.device_id);

        // Create directory if it doesn't exist
        if let Some(parent_dir) = Path::new(&cache_path).parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)
                    .map_err(|e| format!("Failed to create cache directory: {}", e))?;
            }
        }

        let mut file = File::create(&cache_path)
            .map_err(|e| format!("Failed to create cache file: {}", e))?;

        // Write device_id first
        writeln!(file, "# Device ID: {}", self.device_id)
            .map_err(|e| format!("Failed to write cache header: {}", e))?;

        // Write used passwords (sorted for consistency)
        let mut passwords: Vec<&String> = self.used_passwords.iter().collect();
        passwords.sort();

        for password in passwords {
            writeln!(file, "{}", password)
                .map_err(|e| format!("Failed to write password to cache: {}", e))?;
        }

        Ok(())
    }

    /// Check if a password is in the cache
    pub fn contains(&self, password: &str) -> bool {
        self.used_passwords.contains(password)
    }

    /// Add a password to the cache
    pub fn add(&mut self, password: String) {
        self.used_passwords.insert(password);
    }

    /// Add multiple passwords to the cache
    pub fn add_many(&mut self, passwords: &[String]) {
        for pwd in passwords {
            self.used_passwords.insert(pwd.clone());
        }
    }
}

/// Get device serial number using PowerShell (Windows)
pub fn get_device_serial_number() -> Result<String, String> {
    let ps_script = r#"
try {
    $serial = (Get-WmiObject -Class Win32_BIOS).SerialNumber
    if ($serial -and $serial -ne "") {
        Write-Output $serial
    } else {
        # Fallback: Get another hardware identifier
        $uuid = (Get-WmiObject -Class Win32_ComputerSystemProduct).UUID
        Write-Output $uuid
    }
} catch {
    Write-Output "unknown"
}
"#;

    let output = std::process::Command::new("powershell")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(ps_script)
        .output()
        .map_err(|e| format!("Failed to run PowerShell: {}", e))?;

    let serial = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Clean up the serial number (remove invalid characters for filenames)
    let cleaned: String = serial.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect();

    if cleaned.is_empty() {
        Err("Failed to retrieve device serial number".to_string())
    } else {
        Ok(cleaned)
    }
}

/// Get the cache file path for a given device ID
pub fn get_cache_file_path(device_id: &str) -> String {
    // Use current working directory with hidden cache prefix
    format!(".bitunlocker-cache-{}.json", device_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_device_serial_number() {
        // This test verifies the PowerShell command works
        let result = get_device_serial_number();
        assert!(result.is_ok());
        println!("Device ID: {}", result.unwrap());
    }

    #[test]
    fn test_cache_new() {
        // Test that we can create a new cache
        let result = DeviceCache::new();
        // This may fail on non-Windows systems or if PowerShell is not available
        match result {
            Ok(cache) => {
                assert!(!cache.device_id.is_empty());
                assert!(cache.used_passwords.is_empty());
            }
            Err(_) => {
                // Cache creation failing is acceptable in test environments
            }
        }
    }

    #[test]
    fn test_cache_contains_and_add() {
        let mut cache = DeviceCache::new().unwrap_or_else(|_| DeviceCache {
            device_id: "test-device".to_string(),
            used_passwords: HashSet::new(),
        });

        // Test contains on empty cache
        assert!(!cache.contains("test-password"));

        // Add a password
        cache.add("test-password".to_string());

        // Test contains after adding
        assert!(cache.contains("test-password"));
        assert!(!cache.contains("other-password"));
    }

    #[test]
    fn test_cache_add_many() {
        let mut cache = DeviceCache::new().unwrap_or_else(|_| DeviceCache {
            device_id: "test-device".to_string(),
            used_passwords: HashSet::new(),
        });

        let passwords = vec!["pwd1", "pwd2", "pwd3"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        cache.add_many(&passwords);

        assert!(cache.contains("pwd1"));
        assert!(cache.contains("pwd2"));
        assert!(cache.contains("pwd3"));
    }
}