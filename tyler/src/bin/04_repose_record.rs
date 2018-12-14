extern crate adventofcode;
extern crate regex;

use regex::Regex;

struct GuardTimestamp {
    action: String,
    day: u8,
    hour: u8,
    minute: u8,
    month: u8,
    year: u16
}

fn parse_input(input: String) -> Vec<GuardTimestamp> {
    // Create an empty vector for fabric plans.
    let mut guard_timestamps: Vec<GuardTimestamp> = Vec::new();

    // Create a regex for the input.
    let re = Regex::new(r"\[(\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] (.*)").unwrap();

    // Iterate over input that matches the regex.
    for cap in re.captures_iter(&input) {
        // Add the fabric plan to the collection of fabric plans.
        guard_timestamps.push(GuardTimestamp {
            action: cap[6].to_string(),
            day: cap[3].parse::<u8>().unwrap(),
            hour: cap[4].parse::<u8>().unwrap(),
            minute: cap[5].parse::<u8>().unwrap(),
            month: cap[2].parse::<u8>().unwrap(),
            year: cap[1].parse::<u16>().unwrap()
        });
    }

    guard_timestamps
}

fn main() {
    println!("You are running day 4 of the advent calendar!");
    let input = adventofcode::read_input_file(4);
    let guard_timestamps = parse_input(input);
    println!("Number of timestamps: {}", guard_timestamps.len());
    println!("Year: {} | Action: {}", guard_timestamps[0].year, guard_timestamps[0].action);
}