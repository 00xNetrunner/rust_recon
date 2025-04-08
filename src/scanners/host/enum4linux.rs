use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Perform Enum4linux Windows/Samba enumeration with spinner
pub fn perform_enum4linux(spinner: &ProgressBar, target: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_enum4linux.txt", scan_dir, base_filename);
    
    // Update spinner message
    spinner.set_message(format!("Enumerating Windows/Samba services on {}...", style(target).cyan()));
    
    let output = Command::new("enum4linux")
        .arg("-a")  // All simple enumeration
        .arg(target)
        .output()?;
    
    spinner.set_message(format!("Processing enum4linux results for {}...", style(target).cyan()));
    
    let mut file = File::create(output_file)?;
    file.write_all(&output.stdout)?;
    
    Ok(())
}