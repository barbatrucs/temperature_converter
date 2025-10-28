# Rust Temperature Converter

A simple temperature converter library written in Rust. Converts between Celsius and Fahrenheit. This project was created as a learning exercise to explore the fundamentals of Rust.

## Features

*   **Celsius to Fahrenheit Conversion:** Converts Celsius temperatures to Fahrenheit.
*   **Fahrenheit to Celsius Conversion:** Converts Fahrenheit temperatures to Celsius.
*   **Written to learn Rust:**  Leverages the safety and performance benefits of the Rust programming language.

## Getting Started

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) and Cargo (Rust's package manager) must be installed on your system.

### Usage

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/barbatrucs/temperature_converter/
    cd rust-temperature-converter
    ```

2.  **Build the project:**

    ```bash
    cargo build
    ```

3.  **Run the project (example):**

    You can either use the library in your own Rust project or create a simple command-line interface (CLI) to use it directly.  Here's a basic example of how you might use the library in a `src/main.rs` file:

    ```rust
    use rust_temperature_converter::{celsius_to_fahrenheit, fahrenheit_to_celsius};

    fn main() {
        let celsius = 25.0;
        let fahrenheit = celsius_to_fahrenheit(celsius);
        println!("{}°C is {}°F", celsius, fahrenheit);

        let fahrenheit = 77.0;
        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("{}°F is {}°C", fahrenheit, celsius);
    }
    ```

    Add the following to your `Cargo.toml` file under `[dependencies]`:

    ```toml
    rust-temperature-converter = { path = "./" } # Assuming your library is in the same directory
    ```

    Then run:

    ```bash
    cargo run
    ```

## API Reference

The library provides the following functions:

*   `celsius_to_fahrenheit(celsius: f64) -> f64`: Converts a Celsius temperature (as a `f64`) to Fahrenheit (as a `f64`).
*   `fahrenheit_to_celsius(fahrenheit: f64) -> f64`: Converts a Fahrenheit temperature (as a `f64`) to Celsius (as a `f64`).

## Examples

```rust
use rust_temperature_converter::{celsius_to_fahrenheit, fahrenheit_to_celsius};

let celsius = 0.0;
let fahrenheit = celsius_to_fahrenheit(celsius); // fahrenheit will be 32.0

let fahrenheit = 212.0;
let celsius = fahrenheit_to_celsius(fahrenheit); // celsius will be 100.0
