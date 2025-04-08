use clap::ArgMatches;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use console::style;

use crate::ui::progress::create_spinner;

// Create scan summary
pub fn create_scan_summary(matches: &ArgMatches, target_ip: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    // Create scan summary
    let summary_spinner = create_spinner(&format!("Creating scan summary..."));
    let summary_file = format!("{}/SCAN_SUMMARY.md", scan_dir);
    let mut summary = File::create(summary_file)?;
    
    // Write summary header
    let summary_header = format!(r#"# Reconnaissance Scan Summary
## Target: {}
## Date: {}
## Tools Used:

"#, target_ip, chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    
    summary.write_all(summary_header.as_bytes())?;
    
    // List all tools used
    if matches.is_present("comprehensive") { summary.write_all(b"- NMAP (Comprehensive Scan)\n")?; }
    if matches.is_present("quick") { summary.write_all(b"- NMAP (Quick Scan)\n")?; }
    if matches.is_present("stealthy") { summary.write_all(b"- NMAP (Stealthy Scan)\n")?; }
    if matches.is_present("noisey") { summary.write_all(b"- NMAP (Noisey Scan)\n")?; }
    if matches.is_present("firewall-bypass") { summary.write_all(b"- NMAP (Firewall Bypass Techniques)\n")?; }
    if matches.is_present("whois") { summary.write_all(b"- WHOIS Lookup\n")?; }
    if matches.is_present("shodan") { summary.write_all(b"- Shodan API Lookup\n")?; }
    if matches.is_present("nslookup") { summary.write_all(b"- NSLookup\n")?; }
    if matches.is_present("dig") { summary.write_all(b"- DIG DNS Lookup\n")?; }
    if matches.is_present("gobuster") { summary.write_all(b"- Gobuster Web Directory Enumeration\n")?; }
    if matches.is_present("nikto") { summary.write_all(b"- Nikto Web Vulnerability Scanner\n")?; }
    if matches.is_present("enum4linux") { summary.write_all(b"- Enum4linux Windows/Samba Enumeration\n")?; }
    if matches.is_present("whatweb") { summary.write_all(b"- WhatWeb Technology Identification\n")?; }
    if matches.is_present("dnsrecon") { summary.write_all(b"- DNSrecon Domain Enumeration\n")?; }
    if matches.is_present("traceroute") { summary.write_all(b"- Traceroute Network Path Discovery\n")?; }
    if matches.is_present("ssl-check") { summary.write_all(b"- SSL/TLS Configuration Check\n")?; }
    
    // List files generated
    summary.write_all(b"\n## Files Generated:\n\n")?;
    
    // Fix: Use a reference to scan_dir to avoid moving its ownership
    let paths = std::fs::read_dir(&scan_dir)?;
    for path in paths {
        let entry = path?;
        if entry.file_name() != "SCAN_SUMMARY.md" {
            let filename = entry.file_name().to_string_lossy().to_string();
            summary.write_all(format!("- {}\n", filename).as_bytes())?;
        }
    }
    
    summary_spinner.finish_with_message(format!("{} Scan summary created", style("✓").green()));
    
    Ok(())
}