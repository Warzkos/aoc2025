use std::fs::read_to_string;

fn create_invalid_id(n: i64) -> i64 {
    return n
        .to_string()
        .repeat(2)
        .parse::<i64>()
        .expect("Failed to create invalid ID");
}

fn find_invalid_ids(range: Vec<&str>) -> i64 {
    let range_start: i64 = range[0]
        .parse::<i64>()
        .expect("Failed to parse start of range");
    let range_end: i64 = range[1]
        .parse::<i64>()
        .expect("Failed to parse end of range");
    let range_start_digits_count: i32 = range[0].len() as i32;

    let mut sub_range_start = range_start / 10_i64.pow((range_start_digits_count) as u32);

    let mut invalid_ids_sum: i64 = 0;
    loop {
        let invalid_id = create_invalid_id(sub_range_start);
        if invalid_id >= range_start && invalid_id <= range_end {
            invalid_ids_sum += invalid_id;
        }
        if invalid_id > range_end {
            break;
        }
        sub_range_start += 1;
    }

    invalid_ids_sum
}

fn main() {
    let result: i64 = read_to_string("../input.txt")
        .expect("Failed to read input file")
        .trim_ascii_end()
        .split(",")
        .map(|range: &str| -> i64 { find_invalid_ids(range.splitn(2, '-').collect()) })
        .sum();

    println!("Sum of all invalid IDs: {}", result);
}
