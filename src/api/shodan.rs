use console::style;
use indicatif::ProgressBar;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::env;
use serde_json::Value;
use reqwest;

// Perform Shodan lookup with spinner
pub fn perform_shodan_lookup(spinner: &ProgressBar, target: &str, base_filename: &str, scan_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_file = format!("{}/{}_shodan.json", scan_dir, base_filename);
    
    // Update spinner message
    spinner.set_message(format!("Connecting to Shodan API for {}...", style(target).cyan()));
    
    // Check for Shodan API key
    let api_key = match env::var("SHODAN_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            spinner.suspend(|| {
                println!("⚠️ SHODAN_API_KEY environment variable not set");
                println!("⚠️ Please set your Shodan API key with: export SHODAN_API_KEY=your_api_key");
            });
            return Err("Shodan API key not found in environment variables".into());
        }
    };
    
    // Make API request to Shodan
    spinner.set_message(format!("Querying Shodan database for {}...", style(target).cyan()));
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.shodan.io/shodan/host/{}?key={}", target, api_key);
    
    match client.get(&url).send() {
        Ok(response) => {
            if response.status().is_success() {
                spinner.set_message(format!("Processing Shodan data for {}...", style(target).cyan()));
                let json: Value = response.json()?;
                let mut file = File::create(output_file)?;
                file.write_all(serde_json::to_string_pretty(&json)?.as_bytes())?;
                Ok(())
            } else {
                Err(format!("Shodan API error: {}", response.status()).into())
            }
        },
        Err(e) => {
            Err(format!("Error connecting to Shodan API: {}", e).into())
        }
    }
}