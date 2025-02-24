use std::{fs::File, io::BufReader};

use rodio::{Decoder, OutputStream, Sink, Source};
use snowstorm_logic::{audio_handler, AudioMessage};
use tokio::sync::mpsc::{channel, Sender};

use crate::messages::{play, PlayFile};

pub async fn await_audio(sender: Sender<AudioMessage>) {let receiver = PlayFile::get_dart_signal_receiver();
   while let Some(signal) = PlayFile::get_dart_signal_receiver().recv().await {
    rinf::debug_print!("{:?}", signal.message.command());
        match signal.message.command() {
            play::AudioCommand::Play => { sender.send(AudioMessage::Play(signal.message.location)).await.expect("Could not start the track.");
            }
            play::AudioCommand::Stop => {sender.send(AudioMessage::Stop).await.expect("Could not stop the track.");},
            play::AudioCommand::Continue => {sender.send(AudioMessage::Start).await.expect("Could not resume the track.");},
        }
    }
}
