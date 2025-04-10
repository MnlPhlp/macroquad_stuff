#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use macroquad::prelude::*;
use macroquad_stuff::{Context, GameState};

enum GridMode {
    Lines,
    Shaded,
    None,
}

const START_SIZE: usize = 40;
struct State {
    rows: usize,
    cols: usize,
    cells: Vec<bool>,
    next_cells: Vec<bool>,
    reset_cells: Vec<bool>,
    step_time: f32,
    last_step_time: f32,
    time_elapsed: f32,
    drawing_mode: bool,
    grid_mode: GridMode,
    paused: bool,
}
impl Default for State {
    fn default() -> Self {
        let mut state = Self {
            rows: START_SIZE,
            cols: START_SIZE,
            cells: vec![false; START_SIZE * START_SIZE],
            next_cells: vec![false; START_SIZE * START_SIZE],
            reset_cells: vec![],
            step_time: 0.5,
            last_step_time: 0.5,
            time_elapsed: 0.0,
            drawing_mode: false,
            grid_mode: GridMode::Lines,
            paused: false,
        };
        state.spawn_glider();
        state.reset_cells = state.cells.clone();
        state
    }
}

impl GameState for State {
    fn bg_color(&self) -> Color {
        BLACK
    }
    async fn update(ctx: &mut Context<Self>, delta_time: f32) {
        handle_input(ctx).await;
        ctx.state.time_elapsed += delta_time;
        if ctx.state.drawing_mode
            || ctx.state.time_elapsed < ctx.state.step_time
            || ctx.state.paused
        {
            return;
        }
        ctx.state.last_step_time = ctx.state.time_elapsed;
        ctx.state.time_elapsed = 0.0;
        ctx.state.update_cells();
    }
    fn draw(&self) {
        let (border_x, border_y) = get_borders();
        let w = screen_width() - border_x * 2.0;
        let h = screen_height() - border_y * 2.0;

        let text_height = 30.0;
        draw_text(
            "Space: draw, R: reset,  Up/Down: delay, Left/Right: size, G: grid mode",
            5.0,
            text_height + 5.0,
            text_height,
            WHITE,
        );
        let text = if self.drawing_mode {
            "drawing mode. press Space to continue".to_string()
        } else if self.paused {
            "Paused, P to continue, S to step".to_string()
        } else {
            format!(
                "Delay Target: {:.1}s, Delay: {:.2}s; P to Pause",
                self.step_time, self.last_step_time
            )
        };
        draw_text(
            text.as_str(),
            5.0,
            text_height * 2.0 + 5.0,
            text_height,
            WHITE,
        );

        let line_thickness = if matches!(self.grid_mode, GridMode::Lines) {
            2.0
        } else {
            0.0
        };
        let cw = w / self.cols as f32;
        let ch = h / self.rows as f32;
        let offset = line_thickness / 2.0;

        for row in 0..self.rows {
            let y = row as f32 * ch + border_y;
            if matches!(self.grid_mode, GridMode::Lines) && row > 0 {
                draw_line(0.0 + border_x, y, w + border_x, y, line_thickness, WHITE);
            }
            for col in 0..self.cols {
                let x = col as f32 * cw + border_x;
                if matches!(self.grid_mode, GridMode::Lines) && col > 0 && row == 0 {
                    draw_line(x, 0.0 + border_y, x, h + border_y, line_thickness, WHITE);
                }
                let cell_color = if self.cells[self.get_index(col, row)] {
                    GREEN
                } else if matches!(self.grid_mode, GridMode::Shaded) {
                    if row % 2 == col % 2 { GRAY } else { DARKGRAY }
                } else {
                    WHITE
                };
                if cell_color != WHITE {
                    draw_rectangle(
                        x + offset,
                        y + offset,
                        cw - offset * 2.0,
                        ch - offset * 2.0,
                        cell_color,
                    );
                }
            }
        }
        draw_rectangle_lines(border_x, border_y, w, h, 4.0, WHITE);
    }
    fn is_paused(&self) -> bool {
        false
    }

