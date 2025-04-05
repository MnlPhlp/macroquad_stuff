use std::time::Instant;

use macroquad::{prelude::*, rand};
use macroquad_stuff::{GameState, run_game_loop};

const BALL_SPEED: f32 = 0.3;
const BALL_SIZE: f32 = 0.01;
const PADDLE_SPEED: f32 = 0.5;
const PADDLE_HEIGHT: f32 = 0.1;

const TEXT_MS: u64 = 1000;
const BLINK_MS: u64 = 100;

struct State {
    text: String,
    text_timer: Instant,
    score_l: u32,
    score_r: u32,
    ball: Vec2,
    ball_speed: Vec2,
    paddle_l: f32,
    paddle_r: f32,
    blink_l: Instant,
    blink_r: Instant,
}
impl Default for State {
    fn default() -> Self {
        Self {
            text: String::new(),
            text_timer: Instant::now(),
            score_l: 0,
            score_r: 0,
            ball: Vec2::new(0.5, 0.5),
            ball_speed: get_random_speed(),
            paddle_l: 0.5 - PADDLE_HEIGHT / 2.0,
            paddle_r: 0.5 - PADDLE_HEIGHT / 2.0,
            blink_l: Instant::now(),
            blink_r: Instant::now(),
        }
    }
}
impl GameState for State {
    fn bg_color(&self) -> Color {
        BLACK
    }
    fn update(&mut self, delta_time: f32) {
        check_points(self);
        update_positions(self, delta_time);
    }
    fn draw(&self) {
        let now = Instant::now();
        let w = screen_width();
        let h = screen_height();

        draw_text(&format!("Left: {}", self.score_l), 10.0, 20.0, 20.0, WHITE);
        draw_text(
            &format!("Right: {}", self.score_r),
            w - 100.0,
            20.0,
            20.0,
            WHITE,
        );

        if self.text_timer > now {
            draw_text(&self.text, w / 2.0 - 100.0, h / 2.0, 20.0, WHITE);
        } else {
            draw_circle(self.ball.x * w, self.ball.y * h, BALL_SIZE * h, WHITE);
        }

        let paddle_height = PADDLE_HEIGHT * h;
        let left_color = if self.blink_l > now { GREEN } else { WHITE };
        draw_rectangle(0.0, self.paddle_l * h, 10.0, paddle_height, left_color);
        let right_color = if self.blink_r > now { GREEN } else { WHITE };
        draw_rectangle(
            w - 10.0,
            self.paddle_r * h,
            10.0,
            paddle_height,
            right_color,
        );
    }
    fn reset(&mut self) {
        *self = State::default();
    }
    fn is_paused(&self) -> bool {
        self.text_timer > Instant::now()
    }
}

#[macroquad::main("Pong")]
async fn main() {
    run_game_loop(State::default()).await;
}

fn update_positions(state: &mut State, delta: f32) {
    if state.ball.y < 0.0 || state.ball.y > 1.0 {
        state.ball_speed.y *= -1.0;
    }
    state.ball += state.ball_speed * delta;
    if is_key_down(KeyCode::W) {
        state.paddle_l -= PADDLE_SPEED * delta;
    }
    if is_key_down(KeyCode::S) {
        state.paddle_l += PADDLE_SPEED * delta;
    }
    state.paddle_l = clamp(state.paddle_l, 0.0, 1.0 - PADDLE_HEIGHT);
    if is_key_down(KeyCode::Up) {
        state.paddle_r -= PADDLE_SPEED * delta;
    }
    if is_key_down(KeyCode::Down) {
        state.paddle_r += PADDLE_SPEED * delta;
    }
    state.paddle_r = clamp(state.paddle_r, 0.0, 1.0 - PADDLE_HEIGHT);
}

fn check_points(state: &mut State) {
    if state.ball.x < 0.0 {
        if state.ball.y > state.paddle_l && state.ball.y < state.paddle_l + PADDLE_HEIGHT {
            state.ball_speed.x *= -1.0;
            state.blink_l = Instant::now() + std::time::Duration::from_millis(BLINK_MS);
        } else {
            // right player scores
            score(state, false);
        }
    } else if state.ball.x > 1.0 {
        if state.ball.y > state.paddle_r && state.ball.y < state.paddle_r + PADDLE_HEIGHT {
            state.ball_speed.x *= -1.0;
            state.blink_r = Instant::now() + std::time::Duration::from_millis(BLINK_MS);
        } else {
            // left player scores
            score(state, true);
        }
    }
}

fn score(state: &mut State, left: bool) {
    if left {
        state.score_l += 1;
        state.text = "Left player scores!".to_string();
    } else {
        state.score_r += 1;
        state.text = "Right player scores!".to_string();
    }
    state.text_timer = Instant::now() + std::time::Duration::from_millis(TEXT_MS);
    state.ball = Vec2::new(0.5, 0.5);
    state.ball_speed = get_random_speed();
    state.paddle_l = 0.5 - PADDLE_HEIGHT / 2.0;
    state.paddle_r = 0.5 - PADDLE_HEIGHT / 2.0;
}

fn get_random_speed() -> Vec2 {
    let x = rand::gen_range(-0.5, 0.5);
    let y = rand::gen_range(-0.5, 0.5);
    Vec2::new(x, y).normalize() * BALL_SPEED
}
