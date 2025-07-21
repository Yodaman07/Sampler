use crate::Song;
use audio_visualizer::waveform::png_file::waveform_static_png_visualize;
use audio_visualizer::Channels;
use eframe::epaint::Color32;
use egui::{include_image, Image, Rect, Ui};
use rodio::{OutputStream, Sink};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

//threads
use std::sync::{Arc, Mutex};
use tokio::runtime;


pub(crate) const PLAYER_LEFT_OFFSET: f32 = 100.0; //the audio player starts 100 pixels from the left of the screen


pub struct AudioPlayer{ //audio player includes the waveform, the pause/play btn, and the time indicator
    pub path: Option<String>,  //representing if a song file has been loaded or not yet
    pub current_time: f32,
    pub playback_time: f32, //total time
    pub stream_handle: OutputStream,
    pub(crate) sink: Option<Sink>,
    pub audio_player_state: AudioPlayerState,
    thread2: runtime::Runtime,
    image_loaded: Arc<Mutex<bool>>
}

pub enum AudioPlayerState{
    PAUSED,
    PLAYING
}

impl AudioPlayer{
    pub fn new(handle: OutputStream, thread2: runtime::Runtime) -> Self{
        Self{
            path: None, //This is the default path of the music player. By default, it won't play because it's waiting to load a file
            current_time: 0.0,
            playback_time: 0.0, //total length of track
            stream_handle: handle,
            sink: None,
            audio_player_state: AudioPlayerState::PAUSED,
            thread2,
            image_loaded: Arc::new(Mutex::new(false))
        }
    } //creates a new default audio player


    pub fn startup(&mut self){ //load sink and song when you load it via file loader (yt or local), also generate the waveform image
        if let Some(path) = &self.path{

            let sink = Sink::connect_new(&self.stream_handle.mixer());
            let s: Song = Song{ path: String::from(path)}; //base song can make modifications via clips

            self.playback_time = s.original_duration().as_secs_f32(); //not good at downloading really short videos
            let clip1 = s.clip(1.0, 0.0, self.playback_time, false); //full song

            //Sink is what is actually playing the music
            sink.append(clip1);
            sink.pause();
            self.sink = Some(sink);


            let f_name = format!("{}.png", s.get_name());
            let mut p: PathBuf = PathBuf::from("waveforms/");
            p.push(&f_name);
            if !Path::new(&p).exists(){
                //Generate png if it doesn't exist already
                println!("Waveform not found, making one now. Saving as {:?}", p);

                let loaded = Arc::clone(&self.image_loaded); //cloning arc
                self.thread2.spawn(async move { //move means it will take ownership of variables

                    waveform_static_png_visualize(
                        &s.get_samples(),
                        Channels::Mono,
                        "waveforms/",
                        &f_name,
                    );

                    let mut state = loaded.lock().unwrap();
                    *state = true;
                    println!("Image generated")
                });

            }else{ //image already exists on device
                *self.image_loaded.lock().unwrap() = true;
                println!("Path exists at {:?}", p);
            }
        }else{ println!("No file has been loaded. Please download from youtube, or load a local song") }
    }
    pub fn skip_to(&mut self, time: f32){
        // let res = self.sink.as_ref().unwrap().try_seek(Duration::from_secs_f32(time));
        if let Some(s) = self.sink.as_ref(){
            s.try_seek(Duration::from_secs_f32(time)).expect("Error skipping content");
        }else{ println!("Unable to skip content. Try loading audio first"); }
        self.current_time = time;
    }
    fn get_player_icon(&self) -> Image{
        match self.audio_player_state{
            AudioPlayerState::PAUSED => { Image::from(include_image!("../imgs/play.svg")) } //optimize with lifetimes later
            AudioPlayerState::PLAYING => { Image::from(include_image!("../imgs/pause.svg")) }
        }
    }
    pub fn get_pos_from_time(&self, time: Option<f32>) -> f32{ //650 pixel long play area
        let window_space = 650.0;
        if let Some(t) = time{ (window_space/self.playback_time) * t }
        else {(window_space/self.playback_time) * self.current_time}
    }
    fn get_time_from_pos(&self, pos: f32) -> f32{
        let window_space = 650.0;

        if pos < 0.0 {return 0.0} //prevent incorrect negative values

        (pos/window_space) * self.playback_time
    }

    fn paint_waveform(&self, ui: &mut Ui, target: Rect){
        //painting waveform
        if *self.image_loaded.lock().unwrap() {
            if let Some(path) = &self.path {
                //self.path is the song path, and the song name is the same as the image name, so we can extract it using the regex from earlier
                let name = Song { path: String::from(path) }.get_name();
                let path = format!("waveforms/{}.png", name);
                // println!("Loading image from {}", path);

                let mut buffer = vec![];
                File::open(path)
                    .unwrap()
                    .read_to_end(&mut buffer)
                    .unwrap();

                let image = Image::from_bytes("image_id", buffer)
                    .corner_radius(25)
                    .max_width(650.0)
                    .fit_to_original_size(1.0);
                ui.put(target, image);
                //height is 65.0 and width is 650.0
            }
        }
    }
    pub fn construct(&mut self, ui: &mut Ui){
        if let Some(s) = &self.sink { //handles the auto pause at the end of the track, may be buggy
            self.current_time = s.get_pos().as_secs_f32();

            if self.current_time >= self.playback_time{
                println!("End of song");
                s.pause();
                self.audio_player_state = AudioPlayerState::PAUSED; //will auto pause the song at the end
            }
        }


        ui.horizontal(|ui| {

            ui.add_space(10.0);
            ui.vertical(|ui|{
                ui.add_space(-6.0); // bring it back to the top of the screen
                ui.add_space(12.5); //move down 10 to match the player rect
                let control_button = egui::Button::image( self.get_player_icon().clone() )
                    .corner_radius(13);
                if ui.add_sized([55.0, 55.0], control_button).clicked(){
                    if let Some(sink) = &self.sink{
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
                    }else { self.startup() } //initializing the sink if it doesn't exist
                }
            });

            //Song player
            let player_rect = Rect::from_two_pos(egui::pos2(PLAYER_LEFT_OFFSET, 10.0), egui::pos2(650.0 + PLAYER_LEFT_OFFSET, 75.0));
            let resp = ui.allocate_rect(player_rect, egui::Sense::CLICK);

            if resp.clicked(){ //scrolling along the song
                let pos = resp.interact_pointer_pos().expect("Error getting mouse position");
                let accurate_x = pos.x - PLAYER_LEFT_OFFSET; //100 pixel offset from the left of the screen
                println!("scroll to {}", accurate_x);

                self.skip_to(self.get_time_from_pos(accurate_x));
            }

            ui.painter().rect_filled(player_rect, 25, Color32::DARK_GRAY);
            self.paint_waveform(ui, player_rect); //loads the waveform and paints it to the player rectangle

            //trackhead (where you are in the song)
            let pos = self.get_pos_from_time(None);
            ui.painter().rect_filled(Rect::from_two_pos(egui::pos2(PLAYER_LEFT_OFFSET + pos, 10.0), egui::pos2(PLAYER_LEFT_OFFSET + 9.0 + pos, 75.0)), 10, Color32::WHITE); //height is 65.0


            ui.label(format!("{:.2?}/{:.2?}", Duration::from_secs_f32(self.current_time), Duration::from_secs_f32(self.playback_time)));
        });

        if ui.button("TrackTime").clicked(){ //UNSAFE
            let time = self.sink.as_ref().unwrap().get_pos();
            println!("Current Time {:?}", time); //This is the time since the clip started. Need to account for starting delay and any speed changes
        }



    }
}