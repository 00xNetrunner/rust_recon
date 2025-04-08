use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Perform WHOIS lookup with spinner
pub fn perform_whois(spinner: &ProgressBar, target: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_whois.txt", scan_dir, base_filename);
    
    // Update spinner with current status
    spinner.set_message(format!("Running WHOIS lookup on {}...", style(target).cyan()));
    
    // Execute command
    let output = Command::new("whois")
        .arg(target)
        .output()?;
    
    let mut file = File::create(output_file)?;
    file.write_all(&output.stdout)?;
    
    Ok(())
}