# Rust GCD Calculator

A simple Rust project demonstrating GCD (Greatest Common Divisor) calculation with both CLI and web interfaces. This project showcases Rust's capabilities in building both command-line tools and web applications.

## Project Structure

The project is organized as a workspace with three main components:

- `gcd-core`: Core library containing the GCD calculation logic
- `cli`: Command-line interface for GCD calculation
- `web`: Web interface for GCD calculation

## Installation

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

Clone this repository:
```bash
git clone <repository-url>
cd gcd-calculator
```

## Installation Options

cargo install gcd-calculator-cli


### CLI Tool

To calculate GCD using the CLI:
```bash
cargo run -p cli -- gcd <number1> <number2>
```

Example:
```bash
cargo run -p cli -- gcd 36 20
```

### Web Interface

To start the web server:
```bash
cargo run -p web
```

Then open your browser and navigate to `http://localhost:7878`. The web interface allows you to input two numbers and calculate their GCD.

## Development

To build all components:
```bash
cargo build
```

To run tests:
```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

Based on the tutorial from [Medium article](https://medium.com/@benacq44/practical-rust-for-transitioning-engineers-baebaf20468c)