    fn reset(&mut self) {
        self.cells.clone_from(&self.reset_cells);
        self.time_elapsed = 0.0;
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
        std::mem::swap(&mut self.reset_cells, &mut self.next_cells);
        self.reset_cells.resize(rows * cols, false);
        self.reset_cells.fill(false);
        // map cells to new indices
        for row in 0..rows {
            for col in 0..cols {
                if col >= self.cols || row >= self.rows {
                    continue;
                }
                let old_index = row * self.cols + col;
                let new_index = row * cols + col;
                if new_index < self.cells.len() {
                    self.reset_cells[new_index] = self.next_cells[old_index];
                }
            }
        }
        self.next_cells.resize(rows * cols, false);
        self.rows = rows;
        self.cols = cols;
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x * self.rows + y
    }
}

async fn handle_input(ctx: &mut Context<State>) {
    if is_key_pressed(KeyCode::Space) {
        if ctx.state.drawing_mode {
            // save drawing for reset
            ctx.state.reset_cells.clone_from(&ctx.state.cells);
            info!("Saved drawing");
        } else {
            ctx.state.cells.fill(false);
        }
        ctx.state.drawing_mode = !ctx.state.drawing_mode;
    }
    if is_key_pressed(KeyCode::R) {
        ctx.state.reset();
    }
    if is_key_pressed(KeyCode::Up) {
        ctx.state.step_time = (ctx.state.step_time + 0.1).min(2.0);
    }
    if is_key_pressed(KeyCode::Down) {
        ctx.state.step_time = (ctx.state.step_time - 0.1).max(0.0);
    }
    if ctx.is_key_pressed_loop(KeyCode::Left) {
        ctx.state.resize(ctx.state.rows - 1, ctx.state.cols - 1);
    }
    if ctx.is_key_pressed_loop(KeyCode::Right) {
        ctx.state.resize(ctx.state.rows + 1, ctx.state.cols + 1);
    }
    if is_key_pressed(KeyCode::G) {
        ctx.state.grid_mode = match ctx.state.grid_mode {
            GridMode::Lines => GridMode::Shaded,
            GridMode::Shaded => GridMode::None,
            GridMode::None => GridMode::Lines,
        };
    }
    if !ctx.state.drawing_mode && is_key_pressed(KeyCode::P) {
        ctx.state.paused = !ctx.state.paused;
    }
    if ctx.state.paused && ctx.is_key_pressed_loop(KeyCode::S) {
        // do a single step
        ctx.state.update_cells();
    }
    if ctx.state.drawing_mode {
        if is_key_pressed(KeyCode::O) {
            load_from_file(ctx).await;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let (border_x, border_y) = get_borders();
            let mouse_pos = {
                let (x, y) = mouse_position();
                (x - border_x, y - border_y)
            };

            let w = screen_width() - border_x * 2.0;
            let h = screen_height() - border_y * 2.0;
            let cw = w / ctx.state.cols as f32;
            let ch = h / ctx.state.rows as f32;
            let x = (mouse_pos.0 / cw).floor() as usize;
            let y = (mouse_pos.1 / ch).floor() as usize;
            let index = ctx.state.get_index(x, y);
            if index < ctx.state.cells.len() {
                ctx.state.cells[index] = !ctx.state.cells[index];
            }
        }
    }
}

async fn load_from_file(ctx: &mut Context<State>) {
    let text = ctx.open_file().await;
    for line in text.lines() {
        if line.starts_with("//") || line.is_empty() {
            continue;
        }
        let (Ok(x), Ok(y)) = ({
            let (x, y) = line.split_once(' ').unwrap();
            let x = x.parse();
            let y = y.parse();
            (x, y)
        }) else {
            println!("Invalid line: {line}");
            continue;
        };
        let index = ctx.state.get_index(x, y);
        if index < ctx.state.cells.len() {
            ctx.state.cells[index] = true;
        }
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
