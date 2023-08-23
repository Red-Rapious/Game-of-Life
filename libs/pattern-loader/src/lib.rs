use lib_game_of_life::Cell;

pub struct Pattern {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>
}

impl Pattern {
    pub fn from_grid(grid: Vec<Vec<Cell>>) -> Self {
        assert!(grid.len() != 0);

        Self {
            width: grid[0].len(),
            height: grid.len(),
            grid
        }
    }

    pub fn load(filepath: String) -> Self {
        todo!()
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