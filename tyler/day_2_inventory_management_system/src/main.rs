use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // Class to hold number of times a letter has occurred.
    struct Occurrence {
        letter: char,
        num: u8
    }

    // Declare variables.
    let filename = "input.txt".to_string(); // Filename to read data from.
    let mut inputs: Vec<String> = Vec::new(); // Vector of inputs taken from the input file.
    let mut num_three_letter_ids: i16 = 0; // Counter for IDs that contain three of a letter.
    let mut num_two_letter_ids: i16 = 0; // Counter for IDs that contain two of a letter.

    // Open the file.
    let f = File::open(filename).expect("File not found.");

    // Create a BufReader from the file.
    let f = BufReader::new(f);

    // Iterate over the lines in the file.
    for line in f.lines() {
        // Add them to the inputs vector.
        inputs.push(line.unwrap().trim().to_string());
    }

    // Iterate over the IDs given.
    for input in &inputs {
        // Create an empty vector to hold the occurrences for the current input.
        let mut occurrences: Vec<Occurrence> = Vec::new();

        // Iterate over the characters of the string.
        for character in input.chars() {
            // Flag for if the character has occurred already.
            let mut has_already_occurred = false;

            // Iterate over the occurrences.
            for occurrence in &mut occurrences {

                // If an occurrence of the given letter has already appeared.
                if occurrence.letter == character {
                    // Increment occurence number.
                    occurrence.num += 1;

                    // Raise flag that the letter exist.
                    has_already_occurred = true;
                }
            }

            // Check if the letter already has an occurrence.
            if !has_already_occurred {
                // Create an occurrence for the letter.
                occurrences.push(Occurrence{ letter: character, num: 1 });
            }
        }

        // Flags for if an occurence has happened more than once.
        let mut has_two_occurred = false;
        let mut has_three_occurred = false;

        // Iterate over the occurrences.
        for occurrence in occurrences {
            // Check if a letter has occurred two times.
            if occurrence.num == 2 && !has_two_occurred {
                // Increment the two letter counter.
                num_two_letter_ids += 1;

                // Set the flag to true.
                has_two_occurred = true;
            
            // Check if a letter has occurred three times.
            } else if occurrence.num == 3 && !has_three_occurred {
                // Increment the two letter counter.
                num_three_letter_ids += 1;

                // Set the flag to true.
                has_three_occurred = true;
            }
        }
    }

    // Print checksum of IDs.
    println!("2: {}, 3: {}, Checksum: {}", num_two_letter_ids, num_three_letter_ids, num_two_letter_ids * num_three_letter_ids);

    // Print the combined similar ID.
    println!("{}", combined_similar_id(&inputs));
}

// Find two IDs that vary by only one letter difference and return a combined string with the mismatch character removed.
fn combined_similar_id(inputs: &Vec<String>) -> String {
    // Iterate over the inputs for the left side of the comparison.
    for x in 0..inputs.len() {
        // Iterate over the input for the right side of the comparison.
        for y in 0..inputs.len() {
            // Don't compare an ID to itself.
            if (x == y) { continue; }
            let mut differences: u8 = 0; // Number of different characters in ID.
            let mut final_id: String = String::new(); // Letter that is different between the IDs.

            // Get the chars within the left side ID.
            let mut id_x_chars = inputs[x].chars();
            // Get the chars within the right side ID.
            let mut id_y_chars = inputs[y].chars();

            // Iterate over the letters in the ID. Luckily all IDs are the same length so we don't need to worry about our counter too much.
            for letter in inputs[x].chars().enumerate() {
                // Get the current character being iterated over on both sides.
                let id_x_char = id_x_chars.next().unwrap();
                let id_y_char = id_y_chars.next().unwrap();

                // Check if the characters are the same.
                if id_x_char != id_y_char {
                    // Iterate the difference counter.
                    differences += 1;
                } else {
                    // Add the matching characters to the final ID to be returned at the end.
                    final_id.push(id_x_char);
                }
            }

            // Check if the difference is 1, luckily no IDs matched exactly so that check was left out.
            if differences < 2 {
                return final_id;
            }
        }
    }

    // Output that no valid output was found from the input.
    "No similar ID combination found. :(".to_string()
}