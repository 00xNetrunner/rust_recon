use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Perform Traceroute network path discovery with spinner
pub fn perform_traceroute(spinner: &ProgressBar, target: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_traceroute.txt", scan_dir, base_filename);
    
    // Update spinner message
    spinner.set_message(format!("Tracing network path to {}...", style(target).cyan()));
    
    let output = Command::new("traceroute")
        .arg(target)
        .output()?;
    
    spinner.set_message(format!("Saving traceroute results for {}...", style(target).cyan()));
    
    let mut file = File::create(output_file)?;
    file.write_all(&output.stdout)?;
    
    Ok(())
}