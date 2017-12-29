// Calculate the BMI of a person given their weight (kg) and height (m).
// This program is aimed at learning about structs.

use std::io;

#[derive(Debug)]
struct Body {
    weight: f32,
    height: f32,
}

#[derive(Debug)]
struct BMI {
    index: f32,
    status: String,
}

impl Body {
    fn bmi(&self) -> BMI {
        let index = self.weight / (self.height * self.height);
        // this BMI categorization is for adults > 20
        let status = if index < 18.5 {
            String::from("UNDERWEIGHT")
        } else if index >= 18.5 && index < 25.0 {
            String::from("NORMAL")
        } else if index >= 25.0 && index < 30.0 {
            String::from("OVERWEIGHT")
        } else {
            String::from("OBESE")
        };

        BMI {
            index,
            status,
        }
    }
}

fn main() {
    println!("Body Mass Index Calculator");

    let mut weight = String::new();
    let mut height = String::new();

    println!("Enter your weight (kg):");
    io::stdin().read_line(&mut weight).expect("Failed to read line");
    let weight: f32 = weight.trim().parse::<f32>().expect("Invalid weight");

    println!("Enter your height (m):");
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let height: f32 = height.trim().parse::<f32>().expect("Invalid height");

    let body = Body {
        weight,
        height,
    };
    println!(
        "Your BMI is {} which is {}", body.bmi().index, body.bmi().status
    );
}
