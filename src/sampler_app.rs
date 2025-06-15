use std::io::sink;
use eframe::epaint::Color32;
use eframe::epaint::ImageData::Color;
use egui::{include_image, Align, Direction, Image, ImageSource, Layout, Pos2, TopBottomPanel, Vec2};
use egui_extras::StripBuilder;
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
        if matches!(self.audio_player_state, AudioPlayerState::PLAYING) { //ai help
            ctx.request_repaint(); // This forces egui to update continuously
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            //App content goes here

            ui.painter().rect_filled(egui::Rect::from_two_pos(egui::pos2(100.0, 10.0), egui::pos2(750.0, 60.0)), 25, Color32::DARK_GRAY);

            let mut time: f32 = 0.0;
            if let Some(a) = &self.sink {
                time = a.get_pos().as_secs_f32();
            }
            ui.painter().rect_filled(egui::Rect::from_two_pos(egui::pos2((100.0 + time*2.0), 10.0), egui::pos2((105.0 + time*2.0), 60.0)), 10, Color32::YELLOW);

            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.add_space(30.0);
                if ui.add_sized(Vec2::new(40.0, 40.0), egui::Button::image( self.get_player_icon().clone() )).clicked(){
                    match &self.sink{
                        None => { //if sink doesn't exist, this will make it and load a clip
                            let sink = Sink::try_new(&self.stream_handle).expect("Couldn't make sink");

                            let dtmlive: Song = Song{ path: String::from("music/closetoyou.mp3"), }; //base song, can make modifications via clips
                            let clip1 = dtmlive.clip(1.0, 0.0, 30.0, false);

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
            });


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
