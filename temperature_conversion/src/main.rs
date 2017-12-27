/**
 * Convert temperatures between Fahrenheit and Celsius.
 */

use std::io;

fn convert_temperature(temperature: f64, temp_type: char) -> f64 {
    if temp_type == 'F' {
        // convert to celsius
        (temperature - 32.0) * 0.5556
    } else if temp_type == 'C' {
        // convert to fahrenheit
        temperature * 1.8 + 32.0
    } else {
        temperature
    }
}

fn main() {
    println!("Convert temperatures between Fahrenheit and Celsius");

    loop {
        let mut choice = String::new();

        println!("Choose the type of conversion you want. [1, 2, 3]");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Exit");

        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let temp_type: char = match choice.chars().nth(0).unwrap() {
            '1' => 'F',
            '2' => 'C',
            '3' => break,
            _ => continue,
        };

        let mut temperature = String::new();
        println!("Enter your temperature.");
        io::stdin().read_line(&mut temperature).expect("Failed to read line");
        let temperature = temperature.trim().parse::<f64>().unwrap();

        println!(
            "Your converted temperature is {:.1}",
            convert_temperature(temperature, temp_type)
        );
    }
}
