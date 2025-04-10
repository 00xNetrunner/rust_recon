use clap::ArgMatches;
use console::style;
use indicatif::MultiProgress;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::ui::progress::create_spinner;

// Run NMAP scan with specified options and animated progress with status updates
pub fn run_nmap_scan(mp: &MultiProgress, matches: &ArgMatches, target: &str, base_filename: &str, scan_dir: &str) -> Result<Output, Box<dyn Error>> {
    // Add arguments
    let mut nmap_args = vec![];
    
    // Add nmap stats progress flag to enable verbose status updates
    nmap_args.push("--stats-every");
    nmap_args.push("30s");  // Update every 30 seconds
    
    // Comprehensive scan (simplified to avoid redundancies since -A already includes -sC and -sV and OS detection)
    if matches.is_present("comprehensive") {
        nmap_args.extend(vec!["-A", "-p-"]);
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
    
    // Prepare stdout file
    let stdout_output = format!("{}/{}_nmap_stdout.txt", scan_dir, base_filename);
    let stdout_file = File::create(&stdout_output)?;
    let stdout_file = Arc::new(Mutex::new(stdout_file));
    
    // Record the start time
    let start_time = Instant::now();
    
    // Execute NMAP command with piped output to capture progress
    let mut command = Command::new("nmap")
        .args(nmap_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    
    // Capture stdout
    let stdout = command.stdout.take().ok_or("Failed to capture stdout")?;
    let stdout_reader = BufReader::new(stdout);
    
    // Collect all output for final result
    let collected_output = Arc::new(Mutex::new(Vec::new()));
    
    // Update the spinner based on NMAP progress output
    let spinner_clone = spinner.clone();
    let collected_output_clone = Arc::clone(&collected_output);
    let stdout_file_clone = Arc::clone(&stdout_file);
    
    // Create a thread to handle stdout and update progress
    let stdout_thread = thread::spawn(move || {
        let mut progress_percent = 0;
        let mut current_phase = String::new();
        
        for line in stdout_reader.lines().flatten() {
            // Write line to the output file
            if let Ok(mut file) = stdout_file_clone.lock() {
                let _ = writeln!(file, "{}", line);
            }
            
            // Save for final output
            if let Ok(mut output) = collected_output_clone.lock() {
                output.push(line.clone().into_bytes());
            }
            
            // Update current task and progress
            if line.contains("About") && line.contains("done") {
                // Update progress
                if let Some(pct) = extract_percentage(&line) {
                    progress_percent = pct;
                }
                
                // Update task info
                let elapsed = start_time.elapsed();
                let elapsed_secs = elapsed.as_secs();
                let elapsed_str = format!(
                    "{}:{:02}:{:02}", 
                    elapsed_secs / 3600, 
                    (elapsed_secs % 3600) / 60, 
                    elapsed_secs % 60
                );
                
                spinner_clone.set_message(format!(
                    "NMAP {} scan progress: {}% | Running for {} | {}", 
                    style(scan_type).yellow(),
                    style(progress_percent).green(),
                    style(elapsed_str).blue(),
                    extract_current_task(&line)
                ));
            }
            
            // Detect current task
            if line.contains("Initiating") {
                current_phase = extract_current_task(&line);
                spinner_clone.set_message(format!(
                    "NMAP {} scan: {} | {}%", 
                    style(scan_type).yellow(),
                    style(&current_phase).cyan(),
                    style(progress_percent).green()
                ));
            }
        }
    });
    
    // Wait for the command to complete
    let result = command.wait()?;
    
    // Wait for the thread to finish processing output
    let _ = stdout_thread.join();
    
    // Build the final output
    let mut stdout_content = Vec::new();
    if let Ok(output) = collected_output.lock() {
        for line in output.iter() {
            stdout_content.extend_from_slice(line);
            stdout_content.push(b'\n');
        }
    }
    
    // Create an Output struct to maintain the same return type
    let output = Output {
        status: result,
        stdout: stdout_content,
        stderr: Vec::new(),
    };
    
    // Calculate the total scan time
    let total_time = start_time.elapsed();
    let total_seconds = total_time.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    // Check if scan was successful and finish the spinner
    if output.status.success() {
        spinner.finish_with_message(format!(
            "{} NMAP {} scan completed successfully in {}:{:02}:{:02}", 
            style("✓").green(), 
            scan_type,
            hours,
            minutes,
            seconds
        ));
    } else {
        spinner.finish_with_message(format!(
            "{} NMAP scan encountered issues after {}:{:02}:{:02}", 
            style("⚠").yellow(),
            hours,
            minutes,
            seconds
        ));
    }
    
    Ok(output)
}

// Helper function to extract percentage from NMAP output
fn extract_percentage(line: &str) -> Option<u32> {
    if let Some(pct_str) = line.split("About ").nth(1) {
        if let Some(pct_num) = pct_str.split('%').next() {
            return pct_num.trim().parse::<u32>().ok();
        }
    }
    None
}

// Helper function to extract the current task from NMAP output
fn extract_current_task(line: &str) -> String {
    if line.contains("SYN Stealth Scan") {
        return "SYN Stealth Scanning".to_string();
    } else if line.contains("Service scan") {
        return "Service Version Detection".to_string();
    } else if line.contains("NSE") {
        return "Script Scanning".to_string();
    } else if line.contains("OS detection") {
        return "OS Detection".to_string();
    } else if line.contains("Traceroute") {
        return "Traceroute".to_string();
    } else if line.contains("Host discovery") {
        return "Host Discovery".to_string();
    }
    
    line.to_string()
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
        .args(["-o", &html_file, &rose_pine_xsl, &xml_file])
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
            .args(["-o", &html_file, nmap_xsl, &xml_file])
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