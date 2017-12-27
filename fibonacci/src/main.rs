/**
 * Generate the nth Fibonacci number
 */

use std::io;

fn gen_fibonacci(number: u64) -> u64 {
    if number == 0 {
        0
    } else if number == 1 || number == 2 {
        1
    } else {
        gen_fibonacci(number - 1) + gen_fibonacci(number - 2)
    }
}

fn main() {
    println!("Generate the nth Fibonacci number");

    loop {
        let mut number = String::new();

        println!("Enter a number.");
        io::stdin().read_line(&mut number).expect("Failed to read line");

        let number: u64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!(
            "The Fibonacci number for {} is {}",
            number, gen_fibonacci(number)
        );
    }
}
