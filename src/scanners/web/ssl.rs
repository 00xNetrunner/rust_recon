use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Perform SSL/TLS configuration check with spinner
pub fn perform_ssl_check(spinner: &ProgressBar, target: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_ssl_check.txt", scan_dir, base_filename);
    
    // Extract host and port if provided
    let (host, port) = if target.contains(":") {
        let parts: Vec<&str> = target.split(":").collect();
        (parts[0], parts[1])
    } else {
        (target, "443")  // Default HTTPS port
    };
    
    let server = format!("{}:{}", host, port);
    let mut file = File::create(output_file)?;
    
    // Check SSL certificate details
    spinner.set_message(format!("Checking SSL certificates on {}...", style(&server).cyan()));
    let cert_output = Command::new("openssl")
        .arg("s_client")
        .arg("-showcerts")
        .arg("-connect").arg(&server)
        .arg("-servername").arg(host)
        .arg("-verify_hostname").arg(host)
        .output()?;
    
    file.write_all(b"=== CERTIFICATE DETAILS ===\n\n")?;
    file.write_all(&cert_output.stdout)?;
    file.write_all(b"\n\n")?;
    
    // Check supported protocols
    spinner.set_message(format!("Checking SSL/TLS protocols on {}...", style(&server).cyan()));
    file.write_all(b"=== SUPPORTED PROTOCOLS ===\n\n")?;
    
    for protocol in &["ssl2", "ssl3", "tls1", "tls1_1", "tls1_2", "tls1_3"] {
        let proto_output = Command::new("openssl")
            .arg("s_client")
            .arg(format!("-{}", protocol))
            .arg("-connect").arg(&server)
            .arg("-servername").arg(host)
            .output()?;
        
        file.write_all(format!("Protocol {}: ", protocol).as_bytes())?;
        if proto_output.status.success() && String::from_utf8_lossy(&proto_output.stdout).contains("BEGIN CERTIFICATE") {
            file.write_all(b"Supported\n")?;
        } else {
            file.write_all(b"Not supported\n")?;
        }
    }
    
    // Check cipher strength
    spinner.set_message(format!("Checking cipher strength on {}...", style(&server).cyan()));
    let ciphers_output = Command::new("nmap")
        .arg("--script").arg("ssl-enum-ciphers")
        .arg("-p").arg(port)
        .arg(host)
        .output()?;
    
    file.write_all(b"\n=== CIPHER STRENGTH (NMAP) ===\n\n")?;
    file.write_all(&ciphers_output.stdout)?;
    
    Ok(())
}