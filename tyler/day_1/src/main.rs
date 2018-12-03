use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // Declare variables.
    let mut done = false; // Flag for when a frequency has occured twice.
    let filename = "input.txt".to_string(); // Filename to read data from.
    let mut frequency: i32 = 0; // "Frequency" to be changed by input.
    let mut inputs: Vec<i32> = Vec::new(); // Vector of inputs taken from the input file.
    let mut prev_frequencies: Vec<i32> = Vec::new(); // Vector to hold previous frequency values.

    // Declare debug variables.
    let mut iteration = 1;

    // Open the file.
    let f = File::open(filename).expect("File not found.");

    // Create a BufReader from the file.
    let f = BufReader::new(f);

    // Iterate over the lines in the file.
    for line in f.lines() {
        // Add them to the inputs vector.
        inputs.push(line.unwrap().parse::<i32>().unwrap());
    }

    // Keep iterating over the list of inputs until the second occurance of a frequency is found.
    while !done {
        println!("Iteration: {}", iteration);
        // Iterate over the inputs.
        for input in inputs.iter() {
            // Modify the frequency with the current input.
            frequency += input;

            // Check if a frequency has occured before.
            if prev_frequencies.contains(&frequency) {
                // Set the flag to true.
                done = true;

                // Debug.
                // println!("A duplicate frequency has occured at: {}", frequency);

                // Stop iterating over the inputs.
                break;
            }

            // Add the new frequency to the list of previously seen frequencies.
            prev_frequencies.push(frequency);
        }

        iteration += 1;
    }

    // Display the end result.
    println!("The calibrated frequency is: {}", frequency);
}
