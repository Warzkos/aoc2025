use std::fs::read_to_string;

const INPUT_FILE: &str = "../input.txt";

struct Cell {
    x: i32,
    y: i32,
    is_paperroll: bool,
}

impl Cell {
    pub fn from_char(ch: char, x: i32, y: i32) -> Self {
        Cell {
            x,
            y,
            is_paperroll: ch == '@',
        }
    }

    pub fn count_neighbors_with_paperroll(&self, grid: &Vec<Vec<Cell>>) -> usize {
        let directions = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        directions
            .iter()
            .filter_map(|(dx, dy)| {
                let nx = self.x + dx;
                let ny = self.y + dy;
                if ny >= 0 && (ny as usize) < grid.len() && nx >= 0 && (nx as usize) < grid[0].len()
                {
                    Some(&grid[ny as usize][nx as usize])
                } else {
                    None
                }
            })
            .filter(|cell| cell.is_paperroll)
            .count()
    }
}

fn main() {
    let mut cell_grid = read_to_string(INPUT_FILE)
        .expect("Failed to read input file")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| Cell::from_char(ch, x as i32, y as i32))
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    loop {
        let mut coords_to_remove: Vec<(usize, usize)> = Vec::new();
        cell_grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| {
                if cell.is_paperroll && cell.count_neighbors_with_paperroll(&cell_grid) < 4 {
                    coords_to_remove.push((x, y));
                }
            });
        });

        let cur_sum = coords_to_remove.len();

        for (x, y) in coords_to_remove {
            cell_grid[y][x].is_paperroll = false;
        }

        if cur_sum == 0 {
            break;
        }
        result += cur_sum;
    }

    println!("Result: {}", result);
}
