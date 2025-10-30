# Rust Temperature Converter

A simple temperature converter CLI program written in Rust. Converts between Celsius and Fahrenheit. This project was created as a learning exercise to explore the fundamentals of Rust.

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

3.  **Run the project:**

    ```bash
    cargo run
    ```

## API Reference

The library provides the following functions:

*   `celsius_to_fahrenheit(celsius: f64) -> f64`: Converts a Celsius temperature (as a `f64`) to Fahrenheit (as a `f64`).
*   `fahrenheit_to_celsius(fahrenheit: f64) -> f64`: Converts a Fahrenheit temperature (as a `f64`) to Celsius (as a `f64`).

## Examples output from CLI

```bash
--- Welcome to the rusty Temperature Converter program! --- 

Would you like to convert from Fahrenheit (F) or from Celsius(C)?
F
please enter a temperature to be converted (expected format: 00.0)
68
You choosed to convert 68°F  from Celsius.
result of the conversion: 20°C
