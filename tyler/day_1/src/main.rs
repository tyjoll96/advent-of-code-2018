use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // Declare variables.
    let filename = "input.txt".to_string(); // Filename to read data from.
    let mut frequency: i32 = 0; // "Frequency" to be changed by input.

    // Open the file.
    let f = File::open(filename).expect("File not found.");

    // Create a BufReader from the file.
    let f = BufReader::new(f);

    // Iterate over the lines in the file.
    for line in f.lines() {
        // Convert the line to a 32-bit integer and add it to the frequency.
        frequency += line.unwrap().parse::<i32>().unwrap();
    }

    // Display the end result.
    println!("The calibrated frequency is: {}", frequency);
}
