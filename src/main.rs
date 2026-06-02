use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: audio_player <path-to-file.mp3-or-.wav>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        eprintln!("Error: File not found: {}", file_path);
        std::process::exit(1);
    }

    let (_stream, stream_handle) = OutputStream::try_default()
        .expect("Could not open audio output device.");

    let sink = Sink::try_new(&stream_handle)
        .expect("Could not create audio sink.");

    let file = File::open(file_path)
        .expect("Failed to open file.");

    let source = Decoder::new(BufReader::new(file))
        .expect("Could not decode audio. Is it a valid MP3 or WAV file?");

    sink.append(source);

    println!("Playing: {}", file_path);
    println!("Press Ctrl+C to stop.");

    // This blocks the main thread until the track finishes — the proper way
    sink.sleep_until_end();

    println!("Done.");
}
