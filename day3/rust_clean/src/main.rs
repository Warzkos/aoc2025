use std::fs::read_to_string;

const INPUT_FILE: &str = "../input.txt";

// const BATTERIES_TO_FIND: usize = 2;
const BATTERIES_TO_FIND: usize = 12;

fn find_max_values(mut data: &[u32], count: usize) -> Vec<u32> {
    let mut result = Vec::with_capacity(count);

    for i in (0..count).rev() {
        let end = data.len() - i;
        let (idx, &max) = data[..end]
            .iter()
            .enumerate()
            .max_by_key(|&(_, v)| v)
            .expect("No max found");

        result.push(max);
        data = &data[idx + 1..];
    }

    result
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
            find_max_values(&batteries, BATTERIES_TO_FIND)
        })
        .map(|vals| {
            vals.iter()
                .rev()
                .enumerate()
                .map(|(i, &v)| (v as u64) * 10u64.pow(i as u32))
                .sum::<u64>()
        })
        .sum();

    println!("{}", result);
}
