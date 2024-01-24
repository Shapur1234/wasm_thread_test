use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
use std::thread;
#[cfg(target_arch = "wasm32")]
use wasm_thread as thread;

fn run() {
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    for _ in 0..2 {
        thread::spawn(|| {
            for i in 1..3 {
                log::info!(
                    "hi number {} from the spawned thread {:?}!",
                    i,
                    thread::current().id()
                );
                thread::sleep(Duration::from_millis(1));
            }
        });
    }

    for i in 1..3 {
        log::info!(
            "hi number {} from the main thread {:?}!",
            i,
            thread::current().id()
        );
    }

    // let document = window()
    //     .and_then(|win| win.document())
    //     .expect("Could not access document");
    // let body = document.body().expect("Could not access document.body");
    // let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    // body.append_child(text_node.as_ref())
    //     .expect("Failed to append text");
}

fn main() {
    console_log::init().unwrap();
    console_error_panic_hook::set_once();

    run();
}
