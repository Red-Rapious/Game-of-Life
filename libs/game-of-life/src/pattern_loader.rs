use std::io::BufRead;
use std::{fs::File, io::BufReader};

use crate::Cell;

pub struct Pattern {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
}

impl Pattern {
    pub fn from_grid(grid: Vec<Vec<Cell>>) -> Self {
        assert!(!grid.is_empty());

        Self {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        }
    }

    pub fn load(filepath: String) -> Self {
        let file = File::open(filepath).unwrap();
        let buffer = BufReader::new(file);

        let mut grid = vec![];

        for line in buffer.lines() {
            let line = line.unwrap();
            if line.chars().take(1).collect::<Vec<char>>()[0] == '!' {
                continue;
            }

            grid.push(vec![]);
            let index = grid.len() - 1;
            for cell in line.chars() {
                grid[index].push(match cell {
                    '.' => Cell::Black,
                    'O' => Cell::White,
                    ' ' => { continue; },
                    _ => panic!("Unsupported cell '{}'", cell),
                });
            }
        }

        Self::from_grid(grid)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }
}
