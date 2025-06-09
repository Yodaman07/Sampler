use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use rodio::source::SineWave;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).expect("Couldn't make sink");



    //file path relative to Cargo.toml
    let file = File::open("music/ivy.mp3").expect("Couldn't open file");
    let file = BufReader::new(file);

    let source = Decoder::new(file).expect("Couldn't decode");
    // let source = source.fade_in(Duration::from_secs(4)).take_duration(Duration::from_secs(2)).repeat_infinite();
    let source = source.skip_duration(Duration::from_secs(100)).take_duration(Duration::from_secs(10));
    
    
    
    
    sink.append(source);

    sink.sleep_until_end();
    println!("Hello, world!");
}
