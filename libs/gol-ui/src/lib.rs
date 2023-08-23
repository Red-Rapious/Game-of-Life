use speedy2d::color::Color;
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

use lib_game_of_life::{Board, Cell};
use std::{thread, time};

pub fn run() {
    let (width, height) = (240, 180);
    let square_side = 10.0;
    let window = Window::new_centered("Game of Life", (width * square_side as u32, height * square_side as u32)).unwrap();
    window.run_loop(MyWindowHandler::new(
        width as usize,
        height as usize,
        square_side,
    ));
}

struct MyWindowHandler {
    board: Board,
    square_side: f32,
}

impl MyWindowHandler {
    pub fn new(width: usize, height: usize, square_side: f32) -> Self {
        Self {
            board: Board::random(width, height),
            square_side,
        }
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<()>,
        _info: speedy2d::window::WindowStartupInfo,
    ) {
        helper.set_resizable(false);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::GRAY);

        for x in 0..self.board.width() {
            for y in 0..self.board.height() {
                graphics.draw_rectangle(
                    Rectangle::from_tuples(
                        (
                            x as f32 * self.square_side,
                            y as f32 * self.square_side,
                        ),
                        (
                            (x as f32 + 1.0) * self.square_side,
                            (y as f32 + 1.0) * self.square_side,
                        ),
                    ),
                    match self.board.grid()[y][x] {
                        Cell::Black => Color::BLACK,
                        Cell::White => Color::WHITE,
                    },
                );
            }
        }

        self.board.step();

        thread::sleep(time::Duration::from_millis(100));
        helper.request_redraw();
    }
}
