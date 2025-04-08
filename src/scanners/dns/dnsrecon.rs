use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Perform DNSrecon domain enumeration with spinner
pub fn perform_dnsrecon(spinner: &ProgressBar, target: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_dnsrecon.txt", scan_dir, base_filename);
    
    // Update spinner message
    spinner.set_message(format!("Starting DNS reconnaissance on {}...", style(target).cyan()));
    
    let output = Command::new("dnsrecon")
        .arg("-d").arg(target)
        .arg("-t").arg("std,srv,axfr,rvl")  // Standard tests
        .arg("--csv").arg(&output_file)
        .output()?;
    
    // Save text output too
    spinner.set_message(format!("Processing DNSrecon results for {}...", style(target).cyan()));
    let text_output = format!("{}/{}_dnsrecon_text.txt", scan_dir, base_filename);
    let mut file = File::create(text_output)?;
    file.write_all(&output.stdout)?;
    
    // If there was an error, save stderr
    if !output.status.success() {
        let error_file = format!("{}/{}_dnsrecon_error.txt", scan_dir, base_filename);
        let mut file = File::create(&error_file)?;
        file.write_all(&output.stderr)?;
        return Err(format!("DNSrecon errors - see {} for details", error_file).into());
    }
    
    Ok(())
}