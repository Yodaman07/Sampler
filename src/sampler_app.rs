use std::fmt::format;
use eframe::epaint::Color32;
use egui::{include_image, Image, Ui, Vec2};
use rodio::{OutputStreamHandle, Sink};
use youtube_dl::{YoutubeDl, YoutubeDlOutput};
use crate::Song;

struct SamplerApp{
    file_loader: FileLoader,
    audio_player: AudioPlayer
}

struct FileLoader{
    path: Option<String>, //representing if a song file has been loaded or not yet
    yt_url: Option<String> //option is none is using local file, and is some if downloading from yt
}
impl FileLoader{
    fn download_file(&self) -> Option<String> { //if couldn't dl, will return none

        if let Some(link) = &self.yt_url {
            //grab metadata first then download file
            let metadata : Result<YoutubeDlOutput, youtube_dl::Error> = YoutubeDl::new(link)
                .socket_timeout("15")
                .run();

            if let Err(e) = metadata{
                println!("{}", e);
                println!("An error occurred when grabbing metadata. The video link may be invalid. Please try again");
                return None;
            }

            let metadata = metadata.unwrap();
            let title = metadata.into_single_video().unwrap().title.expect("Error getting video title");

            let output = YoutubeDl::new(link)
                .socket_timeout("15") // after 15s, a failed download will be reported
                .output_template(format!("{}.%(ext)s", title))
                .extract_audio(true)
                .extra_arg("--audio-format")
                .extra_arg("mp3")
                .download_to("music")
                .unwrap();

            println!("Downloaded {}", title);
            Some(title)
        }else{ None }
    }
}

struct AudioPlayer{ //audio player includes the waveform, the pause/play btn, and the time indicator
    path: Option<String>,
    current_time: f32,
    stream_handle: OutputStreamHandle,
    sink: Option<Sink>,
    audio_player_state: AudioPlayerState
}

enum AudioPlayerState{
    PAUSED,
    PLAYING
}

impl AudioPlayer{
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
    fn construct(&mut self, ui: &mut Ui){
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
        if matches!(self.audio_player.audio_player_state, AudioPlayerState::PLAYING) { //ai help
            ctx.request_repaint(); // This forces egui to update continuously
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            //App content goes here


            self.audio_player.construct(ui); // ai help, but it is a compact way to display the audio player

            ui.add_space(10.0);
            match &mut self.file_loader.yt_url {
                None => {
                    //Code for file dialog
                }
                Some(txt) => {
                    ui.text_edit_singleline(txt);
                    if ui.button("Download Video").clicked(){
                        println!("{}", txt); //Video url
                        let title: Option<String> = self.file_loader.download_file();
                        if let Some(t) = title{
                            let path = format!("music/{}.mp3", t);
                            println!("{}", path);
                            self.audio_player.path = Some(path);
                        }

                    }
                }
            }

        });
    }
}

impl SamplerApp{
    fn new( stream_handle: OutputStreamHandle) -> Self {
        Self{
            file_loader: FileLoader{
                path: None,
                yt_url: Some(String::from("")), //Some value
            },
            audio_player: AudioPlayer{
                path: None, //This is the default path of the music player. By default it won't play because its waiting to load a file
                current_time: 0.0,
                stream_handle,
                sink: None,
                audio_player_state: AudioPlayerState::PAUSED,
            }
        }
    }
}
