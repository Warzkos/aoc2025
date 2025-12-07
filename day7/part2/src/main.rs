use std::collections::HashMap;
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

    let mut beam_positions: HashMap<i32, usize> = HashMap::new();
    for (i, c) in start_line.chars().enumerate() {
        if TachyonManifoldPosition::from_char(c) == TachyonManifoldPosition::Start {
            beam_positions.insert(i as i32, 1);
        }
    }

    for line in rest_lines.iter() {
        let mut new_beam_positions: HashMap<i32, usize> = HashMap::new();

        if !line
            .chars()
            .all(|c| TachyonManifoldPosition::from_char(c) == TachyonManifoldPosition::Empty)
        {
            for (j, c) in line.chars().enumerate() {
                let position = TachyonManifoldPosition::from_char(c);
                match position {
                    TachyonManifoldPosition::Empty => {
                        if let Some(&count) = beam_positions.get(&(j as i32)) {
                            *new_beam_positions.entry(j as i32).or_insert(0) += count;
                        }
                    }
                    TachyonManifoldPosition::Splitter => {
                        if let Some(&count) = beam_positions.get(&(j as i32)) {
                            *new_beam_positions.entry(j as i32 - 1).or_insert(0) += count;
                            *new_beam_positions.entry(j as i32 + 1).or_insert(0) += count;
                        }
                    }
                    _ => {}
                }
            }
            beam_positions = new_beam_positions;
        }
    }

    let total_beams: usize = beam_positions.values().sum();
    println!("Beam timelines: {}", total_beams);
}
