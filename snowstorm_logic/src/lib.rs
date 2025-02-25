use std::{
    ffi::OsStr, fs::{read_dir, File, OpenOptions, ReadDir}, io::BufReader, path::{self, Path}, sync::OnceLock
};

mod metadata;

use metadata::{Metadata, Song};
use rodio::{Decoder, OutputStream, Sink};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use symphonia::{
    core::{
        formats::FormatOptions,
        io::{MediaSourceStream, MediaSourceStreamOptions},
        meta::{MetadataOptions, Tag},
        probe::Hint,
    },
    default::{get_codecs, get_probe},
};
use tokio::sync::mpsc::Receiver;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub async fn add_folder(dir: String) {
    if let Some(files) = traverse_directory(dir.clone()) {
        for file in files {
            if let Some((_, extension)) = file.rsplit_once(".") {
                if extension == "flac" {
                    // TODO: Remove arbritrary flac restriction
                    add_song(file).await;
                }
            };
        }
    }
}

fn traverse_directory(dir: String) -> Option<Vec<String>> {
    let mut result = Vec::new();

    if let Ok(val) = read_dir(dir) {
        for entry in val {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Ok(path) = entry.path().canonicalize() {
                            if let Some(location) = path.as_path().to_str() {
                                result.push(location.to_string());
                            }
                        }
                    }
                    if file_type.is_dir() {
                        if let Some(path) = entry.path().as_path().to_str() {
                            if let Some(recursive_result) = traverse_directory(path.to_string()) {
                                result.append(&mut recursive_result.clone());
                            }
                        }
                    }
                }
            }
        }
        return Some(result);
    }
    None
}

static  DB: OnceLock<Surreal<Client>> = OnceLock::new();

pub async fn create_db(dir: String) {    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.expect("Could not create a db.");
    DB.get_or_init(|| db);
}

pub async fn init_db() -> Surreal<Client> {
    let db = DB.get()
        .expect("Could not connect to the db.");

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Could not login to the db.");
    return db.clone();
}

pub async fn read_metadata(song_location: String) -> Metadata {
    let codec = get_codecs();
    let probe = get_probe();
    let file =
        File::open(Path::new(&song_location)).expect("Could not open file in read_metadata.");
    let mss = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());
    let mut result = Metadata {
        name: song_location,
        album: "null".to_string(),
        artist: "null".to_string(),
    };
    if let Ok(mut metadata) = probe.format(
        &Hint::new(),
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    ) {
        if let Some(mut metadata) = metadata.metadata.get() {
            if let Some(metadata) = metadata.skip_to_latest() {
                for tag in metadata.tags() {
                    if let Some(tag_key) = tag.std_key {
                        match tag_key {
                            symphonia::core::meta::StandardTagKey::Album => {
                                result.album = tag.value.to_string();
                            }
                            symphonia::core::meta::StandardTagKey::Artist => {
                                result.artist = tag.value.to_string();
                            }
                            symphonia::core::meta::StandardTagKey::TrackTitle => {
                                result.name = tag.value.to_string();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    result
}

pub async fn add_song(song_location: String) {
    let db = init_db().await;
    db.use_ns("snowstorm")
        .use_db("songs")
        .await
        .expect("Could not connect to the namespace.");
    let metadata = read_metadata(song_location.clone()).await;
    let song: Option<Song> = db
        .create(("song", &metadata.name))
        .content(Song {
            location: song_location.clone(),
            metadata: metadata,
        })
        .await
        .expect("Could not write to db.");
}

pub async fn get_songs() -> Vec<Song> {
    let db = init_db().await;
    db.use_ns("snowstorm")
        .await
        .expect("Could not find the snowstorm namespace");
    db.use_db("songs")
        .await
        .expect("Could not find the songs db");

    let songs: Vec<Song> = db
        .select("song")
        .await
        .expect("Could not select resources with the identifier song.");
    return songs;
}

pub enum AudioMessage {
    Play(String),
    Start,
    Stop,
}

pub async fn audio_handler(mut receiver: Receiver<AudioMessage>) {
    println!("Attempting to play");
    let (_stream, stream_handle) = OutputStream::try_default().expect("Could not obtain an output device.");
    let sink = Sink::try_new(&stream_handle).expect("Could not create a new sink.");
   
    while let Some(command) = receiver.recv().await {
        match command {
            AudioMessage::Play(location) => {
                sink.skip_one();
                let file = BufReader::new(File::open(location).expect("Could not find the file that should be played."));
                let source = Decoder::new(file).expect("Could not decode into a source.");
                sink.append(source);
                sink.play();
                println!("can you hear me?");
            },
            AudioMessage::Stop => {
                sink.pause();
            },
            AudioMessage::Start => {
                sink.play();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
