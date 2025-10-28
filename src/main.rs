use std::io;

fn main() {

    println!("--- Welcome to the rusty Temperature Converter program! --- \n");

    //println!("Would you like to convert from Fahrenheit (F) or from Celsius(C)?");
    //let mut unit = String::new();

    println!("please enter a temperature to be converted");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    //Shadowing the previous value of temperature
    let temperature: f32 = temperature.trim().parse().expect("enter a float please");

    println!("You entered: {temperature}°C");

    /*io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    println!("{unit}");*/

    // give feedback
   //if unit=="C" {
    //    println!("You choosed to convert from Celsius.")
    //} else {
    //    println!("You choosed to convert from Fahrenheit.")
    //}

    // convert the float to the correct unit
    let temperature = (temperature * 9.0 /5.0) + 32.0;
    // Provide the value to the user
    println!("result of the conversion: {temperature}°F");
}
