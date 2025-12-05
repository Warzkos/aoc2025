use std::fs::read_to_string;

fn create_non_overlapping_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by_key(|&(start, _)| start);

    let mut non_overlapping: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges.iter() {
        if let Some((_, last_end)) = non_overlapping.last_mut() {
            if *start <= *last_end {
                *last_end = (*last_end).max(*end);
            } else {
                non_overlapping.push((*start, *end));
            }
        } else {
            non_overlapping.push((*start, *end));
        }
    }

    non_overlapping
}

fn main() {
    let mut ranges: Vec<(u64, u64)> = read_to_string("../input.txt")
        .expect("Failed to read input file")
        .split("\n\n")
        .nth(0)
        .expect("Failed to get first section")
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").expect("Failed to split range");
            (
                start
                    .parse::<u64>()
                    .expect("Failed to parse start of range"),
                end.parse::<u64>().expect("Failed to parse end of range"),
            )
        })
        .collect();

    let result = create_non_overlapping_ranges(&mut ranges)
        .iter()
        .map(|(start, end)| (end - start) + 1)
        .sum::<u64>();

    println!("Total non-overlapping range length: {}", result);
}
