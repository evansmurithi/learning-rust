// Convert strings to Pig Latin, where the first consonant of each word is
// moved to the end of the word with an added “ay”, so “first” becomes
// “irst-fay”. Words that start with a vowel get “hay” added to the end
// instead (“apple” becomes “apple-hay”).

use std::io;

fn main() {
    println!("Enter word to convert to Pig Latin.");

    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let word = word.trim();

    let first_letter = word.to_lowercase().chars().nth(0)
        .expect("Cannot get first letter");

    match first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => {
            println!("{}-hay", word)
        },
        'a'...'z' => {
            println!("{}-{}ay", &word[1..], first_letter)
        },
        _ => println!("The word {} cannot be Pig Latinized", word),
    };
}
