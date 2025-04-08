use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::env;
use std::error::Error;

// Create directory if it doesn't exist
// This function is kept as a utility for future use
#[allow(dead_code)]
pub fn ensure_dir_exists(path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(path).exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

// Resource management for system-wide installation support
pub struct ResourceManager;

impl ResourceManager {
    // Get a resource file (checks multiple locations to support both development and installed usage)
    pub fn get_resource_path(resource_name: &str) -> Option<PathBuf> {
        // Check for environment variable pointing to resources
        if let Ok(base_dir) = env::var("RUST_RECON_RESOURCES") {
            let path = Path::new(&base_dir).join(resource_name);
            if path.exists() {
                return Some(path);
            }
        }
        
        // Possible locations to check in order of preference
        let locations = vec![
            // 1. Current executable directory
            env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf())),
            
            // 2. Current executable's parent directory (for /usr/bin -> /usr/share pattern)
            env::current_exe().ok().and_then(|p| p.parent().and_then(|p| p.parent().map(|p| p.to_path_buf()))),
            
            // 3. Current working directory
            env::current_dir().ok(),
            
            // 4. System-wide installation directory
            Some(PathBuf::from("/usr/share/rust_recon")),
            
            // 5. User's local share directory
            None, // Would use dirs::data_local_dir().map(|p| p.join("rust_recon")) with dirs crate
            
            // 6. The repository location (fallback for development)
            Some(PathBuf::from("/home/kali/rust_recon")),
        ];
        
        // Check each location combined with different possible subdirectories
        for base_dir in locations.into_iter().flatten() {
            // Check directly in the base directory
            let direct_path = base_dir.join(resource_name);
            if direct_path.exists() {
                return Some(direct_path);
            }
            
            // Check in src/utils/ subdirectory (for dev environment)
            let src_utils_path = base_dir.join("src").join("utils").join(resource_name);
            if src_utils_path.exists() {
                return Some(src_utils_path);
            }
            
            // Check in share/ subdirectory (for installed environment)
            let share_path = base_dir.join("share").join(resource_name);
            if share_path.exists() {
                return Some(share_path);
            }
        }
        
        None
    }
    
    // Create a temporary file with embedded content
    pub fn create_temp_resource(resource_name: &str, content: &str) -> Result<PathBuf, Box<dyn Error>> {
        let temp_dir = env::temp_dir();
        fs::create_dir_all(&temp_dir)?;
        
        let temp_file_path = temp_dir.join(resource_name);
        let mut file = File::create(&temp_file_path)?;
        file.write_all(content.as_bytes())?;
        
        Ok(temp_file_path)
    }
}