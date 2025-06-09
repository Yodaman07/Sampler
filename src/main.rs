use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use youtube_dl::YoutubeDl;

fn main() {
    let video_title = download_file("https://www.youtube.com/watch?v=nLpTpr7_Ye8");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).expect("Couldn't make sink");

    //file path relative to Cargo.toml
    let file = File::open(format!("music/{}.mp3", video_title)).expect("Couldn't open file");
    // let file = File::open("music/dtmlive.mp3").expect("Couldn't open file");
    let file = BufReader::new(file);

    let source = Decoder::new(file).expect("Couldn't decode");
    //messing with the song
    // let source = source.take_duration(Duration::from_secs_f32(0.25));
    // let source = source.speed(1.2);

    // let source = source.fade_in(Duration::from_secs(4)).take_duration(Duration::from_secs(2)).repeat_infinite();
    // let source = source.skip_duration(Duration::from_secs(100)).take_duration(Duration::from_secs(10));
    sink.append(source);

    sink.sleep_until_end();
    println!("Hello, world!");
}


// #[tokio::main]
fn download_file(video_link: &str) -> String {
    //grab metadata first then download file
    let metadata = YoutubeDl::new(video_link)
        .socket_timeout("15")
        .run()
        .expect("An error occurred when grabbing metadata");


    let title = metadata.into_single_video().unwrap().title.expect("Error getting video title");

    let output = YoutubeDl::new(video_link)
        .socket_timeout("15")
        .output_template(format!("{}.%(ext)s", title))
        .extract_audio(true)
        .extra_arg("--audio-format")
        .extra_arg("mp3")
        .download_to("music")
        .unwrap();

    println!("Downloaded {}", title);
    title
}