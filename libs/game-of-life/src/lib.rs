use rand::Rng;

pub mod pattern_loader;

pub struct Board {
    grid: Vec<Vec<Cell>>,
}

impl Board {
    pub fn from_grid(grid: Vec<Vec<Cell>>) -> Self {
        Self { grid }
    }

    pub fn empty(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0);

        let grid = (0..height).map(|_| vec![Cell::Black; width]).collect();

        Self { grid }
    }

    pub fn random(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0);

        let mut rng = rand::thread_rng();
        let grid = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| {
                        if rng.gen_bool(0.5) {
                            Cell::White
                        } else {
                            Cell::Black
                        }
                    })
                    .collect()
            })
            .collect();

        Self { grid }
    }

    pub fn grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    fn cell_white_neighbours(&self, x: usize, y: usize) -> u8 {
        assert!(x < self.width());
        assert!(y < self.height());

        let mut white_neighbours = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

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
        self.grid = (0..self.height())
            .map(|y| {
                (0..self.width())
                    .map(|x| match self.cell_white_neighbours(x, y) {
                        3 => Cell::White,     // birth or stay alive
                        2 => self.grid[y][x], // stay dead or stay alive
                        _ => Cell::Black,     // no birth, or die of under/over-population
                    })
                    .collect()
            })
            .collect();
    }

    pub fn set_cell(&mut self, x: usize, y: usize, new_cell: Cell) {
        assert!(x < self.width());
        assert!(y < self.height());

        self.grid[y][x] = new_cell;
    }

    pub fn invert_cell(&mut self, x: usize, y: usize) {
        assert!(x < self.width());
        assert!(y < self.height());

        self.grid[y][x].invert();
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Black,
    White,
}

impl Cell {
    pub fn invert(&mut self) {
        use Cell::*;
        *self = match *self {
            Black => White,
            White => Black,
        };
    }
}

#[cfg(test)]
mod game_of_life_tests {
    use super::*;

    #[test]
    fn same_dimensions() {
        let mut board = Board::random(3, 2);
        board.step();

        assert_eq!(board.width(), 3);
        assert_eq!(board.height(), 2);
    }

    #[test]
    fn alternate_3_neighbours() {
        use Cell::*;

        let grid = vec![
            vec![Black, Black, Black, Black, Black],
            vec![Black, Black, White, Black, Black],
            vec![Black, Black, White, Black, Black],
            vec![Black, Black, White, Black, Black],
            vec![Black, Black, Black, Black, Black],
        ];

        let board = Board::from_grid(grid);

        assert_eq!(board.cell_white_neighbours(1, 2), 3);
        assert_eq!(board.cell_white_neighbours(3, 2), 3);
        assert_eq!(board.cell_white_neighbours(2, 2), 2);
    }

    #[test]
    fn alternate_3_step() {
        use Cell::*;

        let grid1 = vec![
            vec![Black, Black, Black, Black, Black],
            vec![Black, Black, White, Black, Black],
            vec![Black, Black, White, Black, Black],
            vec![Black, Black, White, Black, Black],
            vec![Black, Black, Black, Black, Black],
        ];
        let grid2 = vec![
            vec![Black, Black, Black, Black, Black],
            vec![Black, Black, Black, Black, Black],
            vec![Black, White, White, White, Black],
            vec![Black, Black, Black, Black, Black],
            vec![Black, Black, Black, Black, Black],
        ];

        let mut board = Board::from_grid(grid1.clone());
        board.step();
        assert_eq!(board.grid, grid2);

        board.step();
        assert_eq!(board.grid, grid1);
    }

    #[test]
    fn invert() {
        let mut board = Board::empty(2, 1);
        board.invert_cell(1, 0);

        assert_eq!(board.grid, vec![vec![Cell::Black, Cell::White]]);
    }
}
