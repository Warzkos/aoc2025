use std::fs::read_to_string;

const INPUT_FILE: &str = "../input.txt";

// const BATTERIES_TO_FIND: usize = 2;
const BATTERIES_TO_FIND: usize = 12;

fn find_max_value_and_index_between_indexes(
    data: &Vec<u32>,
    start: usize,
    end: usize,
) -> Option<(u32, usize)> {
    if start >= end || end > data.len() {
        return None;
    }

    let mut max_value = data[start];
    let mut max_value_index = start;
    for (index, &value) in data[start..end].iter().enumerate() {
        if value > max_value {
            max_value = value;
            max_value_index = start + index;
        }
    }
    Some((max_value, max_value_index))
}

fn main() {
    let result: u64 = read_to_string(INPUT_FILE)
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            let batteries: Vec<u32> = line
                .chars()
                .map(|c| c.to_digit(10).expect("Failed to parse digit"))
                .collect();
            let mut start_index = 0;
            let mut max_values: Vec<u32> = Vec::new();
            for i in (0..BATTERIES_TO_FIND).rev() {
                let (max_value, next_start_index) = find_max_value_and_index_between_indexes(
                    &batteries,
                    start_index,
                    batteries.len() - i,
                )
                .expect("Failed to find max value");

                max_values.push(max_value);
                start_index = next_start_index + 1;
            }

            max_values
        })
        .map(|vals| {
            vals.iter()
                .rev()
                .enumerate()
                .map(|(i, &v)| (v as u64) * 10u64.pow(i as u32))
                .sum::<u64>()
        })
        .sum::<u64>();

    println!("{}", result);
}
