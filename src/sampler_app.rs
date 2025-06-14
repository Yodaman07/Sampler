use egui::{include_image, Image, ImageSource, Vec2};
use rodio::{OutputStreamHandle, Sink};

use crate::Song;

struct SamplerApp{
    stream_handle: OutputStreamHandle,
    sink: Option<Sink>,
    audio_player_state: AudioPlayerState
}

enum AudioPlayerState{
    PAUSED,
    PLAYING
}

//Default boilerplate stuff from https://github.com/emilk/egui/blob/main/examples/hello_world/src/main.rs
pub fn init_app(stream_handle: OutputStreamHandle) -> eframe::Result{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([850.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sampler",
        options,

        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(SamplerApp::new(stream_handle)))
        } ),
    )
}



impl eframe::App for SamplerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //App content goes here
            ui.heading("Awesome Sampler");


            if ui.add_sized(Vec2::new(50.0, 50.0), egui::Button::image( self.get_player_icon().clone() )).clicked(){

                match &self.sink{
                    None => { //if sink doesn't exist, this will make it and load a clip
                        let sink = Sink::try_new(&self.stream_handle).expect("Couldn't make sink");

                        let dtmlive: Song = Song{ path: String::from("music/nikes.mp3"), }; //base song, can make modifications via clips
                        let clip1 = dtmlive.clip(1.0, 30.0, 45.0, false);

                        //Sink is like the audio player
                        sink.append(clip1);
                        self.audio_player_state = AudioPlayerState::PLAYING;

                        self.sink = Some(sink);
                    }
                    Some(sink) => {
                        match &self.audio_player_state{
                            AudioPlayerState::PAUSED => {
                                sink.play();
                                println!("Play");
                                self.audio_player_state = AudioPlayerState::PLAYING;
                            }
                            AudioPlayerState::PLAYING => {
                                sink.pause();
                                println!("Pause");
                                self.audio_player_state = AudioPlayerState::PAUSED;
                            }
                        };

                    }
                }
            }

            if ui.button("TrackTime").clicked(){
                let time = self.sink.as_ref().unwrap().get_pos();
                println!("Current Time {:?}", time); //this is the time since the clip statred. Need to acount for starting delay and any speed changes
            }




        });
    }
}

impl SamplerApp{
    fn new( stream_handle: OutputStreamHandle) -> Self {
        Self{
            stream_handle,
            sink: None,
            audio_player_state: AudioPlayerState::PAUSED
        }
    }

    fn get_player_icon(&self) -> Image{
        match self.audio_player_state{
            AudioPlayerState::PAUSED => { Image::from(include_image!("../imgs/play.svg")) }
            AudioPlayerState::PLAYING => { Image::from(include_image!("../imgs/pause.svg")) }
        }
    }
}
