mod sampler_app;

use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, source::Source, OutputStream, Sink};
use youtube_dl::YoutubeDl;


pub struct Song{ path: String }
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
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    //file path relative to Cargo.toml
    // let video_title = download_file("https://youtu.be/YWXvj_fb1Ec?si=Y8jAqiHDAb3z3J9f");
    // let file = File::open(format!("music/{}.mp3", video_title)).expect("Couldn't open file");


    sampler_app::init_app(stream_handle).expect("Unable to open app");

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