mod sampler_app;
mod audio_player;
mod file_loader;

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Duration;
use audio_visualizer::{ChannelInterleavement, Channels};
use audio_visualizer::waveform::png_file::waveform_static_png_visualize;
use egui::debug_text::print;
use regex::Regex;
use rodio::{Decoder, source::Source, OutputStream};
use symphonia::core::audio::{AudioBuffer, Signal};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};

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
        let re = Regex::new("/((.*).mp3)").unwrap();
        let Some(captured) = re.captures(&self.path) else {
            println!("Unable to find name");
            return String::from("");
        };

        captured[2].to_string()
    }

}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    //file path relative to Cargo.toml
    // let video_title = download_file("https://youtu.be/YWXvj_fb1Ec?si=Y8jAqiHDAb3z3J9f");
    // let file = File::open(format!("music/{}.mp3", video_title)).expect("Couldn't open file");

    let mut path = PathBuf::new();
    path.push("music/closetoyou.mp3");

    // let s = Song{path: "music/closetoyou.mp3".to_string()};

    sampler_app::init_app(stream_handle).expect("Unable to open app");
}
