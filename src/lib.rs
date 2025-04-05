use macroquad::prelude::*;

pub const TEXT_HEIGHT: f32 = 0.05;

pub async fn run_game_loop(mut state: impl GameState) {
    let mut paused = false;
    loop {
        let w = screen_width();
        let h = screen_height();
        clear_background(state.bg_color());

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if paused {
            draw_text("Paused", w / 2.0 - 50.0, h / 2.0, h * TEXT_HEIGHT, WHITE);
        } else {
            state.update(get_frame_time());
        }
        state.draw();

        next_frame().await;
    }
}

pub trait GameState {
    fn bg_color(&self) -> Color;
    fn update(&mut self, delta_time: f32);
    fn draw(&self);
    fn reset(&mut self);
    fn is_paused(&self) -> bool;
}
