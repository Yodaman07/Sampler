use std::fmt::{Debug, Formatter, Pointer};
use eframe::emath::Align;
use egui::{Align2, Color32, FontFamily, FontId, Pos2, Rect, Shape, Ui, Vec2};
use egui::Shape::Vec;
use egui::WidgetText::RichText;
use egui::WidgetType::TextEdit;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};
use crate::audio_player::AudioPlayer;
use crate::sampler_app::SamplerApp;

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

        let p_x = 8.0; //panel x and panel y
        let p_y = 109.0;
        let panel = Rect::from_two_pos(Pos2::new(p_x, p_y), Pos2::new(p_x+194.0, p_y+173.0)); //194 is width, and 173 is height
        ui.painter().rect_filled(panel, 18, Color32::from_rgb(71,71,71));

        ui.add_space(109.0);

        //make the selectable laels we will add later
        let ytdl_label = egui::SelectableLabel::new(
            self.tab == Tabs::YTDl,
            egui::RichText::new("YTDL").color(Color32::WHITE)
        );
        let local_label =  egui::SelectableLabel::new(
            self.tab == Tabs::LOCAL,
            egui::RichText::new("Local").color(Color32::WHITE)
        );


        SamplerApp::add_rel_to_rect(ui, panel, ytdl_label, Vec2::new(10.0, 10.0), Vec2::new(80.0, 15.0))
            .clicked().then(|| self.tab = Tabs::YTDl);

        SamplerApp::add_rel_to_rect(ui, panel, local_label, Vec2::new(100.0, 10.0), Vec2::new(80.0, 15.0))
            .clicked().then(|| self.tab = Tabs::LOCAL);



        match self.tab {
            Tabs::YTDl => {
                let edit =egui::TextEdit::singleline(&mut self.yt_url)
                    .desired_width(157.0)
                    .text_color(egui::Color32::BLACK);

                SamplerApp::add_rel_to_rect(ui, panel, edit, Vec2::new(18.5, 40.0), Vec2::new(157.0, 20.0));
                let download_btn = SamplerApp::add_rel_to_rect(ui, panel, egui::Button::new("Download Video"), Vec2::new(57.0, 80.0),  Vec2::new(80.0, 20.0));
                //btn is 63 x 30


                if download_btn.clicked(){
                    let title: Option<String> = self.download_file();
                    if let Some(t) = title{
                        let path = format!("music/{}.mp3", t);
                        audio_player.path = Some(path);
                        audio_player.startup();
                    }

                }
            }
            Tabs::LOCAL => {
                let open_file_btn = SamplerApp::add_rel_to_rect(ui, panel, egui::Button::new("Open File"), Vec2::new(57.0, 40.0), Vec2::new(80.0, 20.0));
                if open_file_btn.clicked() { //https://github.com/emilk/egui/blob/main/examples/file_dialog/src/main.rs
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("text", &["mp3"])
                        .pick_file() {

                        audio_player.path = Some(path.display().to_string());
                        audio_player.startup();
                        println!("selected {} from file dialog", path.display());
                    }
                }

            }
        }

        let font = FontId::new(10.0, FontFamily::default());

        //10 pixel paddinga
        ui.painter().text(panel.center_bottom()+Vec2::new(0.0, -10.0), Align2::CENTER_BOTTOM, format!("Currently Loaded Track: {:?}", audio_player.path),font, Color32::WHITE);
    }
}
