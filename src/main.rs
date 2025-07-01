mod sampler_app;
mod audio_player;
mod file_loader;

use regex::Regex;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use audio_visualizer::Channels;
use audio_visualizer::waveform::png_file::waveform_static_png_visualize;
use egui::debug_text::print;

//BUG YOU CAN CLICK OUT OF BOUNDS ON THE PLAYING SONG
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

    fn original_duration(&self) -> Duration{ //returns the original song length (before clipping)
        self.new().total_duration().expect("Error decoding duration")
    }

    fn get_samples(&self) -> Vec<i16> { //ai help
        let decoder = self.new();
        decoder.collect()
    }

    fn get_name(&self) -> String{
        let re = Regex::new("([^/]+)(.mp3)").unwrap(); //regex should be good on all file names, as long as they have no / character
        let Some(captured) = re.captures(&self.path) else {
            println!("Unable to find name");
            return String::from("");
        };
        captured[1].to_string() //group 1
    }

}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    //file path relative to Cargo.toml
    // let video_title = download_file("https://youtu.be/YWXvj_fb1Ec?si=Y8jAqiHDAb3z3J9f");
    // let file = File::open(format!("music/{}.mp3", video_title)).expect("Couldn't open file");


    sampler_app::init_app(stream_handle).expect("Unable to open app");
}
