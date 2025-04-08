use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Perform Gobuster web directory enumeration with spinner
pub fn perform_gobuster(spinner: &ProgressBar, target: &str, wordlist: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_gobuster.txt", scan_dir, base_filename);
    
    // Update spinner with status
    spinner.set_message(format!("Setting up Gobuster for {}...", style(target).cyan()));
    
    // Check if target is likely a web server (add http:// if not present)
    let target_url = if target.starts_with("http://") || target.starts_with("https://") {
        target.to_string()
    } else {
        format!("http://{}", target)
    };
    
    spinner.set_message(format!("Scanning {} with wordlist {} (this may take a while)...", 
                               style(&target_url).cyan(), 
                               style(wordlist).yellow()));
    
    let output = Command::new("gobuster")
        .arg("dir")
        .arg("-u").arg(&target_url)
        .arg("-w").arg(wordlist)
        .arg("-o").arg(&output_file)
        .arg("-q")  // Quiet mode
        .output()?;
    
    // If there was an error, save stderr
    if !output.status.success() {
        let error_file = format!("{}/{}_gobuster_error.txt", scan_dir, base_filename);
        let mut file = File::create(&error_file)?;
        file.write_all(&output.stderr)?;
        return Err(format!("Gobuster errors - see {} for details", error_file).into());
    }
    
    Ok(())
}