pub struct Board {
    grid: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0);

        let grid = (0..height).map(|_| vec![Cell::Black; width]).collect();

        Self { grid }
    }

    pub fn grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn cell_white_neighbours(&self, x: usize, y: usize) -> u8 {
        assert!(x < self.width());
        assert!(y < self.height());

        let mut white_neighbours = 0;

        for dx in -1..=1 {
            for dy in [-1, 1] {
                if 0 <= x as i32 + dx
                    && x as i32 + dx < self.width() as i32
                    && 0 <= y as i32 + dy
                    && y as i32 + dy < self.height() as i32
                    && self.grid[(y as i32 + dy) as usize][(x as i32 + dx) as usize] == Cell::White
                {
                    white_neighbours += 1;
                }
            }
        }

        white_neighbours
    }

    pub fn step(&mut self) {
        self.grid = (0..self.width())
            .map(|x| {
                (0..self.height())
                    .map(|y| match self.cell_white_neighbours(x, y) {
                        3 => Cell::White,     // birth or stay alive
                        2 => self.grid[y][x], // stay dead or stay alive
                        _ => Cell::Black,     // no birth, or die of under/over-population
                    })
                    .collect()
            })
            .collect();
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Black,
    White,
}
