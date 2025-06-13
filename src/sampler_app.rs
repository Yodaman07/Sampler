use rodio::{OutputStream, OutputStreamHandle, Sink};

use crate::Song;

struct SamplerApp{
    stream_handle: OutputStreamHandle,
    sink: Option<Sink>
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
        Box::new(|cc| Ok(Box::new(SamplerApp::new(stream_handle))) ),
    )
}



impl eframe::App for SamplerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Awesome Sampler");
            //App content goes here

            if ui.button("Play").clicked(){

                println!("A");
                //base song, can make modifications via clips

                match &self.sink{
                    None => {
                        let sink = Sink::try_new(&self.stream_handle).expect("Couldn't make sink");

                        let dtmlive: Song = Song{ path: String::from("music/nikes.mp3"), };
                        let clip1 = dtmlive.clip(1.2, 6.0, 10.0, false);
                        //Sink is like the audio player

                        sink.append(clip1);
                        self.sink = Some(sink);

                    }
                    Some(sink) => {
                        sink.pause();
                    }
                }


            }

        });
    }
}

impl SamplerApp{
    fn new( stream_handle: OutputStreamHandle) -> Self {
        Self{
            stream_handle,
            sink: None
        }
    }

}
