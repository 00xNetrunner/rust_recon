use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Perform WhatWeb technology identification with spinner
pub fn perform_whatweb(spinner: &ProgressBar, target: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_whatweb.txt", scan_dir, base_filename);
    
    // Update spinner message
    spinner.set_message(format!("Setting up WhatWeb for {}...", style(target).cyan()));
    
    // Prepare target
    let target_url = if target.starts_with("http://") || target.starts_with("https://") {
        target.to_string()
    } else {
        format!("http://{}", target)
    };
    
    spinner.set_message(format!("Identifying web technologies on {}...", style(&target_url).cyan()));
    
    let output = Command::new("whatweb")
        .arg("-v")  // Verbose
        .arg("-a3")  // Aggression level
        .arg("--log-verbose").arg(&output_file)
        .arg(&target_url)
        .output()?;
    
    // If there was an error, save stderr
    if !output.status.success() {
        let error_file = format!("{}/{}_whatweb_error.txt", scan_dir, base_filename);
        let mut file = File::create(&error_file)?;
        file.write_all(&output.stderr)?;
        return Err(format!("WhatWeb errors - see {} for details", error_file).into());
    }
    
    Ok(())
}