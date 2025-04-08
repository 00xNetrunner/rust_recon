use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Perform NSLookup with spinner
pub fn perform_nslookup(spinner: &ProgressBar, target: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_nslookup.txt", scan_dir, base_filename);
    
    // Update spinner message
    spinner.set_message(format!("Resolving {} using nslookup...", style(target).cyan()));
    
    let output = Command::new("nslookup")
        .arg(target)
        .output()?;
    
    let mut file = File::create(output_file)?;
    file.write_all(&output.stdout)?;
    
    Ok(())
}