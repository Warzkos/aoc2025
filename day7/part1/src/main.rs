use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(PartialEq)]
enum TachyonManifoldPosition {
    Start,
    Empty,
    Splitter,
}

impl TachyonManifoldPosition {
    fn from_char(c: char) -> Self {
        match c {
            'S' => TachyonManifoldPosition::Start,
            '.' => TachyonManifoldPosition::Empty,
            '^' => TachyonManifoldPosition::Splitter,
            _ => panic!("Invalid character for tachyon manifold position"),
        }
    }
}

fn main() {
    let content = read_to_string("../input.txt").expect("Failed to read input file");
    let start_line = &content.lines().next().unwrap();
    let rest_lines = &content.lines().skip(1).collect::<Vec<&str>>();

    let mut beam_positions: HashSet<i32> = HashSet::new();
    for (i, c) in start_line.chars().enumerate() {
        if TachyonManifoldPosition::from_char(c) == TachyonManifoldPosition::Start {
            beam_positions.insert(i as i32);
        }
    }

    let mut split_count = 0;
    rest_lines.iter().for_each(|line| {
        let mut new_beam_positions: HashSet<i32> = HashSet::new();
        for (i, c) in line.chars().enumerate() {
            let position = TachyonManifoldPosition::from_char(c);
            match position {
                TachyonManifoldPosition::Empty => {
                    if beam_positions.contains(&(i as i32)) {
                        new_beam_positions.insert(i as i32);
                    }
                }
                TachyonManifoldPosition::Splitter => {
                    if beam_positions.contains(&(i as i32)) {
                        new_beam_positions.insert(i as i32 - 1);
                        new_beam_positions.insert(i as i32 + 1);
                        split_count += 1;
                    }
                }
                _ => {}
            }
        }
        beam_positions = new_beam_positions;
    });

    println!("Number of splits: {}", split_count);
}
