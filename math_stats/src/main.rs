// Given a list of integers, use a vector and return the mean, median and mode
// of the list.
//
// Some insights gotten from https://codereview.stackexchange.com/a/173437/77114

use std::collections::HashMap;

fn calculate_mean(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn calculate_median(numbers: &mut [i32]) -> f32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    // check if the length of the array is even
    if numbers.len() % 2 == 0 {
        (numbers[mid-1] + numbers[mid]) as f32 / 2.0
    } else {
        numbers[mid] as f32
    }
}

fn calculate_mode(numbers: &[i32]) -> i32 {
    let mut number_count = HashMap::new();

    for number in numbers {
        let count = number_count.entry(number).or_insert(0);
        *count += 1;
    }
    *number_count.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn main() {
    let mut numbers = [30, 1, 43, 64, 32, 18, 97, 21, 43, 52];
    println!("The given array is {:?}", numbers);

    println!("Mean:   {}", calculate_mean(&numbers));
    println!("Median: {}", calculate_median(&mut numbers));
    println!("Mode:   {}", calculate_mode(&numbers));
}
