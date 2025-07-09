use eframe::epaint::{Color32, FontFamily, FontId, StrokeKind};
use egui::{include_image, ImageSource, Pos2, Rect, Response, Stroke, Ui, Vec2};
use random_color::RandomColor;
use crate::audio_player::{AudioPlayer, PLAYER_LEFT_OFFSET};

pub struct Marker{
    col: Color32,
    time: f32
}

pub struct Chop{
    start: Option<Marker>,
    end: Option<Marker>,
    col: Color32,
    rect: Rect,
    speed: f32,
    pitch: f32,
    volume: f32,
    fade_in: bool,
    fade_out: bool,
}

pub struct ChopEditor{
    pub chops: Vec<Chop>,
    pub selected_index: usize,
    pub(crate) play: bool
}

impl Chop{
    fn new() -> Self{
        let col = RandomColor::new().to_rgb();

        Self{
            start: None,
            end: None,
            col: Color32::from_rgb(col.r, col.g, col.b),
            rect: Rect::from_min_size(Pos2::new(0.0,0.0), Vec2::new(0.0,0.0)),
            speed: 0.0,
            pitch: 0.0,
            volume: 0.0,
            fade_in: false,
            fade_out: false,
        }
    }

    fn render(&mut self, ui: &mut Ui, surface: Rect, offset: f32){ //surface is the editor
        let mut pos: Pos2 = surface.min;
        pos.x += offset;

        let r = Rect::from_min_size(pos, Vec2::new(100.0, surface.height()));

        self.rect = r;
        let fill = Color32::from_rgba_unmultiplied(self.col.r(), self.col.g(), self.col.b(), 150);
        let stroke = Color32::from_rgba_unmultiplied(self.col.r(), self.col.g(), self.col.b(), 255);

        // let fill = Color32::from_rgba_unmultiplied(78, 141, 192, 150);
        // let stroke = Color32::from_rgba_unmultiplied(78, 141, 192, 255);

        ui.painter().rect_filled(r, 15, Color32::WHITE);
        ui.painter().rect(r, 13, fill, Stroke::new(4.0, stroke), StrokeKind::Inside);
    }
}
impl Marker{
    fn new(col: Color32, time: f32)-> Self {
        Self{ col, time }
    }
    fn draw(&self, ui: &mut Ui, audio_player: &AudioPlayer){
        let pos = audio_player.get_pos_from_time(Some(self.time));
        ui.painter().rect_filled(Rect::from_two_pos(egui::pos2(PLAYER_LEFT_OFFSET + pos, 10.0), egui::pos2(PLAYER_LEFT_OFFSET + 4.0 + pos, 75.0)), 10, self.col); //height is 65.0
    }
}

fn new_btn(ui: &mut Ui, name: impl Into<String>, padding: impl Into<Vec2>, pos: Pos2) -> Response{
    let font = FontId::new(14.0, FontFamily::default());
    let button = egui::Button::new(egui::RichText::new(name).font(font).strong())
        .corner_radius(13)
        .fill(Color32::from_rgb(60,60,60));
    //max size is like padding
    ui.put(Rect::from_min_size(pos, padding.into()), button)
}

fn new_img_btn(ui: &mut Ui, img: ImageSource, size: impl Into<Vec2>, pos: Pos2) -> Response{
    ui.put(Rect::from_min_size(pos, size.into()), egui::Button::image(img).corner_radius(13))
}
impl ChopEditor{

    fn play_chop(&mut self, audio_player: &mut AudioPlayer){

        let current_time = audio_player.current_time;
        let chop: &Chop = &self.chops[self.selected_index];


        if current_time >= chop.end.as_ref().unwrap().time{
            audio_player.skip_to(chop.start.as_ref().unwrap().time);
        }
    }


    pub fn construct(&mut self, ui: &mut Ui, audio_player: &mut AudioPlayer){
        if !self.chops.is_empty() {
            let chop: &Chop = &self.chops[self.selected_index];

            if let Some(c) = &chop.start {c.draw(ui, &audio_player);} //renders the markers if they exist
            if let Some(c) = &chop.end {c.draw(ui, &audio_player);}
        }

        ui.horizontal(|ui|{
            ui.add_space(300.0);

            let start_btn = new_btn(ui,"Place Start Marker", [140.0, 30.0], Pos2::new(210.0, 110.0));

            let left = new_img_btn(ui, include_image!("../imgs/left_arrow.svg"), [32.0,32.0], Pos2::new(210.0+20.0, 150.0));
            let mini_play = new_img_btn(ui, include_image!("../imgs/pause.svg"), [40.0,40.0], Pos2::new(230.0+34.0, 150.0));
            let right = new_img_btn(ui, include_image!("../imgs/right_arrow.svg"), [32.0,32.0], Pos2::new(318.0-20.0, 150.0));

            let end_btn = new_btn(ui,"Place End Marker", [140.0, 30.0], Pos2::new(210.0, 190.0));

            if self.play{
                self.play_chop(audio_player);
            }

            if let Some(sink) = audio_player.sink.as_ref() {
                let chops = &mut self.chops;
                if chops.is_empty() { println!("Please select a chop to modify before continuing") } 
                else {
                    let current = audio_player.current_time;

                    if start_btn.clicked() { chops[self.selected_index].start = Some(Marker::new(chops[self.selected_index].col, sink.get_pos().as_secs_f32())) }
                    if end_btn.clicked() { chops[self.selected_index].end = Some(Marker::new(chops[self.selected_index].col, sink.get_pos().as_secs_f32())) };

                    if left.clicked() {
                        if current > 1.0 {
                            audio_player.skip_to(current - 1.0);
                        } else if current > 0.0 {
                            audio_player.skip_to(0.0); //reset at 0
                        }
                    }

                    if right.clicked() {
                        if current < (audio_player.playback_time - 1.0) {
                            audio_player.skip_to(current + 1.0);
                        }
                    }

                    if mini_play.clicked() {
                        audio_player.sink.as_ref().unwrap().play();
                        self.play = true;
                    }

                }
            }

        });

        ui.add_space(100.0);
        if new_btn(ui,"New Chop", [140.0, 30.0], Pos2::new(10.0,310.0)).clicked(){
            if audio_player.path.is_none(){
                println!("Please load a song before continuing")
            }else { self.chops.push(Chop::new()); }
        }

        new_btn(ui,"Color: ", [140.0, 30.0], Pos2::new(10.0,345.0)); //just display for now
        let chop_timeline = Rect::from_two_pos(egui::pos2(175.0, 310.0), egui::pos2(650.0 + 175.0, 375.0));
        ui.painter().rect_filled(chop_timeline, 25, Color32::DARK_GRAY);
        let resp = ui.allocate_rect(chop_timeline, egui::Sense::CLICK);
        if resp.clicked(){ //clicked anywhere on the screen
            for (index,chop) in self.chops.iter().enumerate()  {
                let resp_rect = Rect::from_min_size(resp.interact_pointer_pos().unwrap(), Vec2::new(2.0,2.0));
                if chop.rect.contains_rect(resp_rect){
                    self.selected_index = index;
                    println!("Selected Chop: {}", index);
                    // println!("Size: {:?}", resp_rect)
                }
            }
        }
        let mut offset = 0.0;
        for chop in &mut self.chops{
            chop.render(ui, chop_timeline, offset);
            offset+=200.0;
        }
    }
}