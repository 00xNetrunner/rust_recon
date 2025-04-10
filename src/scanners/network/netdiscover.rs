use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Perform network discovery using netdiscover with spinner
pub fn perform_netdiscover(spinner: &ProgressBar, target_network: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_netdiscover.txt", scan_dir, base_filename);
    
    // Update spinner message
    spinner.set_message(format!("Discovering active hosts on network {} using netdiscover...", style(target_network).cyan()));
    
    // Run netdiscover with -r flag for specific range, limiting packets to 3 per host and passive mode for faster results
    let output = Command::new("netdiscover")
        .args(["-r", target_network, "-P", "-c", "3"])
        .output()?;
    
    spinner.set_message(format!("Saving netdiscover results for {}...", style(target_network).cyan()));
    
    let mut file = File::create(output_file)?;
    file.write_all(&output.stdout)?;
    
    Ok(())
}