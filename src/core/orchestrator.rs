use clap::ArgMatches;
use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs;

use crate::ui::progress::create_spinner;
use crate::scanners::nmap::{run_nmap_scan, convert_nmap_to_html};
use crate::scanners::dns::{perform_whois, perform_nslookup, perform_dig, perform_dnsrecon};
use crate::scanners::web::{perform_gobuster, perform_nikto, perform_whatweb, perform_ssl_check};
use crate::scanners::host::perform_enum4linux;
use crate::scanners::network::perform_traceroute;
use crate::api::perform_shodan_lookup;
use crate::report::create_scan_summary;

// Core function to orchestrate and run all selected scans
pub fn run_scans(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // Create output directory if it doesn't exist
    let output_dir = matches.value_of("output-dir").unwrap_or("./recon_results");
    fs::create_dir_all(output_dir)?;
    
    // Get target IP
    let target_ip = matches.value_of("ip").unwrap();
    
    // Create the multi-progress bar for managing spinners
    let mp = MultiProgress::new();
    
    // Create overall progress spinner
    let overall_spinner = mp.add(ProgressBar::new_spinner());
    overall_spinner.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
    );
    overall_spinner.set_message(format!("Starting reconnaissance on {}...", style(target_ip).cyan()));
    overall_spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    
    // Print target information
    overall_spinner.suspend(|| {
        println!("🎯 Target: {}", style(target_ip).cyan().bold());
    });
    
    // Generate a timestamp for filenames and folder
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let target_dir_name = format!("{}_{}", target_ip.replace(".", "_").replace("/", "_"), timestamp);
    let scan_dir = format!("{}/{}", output_dir, target_dir_name);
    
    // Create specific directory for this scan
    fs::create_dir_all(&scan_dir)?;
    overall_spinner.suspend(|| {
        println!("📁 Scan directory created: {}", style(&scan_dir).green());
    });
    
    // Base filename (without path)
    let base_filename = target_ip.replace(".", "_");
    
    // NMAP scanning
    if matches.is_present("comprehensive") || matches.is_present("quick") || 
       matches.is_present("noisey") || matches.is_present("stealthy") || 
       matches.is_present("firewall-bypass") {
        
        // Run the appropriate NMAP scan
        let nmap_result = run_nmap_scan(&mp, &matches, target_ip, &base_filename, &scan_dir)?;
        
        // Convert NMAP XML to HTML if scan was successful
        if nmap_result.status.success() {
            let html_spinner = mp.add(create_spinner(&format!("Converting NMAP results to HTML...")));
            match convert_nmap_to_html(&base_filename, &scan_dir) {
                Ok(_) => {
                    html_spinner.finish_with_message(format!("{} NMAP results converted to HTML", style("✓").green()));
                },
                Err(e) => {
                    html_spinner.finish_with_message(format!("{} HTML conversion failed: {}", style("⚠").yellow(), e));
                }
            }
        }
    }
    
    // WHOIS lookup
    if matches.is_present("whois") {
        let whois_spinner = mp.add(create_spinner(&format!("Performing WHOIS lookup on {}...", style(target_ip).cyan())));
        match perform_whois(&whois_spinner, target_ip, &base_filename, &scan_dir) {
            Ok(_) => {
                whois_spinner.finish_with_message(format!("{} WHOIS lookup completed", style("✓").green()));
            },
            Err(e) => {
                whois_spinner.finish_with_message(format!("{} WHOIS lookup failed: {}", style("⚠").yellow(), e));
            }
        }
    }
    
    // Shodan lookup
    if matches.is_present("shodan") {
        let shodan_spinner = mp.add(create_spinner(&format!("Querying Shodan for {}...", style(target_ip).cyan())));
        match perform_shodan_lookup(&shodan_spinner, target_ip, &base_filename, &scan_dir) {
            Ok(_) => {
                shodan_spinner.finish_with_message(format!("{} Shodan query completed", style("✓").green()));
            },
            Err(e) => {
                shodan_spinner.finish_with_message(format!("{} Shodan query failed: {}", style("⚠").yellow(), e));
            }
        }
    }
    
    // NSLookup
    if matches.is_present("nslookup") {
        let nslookup_spinner = mp.add(create_spinner(&format!("Performing NSLookup on {}...", style(target_ip).cyan())));
        match perform_nslookup(&nslookup_spinner, target_ip, &base_filename, &scan_dir) {
            Ok(_) => {
                nslookup_spinner.finish_with_message(format!("{} NSLookup completed", style("✓").green()));
            },
            Err(e) => {
                nslookup_spinner.finish_with_message(format!("{} NSLookup failed: {}", style("⚠").yellow(), e));
            }
        }
    }
    
    // DIG lookup
    if matches.is_present("dig") {
        let dig_spinner = mp.add(create_spinner(&format!("Performing DIG lookup on {}...", style(target_ip).cyan())));
        match perform_dig(&dig_spinner, target_ip, &base_filename, &scan_dir) {
            Ok(_) => {
                dig_spinner.finish_with_message(format!("{} DIG lookup completed", style("✓").green()));
            },
            Err(e) => {
                dig_spinner.finish_with_message(format!("{} DIG lookup failed: {}", style("⚠").yellow(), e));
            }
        }
    }
    
    // Gobuster web directory enumeration
    if matches.is_present("gobuster") {
        let wordlist = matches.value_of("gobuster-wordlist").unwrap();
        let gobuster_spinner = mp.add(create_spinner(
            &format!("Running Gobuster on {} with wordlist {}...", 
            style(target_ip).cyan(), 
            style(wordlist).yellow())
        ));
        
        match perform_gobuster(&gobuster_spinner, target_ip, wordlist, &base_filename, &scan_dir) {
            Ok(_) => {
                gobuster_spinner.finish_with_message(format!("{} Gobuster directory enumeration completed", style("✓").green()));
            },
            Err(e) => {
                gobuster_spinner.finish_with_message(format!("{} Gobuster failed: {}", style("⚠").yellow(), e));
            }
        }
    }
    
    // Nikto web server scan
    if matches.is_present("nikto") {
        let nikto_spinner = mp.add(create_spinner(
            &format!("Running Nikto web vulnerability scan on {} (this could take several minutes)...", 
            style(target_ip).cyan())
        ));
        
        match perform_nikto(&nikto_spinner, target_ip, &base_filename, &scan_dir) {
            Ok(_) => {
                nikto_spinner.finish_with_message(format!("{} Nikto web vulnerability scan completed", style("✓").green()));
            },
            Err(e) => {
                nikto_spinner.finish_with_message(format!("{} Nikto scan failed: {}", style("⚠").yellow(), e));
            }
        }
    }
    
    // Enum4linux Windows/Samba enumeration
    if matches.is_present("enum4linux") {
        let enum4linux_spinner = mp.add(create_spinner(
            &format!("Running Enum4linux Windows/Samba enumeration on {}...", 
            style(target_ip).cyan())
        ));
        
        match perform_enum4linux(&enum4linux_spinner, target_ip, &base_filename, &scan_dir) {
            Ok(_) => {
                enum4linux_spinner.finish_with_message(format!("{} Enum4linux completed", style("✓").green()));
            },
            Err(e) => {
                enum4linux_spinner.finish_with_message(format!("{} Enum4linux failed: {}", style("⚠").yellow(), e));
            }
        }
    }
    
    // WhatWeb technology identification
    if matches.is_present("whatweb") {
        let whatweb_spinner = mp.add(create_spinner(
            &format!("Identifying web technologies on {} with WhatWeb...", 
            style(target_ip).cyan())
        ));
        
        match perform_whatweb(&whatweb_spinner, target_ip, &base_filename, &scan_dir) {
            Ok(_) => {
                whatweb_spinner.finish_with_message(format!("{} WhatWeb technology identification completed", style("✓").green()));
            },
            Err(e) => {
                whatweb_spinner.finish_with_message(format!("{} WhatWeb failed: {}", style("⚠").yellow(), e));
            }
        }
    }
    
    // DNSrecon domain enumeration
    if matches.is_present("dnsrecon") {
        let dnsrecon_spinner = mp.add(create_spinner(
            &format!("Running DNSrecon domain enumeration on {}...", 
            style(target_ip).cyan())
        ));
        
        match perform_dnsrecon(&dnsrecon_spinner, target_ip, &base_filename, &scan_dir) {
            Ok(_) => {
                dnsrecon_spinner.finish_with_message(format!("{} DNSrecon domain enumeration completed", style("✓").green()));
            },
            Err(e) => {
                dnsrecon_spinner.finish_with_message(format!("{} DNSrecon failed: {}", style("⚠").yellow(), e));
            }
        }
    }
    
    // Traceroute network path discovery
    if matches.is_present("traceroute") {
        let traceroute_spinner = mp.add(create_spinner(
            &format!("Running Traceroute to {}...", 
            style(target_ip).cyan())
        ));
        
        match perform_traceroute(&traceroute_spinner, target_ip, &base_filename, &scan_dir) {
            Ok(_) => {
                traceroute_spinner.finish_with_message(format!("{} Traceroute completed", style("✓").green()));
            },
            Err(e) => {
                traceroute_spinner.finish_with_message(format!("{} Traceroute failed: {}", style("⚠").yellow(), e));
            }
        }
    }
    
    // SSL/TLS check
    if matches.is_present("ssl-check") {
        let ssl_spinner = mp.add(create_spinner(
            &format!("Checking SSL/TLS configuration on {}...", 
            style(target_ip).cyan())
        ));
        
        match perform_ssl_check(&ssl_spinner, target_ip, &base_filename, &scan_dir) {
            Ok(_) => {
                ssl_spinner.finish_with_message(format!("{} SSL/TLS configuration check completed", style("✓").green()));
            },
            Err(e) => {
                ssl_spinner.finish_with_message(format!("{} SSL/TLS check failed: {}", style("⚠").yellow(), e));
            }
        }
    }
    
    // Complete the overall progress
    overall_spinner.finish_with_message(format!("{} All reconnaissance tasks completed!", style("✓").green().bold()));
    
    // Create scan summary
    create_scan_summary(matches, target_ip, &scan_dir)?;
    
    println!("\n{} All reconnaissance tasks completed!", style("✓").green().bold());
    println!("📁 Results saved in: {}", style(&scan_dir).green());
    println!("📝 Scan summary created: {}/SCAN_SUMMARY.md", style(&scan_dir).green());
    
    Ok(())
}