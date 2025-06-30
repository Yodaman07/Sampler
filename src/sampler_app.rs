use eframe::emath::Vec2;
use egui::{Color32, CornerRadius, Id, Rect, Response, Theme, Ui, Visuals, Widget};
use egui::FontSelection::Style;
use egui::panel::Side;
use rodio::{OutputStreamHandle};
use crate::audio_player::{AudioPlayer, AudioPlayerState};
use crate::file_loader::FileLoader;

pub struct SamplerApp{
    file_loader: FileLoader,
    audio_player: AudioPlayer
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


            cc.egui_ctx.set_theme(Theme::Dark); //Keeps app in dark mode

            Ok(Box::new(SamplerApp::new(stream_handle)))
        } ),
    )
}

impl eframe::App for SamplerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if matches!(self.audio_player.audio_player_state, AudioPlayerState::PLAYING) { //ai help
            ctx.request_repaint(); // This forces egui to update continuously
        }

        ctx.style_mut(|style| {
            style.visuals.selection.bg_fill = Color32::from_rgb(131, 173, 138);
            style.visuals.extreme_bg_color =  Color32::from_rgb(217, 217, 217); //text-edit bg color


        });

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                //App content goes here
                
                self.audio_player.construct(ui); // ai help, but it is a compact way to display the audio player
                ui.add_space(10.0);
                self.file_loader.construct(&mut self.audio_player, ui);
        });

    }
}

impl SamplerApp{
    fn new( stream_handle: OutputStreamHandle) -> Self {
        Self{
            file_loader: FileLoader::new(),
            audio_player: AudioPlayer::new(stream_handle)
        }
    }


    pub fn add_rel_to_rect(ui: &mut Ui, panel: Rect, widget: impl Widget, widget_offset: Vec2, widget_size: Vec2) -> Response { //method for adding a component relative to a rect

        let widget_rect = Rect::from_min_size(
           panel.min + widget_offset, //offset is like padding
            widget_size  // width and height
        );

        ui.put(widget_rect, widget)
    }
}

