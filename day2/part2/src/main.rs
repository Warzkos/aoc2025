use fancy_regex::Regex;
use std::fs::read_to_string;

fn main() {
    let invalid_id_pattern = Regex::new(r"^([0-9]+)\1+$").expect("Failed to compile regex");

    let result = read_to_string("../input.txt")
        .expect("Failed to read input file")
        .trim_ascii_end()
        .split(",")
        .map(|range| range.split_once('-').expect("Invalid range format"))
        .flat_map(|(start, end)| {
            let start: u64 = start.parse().expect("Invalid start ID");
            let end: u64 = end.parse().expect("Invalid end ID");
            (start..=end).collect::<Vec<u64>>()
        })
        .filter(|&id| {
            invalid_id_pattern
                .is_match(&id.to_string())
                .expect("Regex match failed")
        })
        .sum::<u64>();

    println!("Sum of all invalid IDs: {}", result);
}
