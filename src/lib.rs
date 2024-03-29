use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
use std::thread;
#[cfg(target_arch = "wasm32")]
use wasm_thread as thread;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    // Prevent `wasm_bindgen` from autostarting main on all spawned threads
    #[wasm_bindgen(start)]
    pub fn dummy_main() {}

    // Export explicit run function to start main
    #[wasm_bindgen]
    pub fn run() {
        console_log::init().unwrap();
        console_error_panic_hook::set_once();
        crate::run();
    }
}

pub fn run() {
    for _ in 0..4 {
        thread::spawn(|| {
            for i in 0..2 {
                log::info!(
                    "hi number {} from the spawned thread {:?}!",
                    i,
                    thread::current().id()
                );
                thread::sleep(Duration::from_millis(1));
            }
        });
    }

    for i in 0..2 {
        log::info!(
            "hi number {} from the main thread {:?}!",
            i,
            thread::current().id()
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    thread::sleep(Duration::from_secs(1))
}
