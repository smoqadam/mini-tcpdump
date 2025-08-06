## About

A simple network packet analyzer written in Rust for learning purposes. This project demonstrates Rust programming concepts and networking fundamentals by implementing basic packet capture and filtering functionality similar to tcpdump. Features include protocol filtering (TCP/UDP), port-based filtering, host filtering, and multiple output formats.

## Installation and Usage

### Prerequisites
- Rust (latest stable version)
- Root/administrator privileges (required for packet capture)

### Installation
```bash
git clone <repository-url>
cd mini-tcpdump
cargo build --release
```

### Usage
```bash
# Basic packet capture on interface eth0
sudo ./target/release/mini-tcpdump -i eth0

# Filter by protocol and port
sudo ./target/release/mini-tcpdump -i eth0 --protocol tcp --port 80

# JSON output format
sudo ./target/release/mini-tcpdump -i eth0 --format json

# Filter by source/destination
sudo ./target/release/mini-tcpdump -i eth0 --src-host 192.168.1.1 --dst-port 443
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests, report bugs, or suggest features through GitHub issues.

## License

This project is licensed under the MIT License.