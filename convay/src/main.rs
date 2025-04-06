#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use macroquad::prelude::*;
use macroquad_stuff::{GameState, is_key_pressed_loop};

struct State {
    rows: usize,
    cols: usize,
    line_thickness: f32,
    cells: Vec<bool>,
    next_cells: Vec<bool>,
    step_time: f32,
    time_elapsed: f32,
    drawing_mode: bool,
    left_down: f32,
    right_down: f32,
}
impl Default for State {
    fn default() -> Self {
        Self {
            rows: 10,
            cols: 10,
            line_thickness: 2.0,
            cells: vec![false; 100],
            next_cells: vec![false; 100],
            step_time: 1.0,
            time_elapsed: 0.0,
            drawing_mode: false,
            left_down: 0.0,
            right_down: 0.0,
        }
    }
}

impl GameState for State {
    fn bg_color(&self) -> Color {
        BLACK
    }
    fn update(&mut self, delta_time: f32) {
        self.handle_input(delta_time);
        self.time_elapsed += delta_time;
        if self.drawing_mode || self.time_elapsed < self.step_time {
            return;
        }
        self.time_elapsed = 0.0;
        self.update_cells();
    }
    fn draw(&self) {
        let (border_x, border_y) = get_borders();
        let w = screen_width() - border_x * 2.0;
        let h = screen_height() - border_y * 2.0;

        let text_height = 30.0;
        let text = if self.drawing_mode {
            "Paused for drawing. press Space to Continue"
        } else {
            "Space: pause/draw, R: reset, Up/Down: delay up/down, Left/Right: change size"
        };
        draw_text(text, 5.0, text_height + 5.0, text_height, WHITE);
        draw_text(
            format!("Delay: {:.1}s", self.step_time).as_str(),
            5.0,
            text_height * 2.0 + 5.0,
            text_height,
            WHITE,
        );

        let cw = w / self.cols as f32;
        let ch = h / self.rows as f32;
        let offset = self.line_thickness / 2.0;

        for row in 0..self.rows {
            let y = row as f32 * ch + border_y;
            if row > 0 {
                draw_line(
                    0.0 + border_x,
                    y,
                    w + border_x,
                    y,
                    self.line_thickness,
                    WHITE,
                );
            }
            for col in 0..self.cols {
                let x = col as f32 * cw + border_x;
                if col > 0 && row == 0 {
                    draw_line(
                        x,
                        0.0 + border_y,
                        x,
                        h + border_y,
                        self.line_thickness,
                        WHITE,
                    );
                }
                if self.cells[row * self.cols + col] {
                    draw_rectangle(
                        x + offset,
                        y + offset,
                        cw - offset * 2.0,
                        ch - offset * 2.0,
                        GREEN,
                    );
                };
            }
        }
        draw_rectangle_lines(border_x, border_y, w, h, self.line_thickness * 2., WHITE);
    }
    fn reset(&mut self) {
        self.cells.fill(false);
        self.drawing_mode = false;
        self.time_elapsed = 0.0;
        self.spawn_glider();
    }
    fn is_paused(&self) -> bool {
        false
    }
}

impl State {
    fn update_cells(&mut self) {
        // Rules:
        // A cell keeps its state if it has two neighbors.
        // A cell becomes active if it has three neighbors.
        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut neighbors = 0;
                for n_row in row.saturating_sub(1)..=(row + 1).min(self.rows - 1) {
                    for n_col in col.saturating_sub(1)..=(col + 1).min(self.cols - 1) {
                        // skip self
                        if n_col == col && n_row == row {
                            continue;
                        }
                        // check neighbor
                        if self.cells[n_row * self.cols + n_col] {
                            neighbors += 1;
                        }
                    }
                }
                // apply rules
                if neighbors == 2 {
                    // A cell keeps its state if it has two neighbors.
                    self.next_cells[row * self.cols + col] = self.cells[row * self.cols + col];
                } else if neighbors == 3 {
                    // A cell becomes active if it has three neighbors.
                    self.next_cells[row * self.cols + col] = true;
                } else {
                    self.next_cells[row * self.cols + col] = false;
                }
            }
        }
        // swap cells
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    fn handle_input(&mut self, delta_time: f32) {
        if is_key_pressed(KeyCode::Space) {
            self.drawing_mode = !self.drawing_mode;
        }
        if is_key_pressed(KeyCode::R) {
            self.reset();
        }
        if is_key_pressed(KeyCode::Up) {
            self.step_time = (self.step_time + 0.1).min(2.0);
        }
        if is_key_pressed(KeyCode::Down) {
            self.step_time = (self.step_time - 0.1).max(0.0);
        }
        if is_key_pressed_loop(KeyCode::Left, &mut self.left_down, delta_time) {
            self.resize(self.rows - 1, self.cols - 1);
        }
        if is_key_pressed_loop(KeyCode::Right, &mut self.right_down, delta_time) {
            self.resize(self.rows + 1, self.cols + 1);
        }
        if self.drawing_mode && is_mouse_button_pressed(MouseButton::Left) {
            let (border_x, border_y) = get_borders();
            let mouse_pos = {
                let (x, y) = mouse_position();
                (x - border_x, y - border_y)
            };

            let w = screen_width() - border_x * 2.0;
            let h = screen_height() - border_y * 2.0;
            let cw = w / self.cols as f32;
            let ch = h / self.rows as f32;
            let col = (mouse_pos.0 / cw).floor() as usize;
            let row = (mouse_pos.1 / ch).floor() as usize;
            if col < self.cols && row < self.rows {
                self.cells[row * self.cols + col] = !self.cells[row * self.cols + col];
            }
        }
    }

    fn spawn_glider(&mut self) {
        // spawn glider in top left corner
        for (x, y) in [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)] {
            let idx = y * self.cols + x;
            if idx >= self.cells.len() {
                continue;
            }
            self.cells[idx] = true;
        }
    }

    fn resize(&mut self, rows: usize, cols: usize) {
        if rows < 1 || cols < 1 {
            return;
        }
        std::mem::swap(&mut self.cells, &mut self.next_cells);
        self.cells.resize(rows * cols, false);
        self.cells.fill(false);
        // map cells to new indices
        for row in 0..rows {
            for col in 0..cols {
                if col >= self.cols || row >= self.rows {
                    continue;
                }
                let old_index = row * self.cols + col;
                let new_index = row * cols + col;
                if new_index < self.cells.len() {
                    self.cells[new_index] = self.next_cells[old_index];
                }
            }
        }
        self.next_cells.resize(rows * cols, false);
        self.rows = rows;
        self.cols = cols;
    }
}

fn get_borders() -> (f32, f32) {
    const MIN_BORDER: f32 = 100.0;
    // get borders to keep the grid square
    let w = screen_width();
    let h = screen_height();
    if w > h {
        let border = (w - h) / 2.0 + MIN_BORDER;
        (border, MIN_BORDER)
    } else {
        let border = (h - w) / 2.0 + MIN_BORDER;
        (MIN_BORDER, border)
    }
}

#[macroquad::main("Convay")]
async fn main() {
    State::run_game_loop().await;
}
