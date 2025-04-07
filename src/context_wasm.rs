use crate::{Context, GameState};
use macroquad::prelude::*;
use sapp_jsutils::JsObject;
use std::sync::{Mutex, mpsc::Sender};

static STRING_RESPONSE: Mutex<Option<Sender<String>>> = Mutex::new(None);

impl<S: GameState> Context<S> {
    #[must_use]
    pub async fn open_file(&self) -> String {
        info!("Opening file");
        let (tx, rx) = std::sync::mpsc::channel();
        STRING_RESPONSE.lock().unwrap().replace(tx);
        unsafe { open_file_js() };
        loop {
            match rx.try_recv() {
                Ok(s) => break s,
                Err(_) => {
                    self.state.draw();
                    next_frame().await;
                }
            }
        }
    }
}

unsafe extern "C" {
    fn open_file_js() -> JsObject;
}

#[unsafe(no_mangle)]
pub extern "C" fn string_response(js_obj: JsObject) {
    let mut lock = STRING_RESPONSE.lock().unwrap();
    let Some(sender) = lock.take() else {
        // This should never happen, but if it does, we just ignore the response
        warn!("Received a response, but no sender was set");
        return;
    };
    let mut text = String::new();
    js_obj.to_string(&mut text);
    sender.send(text).unwrap();
}
