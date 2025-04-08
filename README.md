# 🛡️ RustRecon 🔍

A comprehensive network reconnaissance tool written in Rust that combines NMAP, Shodan, WHOIS, and more for efficient security scanning and information gathering.

## ✨ Features

- 🔎 Run different types of NMAP scans (quick, comprehensive, stealthy)
- 🌐 Shodan integration for additional host information
- 🔖 WHOIS domain registration lookup
- 🔤 DNS reconnaissance (nslookup, dig, dnsrecon)
- 🕸️ Web server scanning (Gobuster, Nikto, WhatWeb)
- 🖥️ Windows/Samba enumeration (enum4linux)
- 🔒 SSL/TLS configuration checking
- 🛣️ Network path discovery (traceroute)
- 📊 Beautiful reports with easy-to-read summaries

## 🚀 Installation

### Prerequisites

- Rust and Cargo (https://rustup.rs/)
- External tools (for full functionality):
  - nmap
  - whois
  - dig/nslookup
  - gobuster
  - nikto
  - enum4linux
  - dnsrecon
  - whatweb
  - openssl

### Install from source

```bash
# Clone the repository
git clone https://github.com/00xNetrunner/rust_recon.git
cd rust_recon

# Build the project
cargo build --release

# Install system-wide
sudo cp target/release/rust_recon /usr/local/bin/
```

## 🔧 Shodan API Setup

To use the Shodan integration:

1. Create a Shodan account at https://account.shodan.io/register
2. Get your API key from https://account.shodan.io/
3. Set your API key as an environment variable:

```bash
export SHODAN_API_KEY=your_api_key_here
```

For permanent setup, add to your shell profile:

```bash
echo 'export SHODAN_API_KEY=your_api_key_here' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc  # or source ~/.zshrc
```

## 📋 Usage Examples

Basic scan:
```bash
rust_recon -i scanme.nmap.org
```

Quick scan:
```bash
rust_recon -i scanme.nmap.org --quick
```

Comprehensive scan:
```bash
rust_recon -i scanme.nmap.org --comprehensive
```

Stealthy scan:
```bash
rust_recon -i scanme.nmap.org --stealthy
```

Full reconnaissance with Shodan:
```bash
rust_recon -i scanme.nmap.org --comprehensive --shodan --whois --dig --nslookup --ssl-check --traceroute
```

Web application scanning:
```bash
rust_recon -i scanme.nmap.org --whatweb --nikto --gobuster --gobuster-wordlist /usr/share/wordlists/dirb/common.txt
```

For more options:
```bash
rust_recon --help
```

## 📄 License

MIT License

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.