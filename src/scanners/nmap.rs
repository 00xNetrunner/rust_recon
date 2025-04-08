use clap::ArgMatches;
use console::style;
use indicatif::MultiProgress;
use std::path::Path;
use std::process::{Command, Output};
use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::ui::progress::create_spinner;

// Run NMAP scan with specified options and animated progress
pub fn run_nmap_scan(mp: &MultiProgress, matches: &ArgMatches, target: &str, base_filename: &str, scan_dir: &str) -> Result<Output, Box<dyn Error>> {
    // Add arguments
    let mut nmap_args = vec![];
    
    // Comprehensive scan
    if matches.is_present("comprehensive") {
        nmap_args.extend(vec!["-A", "-p-", "-sT", "-sV", "-O", "--osscan-guess"]);
    }
    
    // Quick scan with custom options
    if matches.is_present("quick") {
        // Check if custom options are provided, otherwise use defaults
        if let Some(quick_opts) = matches.value_of("quick-options") {
            // Split the custom options by spaces and add them
            nmap_args.extend(quick_opts.split_whitespace().collect::<Vec<&str>>());
        } else {
            // Default quick scan options
            nmap_args.extend(vec!["-F", "-T4"]);
        }
    }
    
    // Noisey scan
    if matches.is_present("noisey") {
        nmap_args.extend(vec!["-T5", "-A", "--traceroute"]);
    }
    
    // Stealthy scan
    if matches.is_present("stealthy") {
        nmap_args.extend(vec!["-sS", "-T2", "--data-length", "15", "--mtu", "16"]);
    }
    
    // Firewall bypass techniques
    if matches.is_present("firewall-bypass") {
        nmap_args.extend(vec!["-f", "--mtu", "16", "--spoof-mac", "0", "-D", "RND:5"]);
    }
    
    // Proxy settings
    if let Some(proxy) = matches.value_of("proxy") {
        nmap_args.extend(vec!["--proxies", proxy]);
    }
    
    // Add scripts if specified
    if let Some(scripts) = matches.value_of("scripts") {
        nmap_args.extend(vec!["--script", scripts]);
    }
    
    // Output format
    let xml_output = format!("{}/{}_nmap.xml", scan_dir, base_filename);
    nmap_args.extend(vec!["-oX", &xml_output]);
    
    // Add target
    nmap_args.push(target);
    
    // Create scan type description for the message
    let scan_type = if matches.is_present("comprehensive") {
        "comprehensive"
    } else if matches.is_present("quick") {
        "quick"
    } else if matches.is_present("stealthy") {
        "stealthy"
    } else if matches.is_present("noisey") {
        "noisey"
    } else {
        "custom"
    };
    
    // Create spinner for NMAP scan
    let spinner = mp.add(create_spinner(&format!("Running {} NMAP scan on {} (this may take a while)...", 
                                                 style(scan_type).yellow(), 
                                                 style(target).cyan())));
    
    // Print the command being executed
    spinner.suspend(|| {
        println!("🚀 Executing: nmap {}", nmap_args.join(" "));
    });
    
    // Execute NMAP command
    let output = Command::new("nmap")
        .args(&nmap_args)
        .output()?;
    
    // Save the standard output as well
    let stdout_output = format!("{}/{}_nmap_stdout.txt", scan_dir, base_filename);
    let mut stdout_file = File::create(stdout_output)?;
    stdout_file.write_all(&output.stdout)?;
    
    // Check if scan was successful and finish the spinner
    if output.status.success() {
        spinner.finish_with_message(format!("{} NMAP {} scan completed successfully", style("✓").green(), scan_type));
    } else {
        spinner.finish_with_message(format!("{} NMAP scan encountered issues", style("⚠").yellow()));
    }
    
    Ok(output)
}

// Convert NMAP XML output to HTML using xsltproc with Rose Pine theme
pub fn convert_nmap_to_html(base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let xml_file = format!("{}/{}_nmap.xml", scan_dir, base_filename);
    let html_file = format!("{}/{}_nmap.html", scan_dir, base_filename);
    
    // Check if xsltproc is available
    match Command::new("which").arg("xsltproc").output() {
        Ok(output) => {
            if !output.status.success() {
                println!("⚠️ xsltproc not found, skipping HTML conversion");
                return Ok(());
            }
        },
        Err(_) => {
            println!("⚠️ Unable to check for xsltproc, skipping HTML conversion");
            return Ok(());
        }
    }
    
    // First try to find the XSL file in the filesystem using ResourceManager
    let rose_pine_xsl = match crate::utils::fs::ResourceManager::get_resource_path("rose_pine_nmap.xsl") {
        Some(path) => {
            println!("✓ Found Rose Pine stylesheet at: {}", path.display());
            path.to_string_lossy().to_string()
        },
        None => {
            // If not found, create a temporary file with the embedded content
            println!("🔄 Creating temporary Rose Pine stylesheet...");
            match crate::utils::fs::ResourceManager::create_temp_resource("rose_pine_nmap.xsl", crate::utils::ROSE_PINE_XSL) {
                Ok(temp_path) => {
                    println!("✓ Created temporary Rose Pine stylesheet at: {}", temp_path.display());
                    temp_path.to_string_lossy().to_string()
                },
                Err(e) => {
                    println!("⚠️ Failed to create temporary stylesheet: {}", e);
                    return Ok(());
                }
            }
        }
    };
    
    // Convert XML to HTML with our custom stylesheet
    let output = Command::new("xsltproc")
        .args(&["-o", &html_file, &rose_pine_xsl, &xml_file])
        .output()?;
    
    if !output.status.success() {
        // Fallback to standard nmap.xsl if our custom stylesheet fails
        println!("⚠️ Custom stylesheet failed, falling back to standard NMAP stylesheet");
        
        // NMAP XSL stylesheet path
        let nmap_xsl = match Path::new("/usr/share/nmap/nmap.xsl").exists() {
            true => "/usr/share/nmap/nmap.xsl",
            false => match Path::new("/usr/local/share/nmap/nmap.xsl").exists() {
                true => "/usr/local/share/nmap/nmap.xsl",
                false => {
                    println!("⚠️ NMAP XSL stylesheet not found, skipping HTML conversion");
                    return Ok(());
                }
            }
        };
        
        // Try with standard stylesheet
        let fallback_output = Command::new("xsltproc")
            .args(&["-o", &html_file, nmap_xsl, &xml_file])
            .output()?;
            
        if !fallback_output.status.success() {
            return Err(format!("Error converting XML to HTML: {}", 
                String::from_utf8_lossy(&fallback_output.stderr)).into());
        }
    } else {
        println!("✓ Generated HTML report with Rose Pine theme");
    }
    
    Ok(())
}