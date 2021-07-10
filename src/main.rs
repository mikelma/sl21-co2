// use eframe::{egui, epi};

use sl21_co2::App;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(App::default()), options);
}
