use clap::{Arg, App, ArgMatches};

pub fn parse_args() -> ArgMatches<'static> {
    // Define command-line arguments for normal operation
    let matches = App::new("RustRecon")
        .version("1.8")
        .author("Ethical Hacker")
        .about("Network reconnaissance tool combining NMAP, Shodan, WHOIS, and more")
        .arg(Arg::with_name("ip")
            .short("i")
            .long("ip")
            .value_name("IP")
            .help("Target IP address or hostname to scan")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("comprehensive")
            .long("comprehensive")
            .help("Run comprehensive scan (-A -p- -sT -sV -O) that checks all ports with version detection"))
        .arg(Arg::with_name("quick")
            .long("quick")
            .help("Run quick scan of top ports only (-F -T4) for faster results"))
        .arg(Arg::with_name("scripts")
            .long("scripts")
            .value_name("SCRIPTS")
            .help("NMAP scripts to run (comma-separated, e.g., 'http,vuln,discovery')")
            .takes_value(true))
        .arg(Arg::with_name("whois")
            .long("whois")
            .help("Perform WHOIS lookup to get domain registration information"))
        .arg(Arg::with_name("shodan")
            .long("shodan")
            .help("Query Shodan API for target information (requires SHODAN_API_KEY env variable)"))
        .arg(Arg::with_name("nslookup")
            .long("nslookup")
            .help("Perform DNS lookup using nslookup tool"))
        .arg(Arg::with_name("dig")
            .long("dig")
            .help("Perform detailed DNS lookup using dig tool with all records"))
        .arg(Arg::with_name("noisey")
            .long("noisey")
            .help("Run aggressive scan (-T5 -A --traceroute) that is fast but easily detected"))
        .arg(Arg::with_name("stealthy")
            .long("stealthy")
            .help("Run stealthy scan (-sS -T2) that's slower but harder to detect by IDS/IPS"))
        .arg(Arg::with_name("firewall-bypass")
            .long("firewall-bypass")
            .help("Use techniques to bypass firewalls (-f --mtu 16 -D RND:5)"))
        .arg(Arg::with_name("proxy")
            .long("proxy")
            .value_name("PROXY")
            .help("Use proxy for scans (e.g., 'socks5://127.0.0.1:9050' for Tor)")
            .takes_value(true))
        .arg(Arg::with_name("output-dir")
            .long("output-dir")
            .value_name("DIR")
            .help("Directory to save all scan results")
            .default_value("./recon_results")
            .takes_value(true))
        .arg(Arg::with_name("examples")
            .long("examples")
            .help("Show example commands and usage scenarios"))
        .arg(Arg::with_name("quick-options")
            .long("quick-options")
            .value_name("OPTIONS")
            .help("Custom options for quick scan (e.g., '-T3 -p 80,443,8080')")
            .takes_value(true))
        .arg(Arg::with_name("gobuster")
            .long("gobuster")
            .help("Perform web directory enumeration using Gobuster")
            .requires("gobuster-wordlist"))
        .arg(Arg::with_name("gobuster-wordlist")
            .long("gobuster-wordlist")
            .value_name("WORDLIST")
            .help("Wordlist for Gobuster (e.g., '/usr/share/wordlists/dirb/common.txt')")
            .takes_value(true))
        .arg(Arg::with_name("nikto")
            .long("nikto")
            .help("Scan web server for vulnerabilities using Nikto"))
        .arg(Arg::with_name("enum4linux")
            .long("enum4linux")
            .help("Enumerate Windows/Samba hosts using enum4linux"))
        .arg(Arg::with_name("whatweb")
            .long("whatweb")
            .help("Identify web technologies using WhatWeb"))
        .arg(Arg::with_name("dnsrecon")
            .long("dnsrecon")
            .help("Perform DNS enumeration using dnsrecon"))
        .arg(Arg::with_name("traceroute")
            .long("traceroute")
            .help("Perform network path discovery using traceroute"))
        .arg(Arg::with_name("ssl-check")
            .long("ssl-check")
            .help("Check SSL/TLS configuration using OpenSSL"))
        .get_matches();
    
    matches
}