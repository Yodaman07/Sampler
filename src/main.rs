use std::fs::File;
use std::io::BufReader;
use std::iter::Skip;
use std::time::Duration;
use rodio::{Decoder, OutputStream, source::Source, Sink, Sample};
use rodio::cpal::FromSample;
use rodio::source::{SkipDuration, Speed, TakeDuration};
use youtube_dl::YoutubeDl;

struct Song{ path: String }

impl Song {
    fn new(&self) -> Decoder<BufReader<File>> { //opens and decodes song
        let file = File::open(&self.path).expect("Couldn't open file");
        let file = BufReader::new(file);
        Decoder::new(file).expect("Couldn't decode")
    }

    fn clip(&self, speed: f32, start_time: f32, end_time: f32, loop_track: bool) -> impl Source<Item = i16> + Send{ //cleaned up return statement with claude
        self.new()
            .speed(speed)
            .skip_duration(Duration::from_secs_f32(start_time))
            .take_duration(Duration::from_secs_f32(end_time - start_time))
    }
}


fn main() {
    // let video_title = download_file("https://www.youtube.com/watch?v=nLpTpr7_Ye8");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).expect("Couldn't make sink");
    //Sink is like the audio player
    
    //file path relative to Cargo.toml
    // let file = File::open(format!("music/{}.mp3", video_title)).expect("Couldn't open file");
    let dtmlive: Song = Song{ //base song, can make modifications via clips
        path: String::from("music/dtmlive.mp3"),
    };

    let clip1 = dtmlive.clip(1.5, 6.0, 12.0, false);
    let clip2 = dtmlive.clip(0.80, 30.0, 33.0, false);
    
    sink.append(clip1);
    sink.append(clip2);
    
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