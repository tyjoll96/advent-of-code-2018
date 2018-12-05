extern crate adventofcode;

use std::collections::BTreeSet;

fn main() {
    // Declare variables.
    let mut done = false; // Flag for when a frequency has occured twice.
    let mut frequency: i32 = 0; // "Frequency" to be changed by input.
    let mut inputs: Vec<i32> = Vec::new(); // Vector of inputs taken from the input file.
    let mut prev_frequencies: BTreeSet<i32> = BTreeSet::new(); // Vector to hold previous frequency values.

    // Iterate over the lines in the file.
    for line in adventofcode::read_input_file(1).lines() {
        // Add them to the inputs vector.
        inputs.push(line.parse::<i32>().unwrap());
    }

    // Keep iterating over the list of inputs until the second occurance of a frequency is found.
    while !done {
        // Iterate over the inputs.
        for input in &inputs {
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
            prev_frequencies.insert(frequency);
        }
    }

    // Display the end result.
    println!("The calibrated frequency is: {}", frequency);
}
