use std::io;

fn main() {
    let mut unit = String::new();
    let mut temperature = String::new();

    println!("--- Welcome to the rusty Temperature Converter program! --- \n");

    // Unit input
    println!("Would you like to convert from Fahrenheit (F) or from Celsius(C)?");

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    // shadowing previous value
    let unit = unit.trim();

    // Temperature input
    println!("please enter a temperature to be converted");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    // Shadowing the previous value of temperature
    let temperature: f32 = match temperature.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => panic!("please provide a number"), // how to handle this properly ?
    };

    if unit == "C" {
        println!("You choosed to convert {temperature}°C to Fahrenheit.");
        let temperature = (temperature * 9.0 / 5.0) + 32.0;
        println!("result of the conversion: {temperature}°F");
    } else {
        println!("You choosed to convert {temperature}°F to Celsius.");
        let temperature = (temperature - 32.0) * 5.0 / 9.0;
        println!("result of the conversion: {temperature}°C");
    }
}
