use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use rodio::source::{SkipDuration, Speed, TakeDuration};
use youtube_dl::YoutubeDl;

struct Song{
    path: String,
    //properties
    speed: f32,
    start_time: f32,
    end_time: f32,
    loop_track: bool
}

impl Song {
    fn create(&self) -> TakeDuration<SkipDuration<Speed<Decoder<BufReader<File>>>>> {
        let file = File::open(&self.path).expect("Couldn't open file");
        let file = BufReader::new(file);

        let source = Decoder::new(file).expect("Couldn't decode");

        let source = source
            .speed(self.speed)
            .skip_duration(Duration::from_secs_f32(self.start_time))
            .take_duration(Duration::from_secs_f32(self.end_time-self.start_time));

        source
    }
}


fn main() {
    // let video_title = download_file("https://www.youtube.com/watch?v=nLpTpr7_Ye8");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).expect("Couldn't make sink");
    //Sink is like the audio player


    //file path relative to Cargo.toml
    // let file = File::open(format!("music/{}.mp3", video_title)).expect("Couldn't open file");

    //When I start on the sampler, I should have a song stuct but also have clip structs where I can save each clip 
    let mut dtmlive: Song = Song{
        path: String::from("music/dtmlive.mp3"),
        speed: 1.3,
        start_time: 0.0,
        end_time: 10.0,
        loop_track: false
    };

    //messing with the song
    // let source = source.take_duration(Duration::from_secs_f32(0.25));
    // let source = source.fade_in(Duration::from_secs(4)).take_duration(Duration::from_secs(2)).repeat_infinite();
    
    sink.append(dtmlive.create());

    dtmlive.speed = 1.0;
    dtmlive.start_time = 13.0;
    dtmlive.end_time = 25.0;
    sink.append(dtmlive.create());

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