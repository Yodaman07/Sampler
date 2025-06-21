use egui::Ui;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};
use crate::AudioPlayer::AudioPlayer;

#[derive(PartialEq)]
enum Tabs{ LOCAL, YTDl}
pub struct FileLoader{
    path: Option<String>, //representing if a song file has been loaded or not yet
    yt_url: String,
    tab: Tabs //loading a file from yt or locally?
}
impl FileLoader{
    pub fn new() -> Self{
        Self{
            path: None,
            yt_url: String::from(""),
            tab: Tabs::YTDl
        }
    } //new default file loader
    fn download_file(&self) -> Option<String> { //if couldn't dl, will return none
        println!("Downloading video at {}", self.yt_url);
        //grab metadata first then download file
        let metadata : Result<YoutubeDlOutput, youtube_dl::Error> = YoutubeDl::new(&self.yt_url)
            .socket_timeout("15")
            .run();

        if let Err(e) = metadata{
            println!("{}", e);
            println!("An error occurred when grabbing metadata. The video link may be invalid. Please try again");
            return None;
        }

        let metadata = metadata.unwrap();
        let title = metadata.into_single_video().unwrap().title.expect("Error getting video title");

        let output = YoutubeDl::new(&self.yt_url)
            .socket_timeout("15") // after 15s, a failed download will be reported
            .output_template(format!("{}.%(ext)s", title))
            .extract_audio(true)
            .extra_arg("--audio-format")
            .extra_arg("mp3")
            .download_to("music")
            .unwrap();

        println!("Downloaded {}", title);
        Some(title)
    }
    pub fn construct(&mut self, audio_player: &mut AudioPlayer, ui: &mut Ui){
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tab, Tabs::YTDl, "Ytdl");
            ui.selectable_value(&mut self.tab, Tabs::LOCAL, "Local");
        });

        match self.tab {
            Tabs::YTDl => {
                ui.text_edit_singleline(&mut self.yt_url);
                if ui.button("Download Video").clicked(){
                    let title: Option<String> = self.download_file();
                    if let Some(t) = title{
                        let path = format!("music/{}.mp3", t);
                        audio_player.path = Some(path);
                    }

                }
            }
            Tabs::LOCAL => {
                if ui.button("Open file…").clicked() { //https://github.com/emilk/egui/blob/main/examples/file_dialog/src/main.rs
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("text", &["mp3"])
                        .pick_file() {

                        audio_player.path = Some(path.display().to_string());
                        println!("selected {} from file dialog", path.display());
                    }
                }

            }
        }

        ui.label(format!("Currently loaded video: {:?}", audio_player.path));
    }
}
