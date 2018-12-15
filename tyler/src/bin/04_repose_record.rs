extern crate adventofcode;
extern crate regex;

use regex::Regex;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct GuardTimestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    id: u16,
    action: u8
}

fn parse_input(input: String) -> Vec<GuardTimestamp> {
    // Create an empty vector for fabric plans.
    let mut guard_timestamps: Vec<GuardTimestamp> = Vec::new();

    // Create a regex for the input.
    let re = Regex::new(r"\[(\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\]( Guard #(\d{1,4}))? (.*)").unwrap();

    // Iterate over input that matches the regex.
    for cap in re.captures_iter(&input) {
        let mut guard_id = 0;
        let mut guard_action = 0;

        // Determine the values of ID and action.
        if cap[8].contains("falls") {
            guard_action = 2;
        } else if cap[8].contains("wakes") {
            guard_action = 3;
        } else {
            guard_id = cap[7].parse::<u16>().unwrap();
            guard_action = 1;
        }

        // Add the guard timestamp to the collection of guard timestamps.
        guard_timestamps.push(GuardTimestamp {
            year: cap[1].parse::<u16>().unwrap(),
            month: cap[2].parse::<u8>().unwrap(),
            day: cap[3].parse::<u8>().unwrap(),
            hour: cap[4].parse::<u8>().unwrap(),
            minute: cap[5].parse::<u8>().unwrap(),
            id: guard_id,
            action: guard_action
        });
    }

    // Order guard timestamps.
    guard_timestamps.sort();

    // Return guard timestamps.
    guard_timestamps
}

fn main() {
    println!("You are running day 4 of the advent calendar!");
    let input = adventofcode::read_input_file(4);
    let guard_timestamps = parse_input(input);
    println!("Number of timestamps: {}", guard_timestamps.len());
    println!("Year: {} | Action: {}", guard_timestamps[0].year, guard_timestamps[0].action);
}