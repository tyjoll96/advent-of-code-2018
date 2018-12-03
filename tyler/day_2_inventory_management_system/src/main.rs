use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // Declare variables.
    let filename = "input.txt".to_string(); // Filename to read data from.
    let mut inputs: Vec<String> = Vec::new(); // Vector of inputs taken from the input file.
    let mut num_three_letter_ids: i16 = 0;
    let mut num_two_letter_ids: i16 = 0;

    // Open the file.
    let f = File::open(filename).expect("File not found.");

    // Create a BufReader from the file.
    let f = BufReader::new(f);

    // Iterate over the lines in the file.
    for line in f.lines() {
        // Add them to the inputs vector.
        inputs.push(line.unwrap());
    }
}
