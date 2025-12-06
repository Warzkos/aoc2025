use std::fs::read_to_string;

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn from_str(op: &str) -> Option<Self> {
        match op {
            "+" => Some(Operation::Add),
            "*" => Some(Operation::Multiply),
            _ => None,
        }
    }
}

fn apply_operation(op: &Operation, container: &Vec<u64>) -> u64 {
    match op {
        Operation::Add => container.iter().sum(),
        Operation::Multiply => container.iter().product(),
    }
}

fn main() {
    let file_content = read_to_string("../input.txt").expect("Failed to read the file");
    let lines: Vec<Vec<&str>> = file_content
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let op_line = lines.last().unwrap();
    let data_lines = &lines[..lines.len() - 1];

    let mut result = 0;
    for (i, op_str) in op_line.iter().enumerate() {
        let numbers: Vec<u64> = data_lines
            .iter()
            .map(|row| row[i].parse::<u64>().expect("Invalid number"))
            .collect();

        result += apply_operation(
            &Operation::from_str(op_str).expect("Invalid operation"),
            &numbers,
        );
    }

    println!("{}", result);
}
