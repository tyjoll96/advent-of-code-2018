// use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_input_file(day: usize) -> String {
    // let filename = env::args().nth(1).unwrap_or_else(|| format!("input/{:02}", day));
    let filename = format!("input/{:02}", day);
    let path = Path::new(&filename);

    let mut file = File::open(&path).expect("couldnt open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("couldn't read file");

    s
}