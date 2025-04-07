use macroquad::window::next_frame;

#[cfg(not(target_arch = "wasm32"))]
use pollster::FutureExt;

use crate::{Context, GameState};

impl<S: GameState> Context<S> {
    #[cfg(not(target_arch = "wasm32"))]
    async fn block_on<F, O>(&self, future: F) -> O
    where
        F: std::future::Future<Output = O> + Send + 'static,
        O: Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = future.block_on();
            tx.send(result).unwrap();
        });
        loop {
            if let Ok(s) = rx.try_recv() {
                return s;
            }
            // continue with drawing loop
            self.state.draw();
            next_frame().await;
        }
    }

    #[must_use]
    pub async fn open_file(&self) -> String {
        self.block_on(async move {
            let Some(file) = rfd::AsyncFileDialog::new().pick_file().await else {
                return String::new();
            };
            let data = file.read().await;
            String::from_utf8_lossy(&data).to_string()
        })
        .await
    }
}
