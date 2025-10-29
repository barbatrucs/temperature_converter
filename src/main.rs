use std::io;

fn main() {
    println!("--- Welcome to the rusty Temperature Converter program! --- \n");

    // Unit input
    println!("Would you like to convert from Fahrenheit (F) or from Celsius(C)?");

    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let unit = match unit.trim() {
        "C" => "C",
        "F" => "F",
        _ => "please enter a valid unit, either C for Celsius or F for Fahrenheit",
    };

    // Temperature input
    println!("please enter a temperature to be converted (expected format: 00.0)");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    //Shadowing the previous value of temperature
    let temperature: f32 = temperature.trim().parse().expect("enter a float please");

    println!("You entered: {temperature}°C");

    if unit == "C" {
        println!("You choosed to convert from Celsius.");
        let temperature = (temperature * 9.0 / 5.0) + 32.0;
        println!("result of the conversion: {temperature}°F");
    } else {
        println!("You choosed to convert from Fahrenheit.");
        let temperature = (temperature - 32.0) * 5.0 / 9.0;
        println!("result of the conversion: {temperature}°C");
    }
}
