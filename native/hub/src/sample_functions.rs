//! This module is written for Rinf demonstrations.

use std::{fs::read_to_string, thread::sleep, time::Duration};

use snowstorm_logic::init_db;

use crate::messages::{self, *};

pub async fn communicate() {
    // Send signals to Dart like below.
    SmallNumber { number: 7 }.send_signal_to_dart();
    // Copied from further on down.
    let mut dart_results = Vec::new();
    let songs = snowstorm_logic::get_songs().await;

    for song in songs {
        dart_results.push(messages::Song {
            title: song.metadata.name,
            album: song.metadata.album,
            artist: song.metadata.artist,
            location: song.location,
        });
    }
    Songs {
        songs: dart_results,
    }
    .send_signal_to_dart();

    let receiver = FolderPath::get_dart_signal_receiver();
    while let Some(signal) = receiver.recv().await {
        let msg = signal.message;
        if msg.path == "" {
            rinf::debug_print!("is null");
        }
        snowstorm_logic::add_folder(msg.path.clone()).await;
        rinf::debug_print!("{}", msg.path);
        rinf::debug_print!("wrote to db");
        let self_results = snowstorm_logic::get_songs().await;
        let mut dart_results = Vec::new();
        for song in self_results {
            dart_results.push(messages::Song {
                title: song.metadata.name,
                album: song.metadata.album,
                artist: song.metadata.artist,
                location: song.location,
            });
        }
        messages::Songs {
            songs: dart_results,
        }
        .send_signal_to_dart();
    }
}

// Though async tasks work, using the actor model
// is highly recommended for state management
// to achieve modularity and scalability in your app.
// To understand how to use the actor model,
// refer to the Rinf documentation.
