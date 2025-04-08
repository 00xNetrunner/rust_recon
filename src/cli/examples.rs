// Function to display example commands
pub fn print_examples() {
    println!("\n🔰 EXAMPLE COMMANDS:");
    println!("\n1️⃣  Basic Quick Scan:");
    println!("    ./rust_recon -i 192.168.1.1 --quick");
    println!("    This performs a fast scan of common ports");
    
    println!("\n2️⃣  Quick Scan with Custom Options:");
    println!("    ./rust_recon -i 192.168.1.1 --quick --quick-options \"-T3 -p 80,443,8080,8443,3389\"");
    println!("    Quick scan with custom port selection and timing template");
    
    println!("\n3️⃣  Comprehensive Scan with WHOIS:");
    println!("    ./rust_recon -i 10.0.0.1 --comprehensive --whois");
    println!("    Full port scan with service detection, OS detection, and WHOIS lookup");
    
    println!("\n4️⃣  Stealthy Scan with Firewall Bypass:");
    println!("    ./rust_recon -i target.com --stealthy --firewall-bypass");
    println!("    Slow, methodical scan designed to evade detection and bypass firewalls");
    
    println!("\n5️⃣  Full Reconnaissance with All Tools:");
    println!("    ./rust_recon -i 8.8.8.8 --comprehensive --scripts \"http,vuln\" --whois --shodan --nslookup --dig");
    println!("    Complete scan with HTTP and vulnerability scripts, plus all external tools");
    
    println!("\n6️⃣  Proxied Scan through Tor:");
    println!("    ./rust_recon -i target.com --noisey --proxy \"socks5://127.0.0.1:9050\"");
    println!("    Aggressive scan routed through a SOCKS5 proxy (like Tor) to hide source");
    
    println!("\n7️⃣  Web Application Scanning:");
    println!("    ./rust_recon -i example.com --quick --gobuster --gobuster-wordlist \"/usr/share/wordlists/dirb/common.txt\" --nikto --whatweb");
    println!("    Web-focused scan that identifies technologies, directories, and vulnerabilities");
    
    println!("\n8️⃣  Windows/SMB Enumeration:");
    println!("    ./rust_recon -i 192.168.1.100 --quick --enum4linux");
    println!("    Targeted scan for Windows/Samba hosts to enumerate users, shares, and policies");
    
    println!("\n9️⃣  Domain Reconnaissance:");
    println!("    ./rust_recon -i example.com --dnsrecon --dig --ssl-check");
    println!("    DNS and certificate analysis for a domain");
    
    println!("\n🔟  Complete Scan with Organization:");
    println!("    ./rust_recon -i target.com --comprehensive --scripts \"default,safe\" --shodan --whois --whatweb --nikto --ssl-check");
    println!("    Thorough scan with results organized in a timestamped directory");
    
    println!("\n📝 NOTES:");
    println!("  - Each scan is automatically saved in its own timestamped directory");
    println!("  - A SCAN_SUMMARY.md file is created in each scan directory with details");
    println!("  - For Shodan lookups, set your API key: export SHODAN_API_KEY=your_api_key");
    println!("  - HTML reports are generated automatically when xsltproc is available");
    println!("  - You can combine any scan types and tools based on your needs");
}