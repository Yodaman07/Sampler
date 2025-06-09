use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    //file path relative to Cargo.toml
    let file = File::open("music/nikes.mp3").expect("Couldn't open file");
    let file = BufReader::new(file);

    let source = Decoder::new(file).expect("Couldn't decode");

    stream_handle.play_raw(source.convert_samples()).expect("Unable to convert samples");

    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("Hello, world!");
}
