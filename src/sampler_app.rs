use egui::{Color32, CornerRadius, Id};
use egui::panel::Side;
use rodio::{OutputStreamHandle};
use crate::audio_player::{AudioPlayer, AudioPlayerState};
use crate::file_loader::FileLoader;

struct SamplerApp{
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
            Ok(Box::new(SamplerApp::new(stream_handle)))
        } ),
    )
}

impl eframe::App for SamplerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if matches!(self.audio_player.audio_player_state, AudioPlayerState::PLAYING) { //ai help
            ctx.request_repaint(); // This forces egui to update continuously
        }


        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style())
                .fill(Color32::from_rgb(27, 27, 27)))//ai help

            .show(ctx, |ui| {
                //App content goes here

                // self.audio_player.construct(ui); // ai help, but it is a compact way to display the audio player
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
}
