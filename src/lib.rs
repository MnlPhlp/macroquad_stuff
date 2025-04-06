use macroquad::prelude::*;

pub const TEXT_HEIGHT: f32 = 0.05;

async fn run_game_loop(mut state: impl GameState) {
    let mut paused = false;
    let mut fps = false;
    state.reset();

    loop {
        let w = screen_width();
        let h = screen_height();
        let th = TEXT_HEIGHT * h;

        clear_background(state.bg_color());

        if is_key_pressed(KeyCode::Escape) {
            paused = !paused;
        }
        if !paused {
            state.update(get_frame_time());
        }
        state.draw();

        if fps {
            let fps = get_fps();
            draw_text_top_right(&format!("FPS: {fps}"), w - 5.0, 5.0, th / 2.0, WHITE);
        }

        if paused {
            draw_rectangle(0.0, 0.0, w, h, Color::new(0.0, 0.0, 0.0, 0.6));
            draw_text_centered("Paused", w / 2.0, h / 2.0 - th * 1.2, th, WHITE);
            draw_text_centered("Press ESC to continue", w / 2.0, h / 2.0, th, WHITE);
            draw_text_centered("Press R to reset", w / 2.0, h / 2.0 + th * 1.2, th, WHITE);
            draw_text_centered(
                "Press F to enable FPS",
                w / 2.0,
                h / 2.0 + th * 2.4,
                th,
                WHITE,
            );
            if is_key_pressed(KeyCode::R) {
                state.reset();
                paused = false;
            }
            if is_key_pressed(KeyCode::F) {
                fps = !fps;
            }
        }

        next_frame().await;
    }
}

pub trait GameState: Default {
    fn bg_color(&self) -> Color;
    fn update(&mut self, delta_time: f32);
    fn draw(&self);
    fn reset(&mut self);
    fn is_paused(&self) -> bool;
    #[must_use]
    fn run_game_loop() -> impl Future<Output = ()> {
        run_game_loop(Self::default())
    }
}

pub fn draw_text_centered(text: &str, x: f32, y: f32, size: f32, color: Color) {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let font_size = size as u16;
    let dimensions = measure_text(text, None, font_size, 1.0);
    draw_text(
        text,
        x - dimensions.width / 2.0,
        y + dimensions.height / 2.0,
        size,
        color,
    );
}

/// Draw text with the given position being at the top right corner of the text
pub fn draw_text_top_right(text: &str, w: f32, h: f32, size: f32, white: Color) {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let font_size = size as u16;
    let dimensions = measure_text(text, None, font_size, 1.0);
    draw_text(
        text,
        w - dimensions.width,
        h + dimensions.height,
        size,
        white,
    );
}

const KEY_LOOP_DELAY: f32 = 0.5;
/// Returns true on the first press of the key and when the key is pressed for a minimum time
pub fn is_key_pressed_loop(key: KeyCode, elapsed: &mut f32, delta: f32) -> bool {
    if is_key_down(key) {
        if elapsed.is_nan() {
            *elapsed = 0.0;
            return true;
        }
        *elapsed += delta;
        if *elapsed > KEY_LOOP_DELAY {
            return true;
        }
    } else {
        *elapsed = f32::NAN;
    }
    false
}
