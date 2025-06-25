use std::time::Duration;
use eframe::emath::Vec2;
use eframe::epaint::Color32;
use egui::{include_image, Image, Pos2, Ui};
use rodio::{OutputStreamHandle, Sink};
use crate::Song;

pub struct AudioPlayer{ //audio player includes the waveform, the pause/play btn, and the time indicator
    pub path: Option<String>,
    current_time: f32,
    playback_time: f32, //total time
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
            playback_time: 0.0,
            stream_handle: handle,
            sink: None,
            audio_player_state: AudioPlayerState::PAUSED,
        }
    } //creates a new default audio player
    pub fn startup(&mut self){ //load sink and song when you load it via fileloader (yt or local)
        if let Some(path) = &self.path{
            let sink = Sink::try_new(&self.stream_handle).expect("Couldn't make sink");
            let s: Song = Song{ path: String::from(path)}; //base song can make modifications via clips

            self.playback_time = s.original_duration().as_secs_f32(); //not good at downloading really short videos
            let clip1 = s.clip(1.0, 0.0, self.playback_time, false); //full song

            //Sink is like the audio player
            sink.append(clip1);
            sink.pause();

            self.sink = Some(sink);
        }else{
            println!("No file has been loaded. Please download from youtube, or load a local song")
        }
    }

    fn skip_to(&mut self, time: f32){
        // let res = self.sink.as_ref().unwrap().try_seek(Duration::from_secs_f32(time));
        if let Some(s) = self.sink.as_ref(){
            s.try_seek(Duration::from_secs_f32(time)).expect("Error skipping content");
        }else{
            println!("Unable to skip content. Try loading audio first");
        }
        self.current_time = time;
    }


    fn get_player_icon(&self) -> Image{
        match self.audio_player_state{
            AudioPlayerState::PAUSED => { Image::from(include_image!("../imgs/play.svg")) } //optimize with lifetimes later
            AudioPlayerState::PLAYING => { Image::from(include_image!("../imgs/pause.svg")) }
        }
    }
    fn get_pos_from_time(&self) -> f32{ //650 pixel long play area
        let window_space = 650.0;
        (window_space/self.playback_time) * self.current_time
    }
    fn get_time_from_pos(&self, pos: f32) -> f32{
        let window_space = 650.0;

        if pos < 0.0 {return 0.0} //prevent incorrect negative values

        (pos/window_space) * self.playback_time
    }
    pub fn construct(&mut self, ui: &mut Ui){
        //Song player

        let rect = egui::Rect::from_two_pos(egui::pos2(100.0, 10.0), egui::pos2(750.0, 60.0));
        let resp = ui.allocate_rect(rect, egui::Sense::CLICK);

        if resp.clicked(){
            let pos = resp.interact_pointer_pos().expect("Error getting mouse position");
            let accurate_x = pos.x - 100.0; //100 pixel offset from left of the screen
            println!("{}", accurate_x);

            self.skip_to(self.get_time_from_pos(accurate_x));
        }

        ui.painter().rect_filled(rect, 25, Color32::DARK_GRAY);


        if let Some(s) = &self.sink {
            self.current_time = s.get_pos().as_secs_f32();

            if self.current_time >= self.playback_time{
                println!("End of song");
                s.pause();
                self.audio_player_state = AudioPlayerState::PAUSED; //will auto pause the song at the end
            }

        }




        //yellow pointer
        let pos = self.get_pos_from_time();
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

        if ui.button("TrackTime").clicked(){ //UNSAFE
            let time = self.sink.as_ref().unwrap().get_pos();
            println!("Current Time {:?}", time); //This is the time since the clip started. Need to account for starting delay and any speed changes
        }
        ui.add_space(5.0);

    }
}