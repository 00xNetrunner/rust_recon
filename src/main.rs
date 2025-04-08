// RustRecon V18 - A Rust-based network reconnaissance tool
// Author: Claude
// Description: This tool wraps NMAP and other reconnaissance tools,
//              providing various scan types and integrations.

use std::error::Error;
use std::env;

// Module imports
mod cli;
mod ui;
mod core;
mod scanners;
mod api;
mod report;
mod utils;
mod models;

// Main entry point
fn main() -> Result<(), Box<dyn Error>> {
    // Special case for examples: Check arguments directly to allow --examples to work alone
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && (args[1] == "--examples" || args[1] == "-e") {
        cli::print_examples();
        return Ok(());
    }
    
    // Define command-line arguments for normal operation
    let matches = cli::parse_args();
    
    // If examples flag is provided after other arguments
    if matches.is_present("examples") {
        cli::print_examples();
        return Ok(());
    }
    
    // Run scans with the provided options
    core::run_scans(&matches)?;
    
    Ok(())
}