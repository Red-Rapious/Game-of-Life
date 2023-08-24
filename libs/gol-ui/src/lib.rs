use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rectangle;
use speedy2d::window::VirtualKeyCode;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;
use std::{thread, time};

use lib_game_of_life::{Board, Cell};

/// Minimum time between two frames of the simulation, in milliseconds
const DELTA: u64 = 100;

pub fn run(width: u32, height: u32, square_side: f32) {
    assert_ne!(square_side, 0.0);
    assert_ne!(width, 0);
    assert_ne!(height, 0);

    let window = Window::new_centered(
        "Game of Life",
        (width * square_side as u32, height * square_side as u32),
    )
    .unwrap();
    window.run_loop(GOLSimulation::new(
        width as usize,
        height as usize,
        square_side,
    ));
}

struct GOLSimulation {
    board: Board,
    square_side: f32,
    paused: bool,
    mouse_position: Vec2,
}

impl GOLSimulation {
    pub fn new(width: usize, height: usize, square_side: f32) -> Self {
        Self {
            board: Board::empty(width, height),
            square_side,
            paused: false,
            mouse_position: Vec2::new(0.0, 0.0),
        }
    }

    fn update_title(&self, helper: &mut WindowHelper<()>) {
        helper.set_title(format!(
            "Game of Life{}",
            match self.paused {
                true => "- Paused",
                false => "",
            }
        ));
    }
}

impl WindowHandler for GOLSimulation {
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<()>,
        _info: speedy2d::window::WindowStartupInfo,
    ) {
        helper.set_resizable(false);
        helper.set_cursor_visible(self.paused);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::GRAY);

        for x in 0..self.board.width() {
            for y in 0..self.board.height() {
                graphics.draw_rectangle(
                    Rectangle::from_tuples(
                        (x as f32 * self.square_side, y as f32 * self.square_side),
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

        if !self.paused {
            self.board.step();
        }

        thread::sleep(time::Duration::from_millis(DELTA));
        if !self.paused {
            helper.request_redraw();
        }
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        _: speedy2d::window::KeyScancode,
    ) {
        if virtual_key_code == Some(VirtualKeyCode::Space) {
            self.paused = !self.paused;
            if !self.paused {
                helper.request_redraw();
            }
            self.update_title(helper);
            helper.set_cursor_visible(self.paused);
        }
    }

    fn on_mouse_button_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        _: speedy2d::window::MouseButton,
    ) {
        if self.paused {
            let x = (self.mouse_position.x / self.square_side) as usize;
            let y = (self.mouse_position.y / self.square_side) as usize;

            self.board.invert_cell(x, y);
            helper.request_redraw();
        }
    }

    fn on_mouse_move(&mut self, _: &mut WindowHelper<()>, position: Vec2) {
        self.mouse_position = position;
    }
}
