//! This `hub` crate is the
//! entry point of the Rust logic.

use audio::await_audio;
use snowstorm_logic::audio_handler;
use tokio::{sync::{mpsc::channel, Mutex}, task::spawn_local};

mod messages;
mod sample_functions;
mod audio;

// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

rinf::write_interface!();

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
async fn main() {   let (channel, receiver) = channel::<snowstorm_logic::AudioMessage>(5); // how big should this be?
    snowstorm_logic::create_db("".to_string()).await;
    // Spawn concurrent tasks.
    // Always use non-blocking async functions like `tokio::fs::File::open`.
    // If you must use blocking code, use `tokio::task::spawn_blocking`
    // or the equivalent provided by your async library.
    tokio::spawn(sample_functions::communicate());
    tokio::spawn(audio::await_audio(channel));
    // This should never finish. Place everything above this call
    // TODO! find a different way of starting our audio service
    snowstorm_logic::audio_handler(receiver).await;

    // Keep the main function running until Dart shutdown.
    rinf::dart_shutdown().await;
}
