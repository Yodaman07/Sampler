use eframe::emath::Vec2;
use eframe::epaint::Color32;
use egui::{include_image, Image, Ui};
use rodio::{OutputStreamHandle, Sink};
use crate::Song;

pub struct AudioPlayer{ //audio player includes the waveform, the pause/play btn, and the time indicator
    pub path: Option<String>,
    current_time: f32,
    stream_handle: OutputStreamHandle,
    sink: Option<Sink>,
    pub audio_player_state: AudioPlayerState
}

pub enum AudioPlayerState{
    PAUSED,
    PLAYING
}

impl AudioPlayer{
    pub fn new(handle: OutputStreamHandle) -> Self{
        Self{
            path: None, //This is the default path of the music player. By default it won't play because its waiting to load a file
            current_time: 0.0,
            stream_handle: handle,
            sink: None,
            audio_player_state: AudioPlayerState::PAUSED,
        }
    } //creates a new default audio player
    fn startup(&mut self){ //load sink and song when you play the song for the first time
        if let Some(path) = &self.path{
            let sink = Sink::try_new(&self.stream_handle).expect("Couldn't make sink");

            let s: Song = Song{ path: String::from(path)}; //base song can make modifications via clips

            let clip1 = s.clip(1.0, 0.0, 11.225, false);

            //Sink is like the audio player
            sink.append(clip1);
            self.audio_player_state = AudioPlayerState::PLAYING;

            self.sink = Some(sink);
        }else{
            println!("No file has been loaded. Please download from youtube, or load a local song")
        }

    }
    fn get_player_icon(&self) -> Image{
        match self.audio_player_state{
            AudioPlayerState::PAUSED => { Image::from(include_image!("../imgs/play.svg")) } //optimize with lifetimes later
            AudioPlayerState::PLAYING => { Image::from(include_image!("../imgs/pause.svg")) }
        }
    }
    fn get_pointer_pos(&self) -> f32{ //650 pixel long play area
        let length = 11.225;
        let window_space = 650.0;
        (window_space/11.225) * self.current_time
    }
    pub fn construct(&mut self, ui: &mut Ui){
        //Song player
        ui.painter().rect_filled(egui::Rect::from_two_pos(egui::pos2(100.0, 10.0), egui::pos2(750.0, 60.0)), 25, Color32::DARK_GRAY);
        if let Some(s) = &self.sink {
            self.current_time = s.get_pos().as_secs_f32();
        }

        let pos = self.get_pointer_pos();
        ui.painter().rect_filled(egui::Rect::from_two_pos(egui::pos2((100.0 + pos), 10.0), egui::pos2((105.0 + pos), 60.0)), 10, Color32::YELLOW);

        ui.horizontal(|ui| {
            ui.add_space(30.0);
            if ui.add_sized(Vec2::new(40.0, 40.0), egui::Button::image( self.get_player_icon().clone() )).clicked(){
                match &self.sink{
                    None => self.startup(), //if sink doesn't exist, this will make it and load a clip
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
            println!("Current Time {:?}", time); //This is the time since the clip started. Need to account for starting delay and any speed changes
        }
        ui.add_space(5.0);

    }
}