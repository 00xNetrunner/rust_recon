use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Perform DIG lookup with spinner
pub fn perform_dig(spinner: &ProgressBar, target: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_dig.txt", scan_dir, base_filename);
    
    // Update spinner message
    spinner.set_message(format!("Running DIG with all records for {}...", style(target).cyan()));
    
    let output = Command::new("dig")
        .arg(target)
        .arg("+all")  // More comprehensive output
        .output()?;
    
    let mut file = File::create(output_file)?;
    file.write_all(&output.stdout)?;
    
    Ok(())
}