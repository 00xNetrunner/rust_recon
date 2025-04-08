use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Perform Nikto web scan with spinner
pub fn perform_nikto(spinner: &ProgressBar, target: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_nikto.txt", scan_dir, base_filename);
    
    // Update spinner with status
    spinner.set_message(format!("Preparing Nikto scan for {}...", style(target).cyan()));
    
    // Prepare target
    let target_url = if target.starts_with("http://") || target.starts_with("https://") {
        target.to_string()
    } else {
        format!("http://{}", target)
    };
    
    spinner.set_message(format!("Running comprehensive Nikto vulnerability scan on {}...", style(&target_url).cyan()));
    
    let output = Command::new("nikto")
        .arg("-h").arg(&target_url)
        .arg("-o").arg(&output_file)
        .output()?;
    
    // If there was an error, save stderr
    if !output.status.success() {
        let error_file = format!("{}/{}_nikto_error.txt", scan_dir, base_filename);
        let mut file = File::create(&error_file)?;
        file.write_all(&output.stderr)?;
        return Err(format!("Nikto errors - see {} for details", error_file).into());
    }
    
    Ok(())
}