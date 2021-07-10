use eframe::{egui, epi};
use egui::plot::{Curve, Plot};
use json::JsonValue;

use std::process::Command;

use super::{API_KEY, DB_URL};

pub struct App {
    buffs: Vec<f32>,
}

const BUFF_LEN: usize = 128;

impl Default for App {
    fn default() -> Self {
        Self {
            buffs: Vec::default(),
        }
    }
}

impl App {
    fn update_buffers(&mut self) {
        match request_and_parse() {
            Ok(all) => {
                let skip = if all.len() > BUFF_LEN {
                    all.len() - BUFF_LEN
                } else {
                    all.len()
                };
                self.buffs = all.iter().skip(skip).map(|v| *v).collect();
            }
            Err(e) => eprintln!("Error gathering data from database: {}", e),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "SummerLab2021 CO2 visualizer"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        self.update_buffers();
        ctx.set_visuals(egui::Visuals::light()); // Switch to light mode

        egui::CentralPanel::default().show(ctx, |ui| {
            // animations ON
            ui.ctx().request_repaint();

            let curve = Curve::from_ys_f32(&self.buffs).name("co2");

            let plot = Plot::new("CO2")
                // .center_y_axis(true)
                // .height(settings.plot_height)
                .show_legend(true)
                .show_x(false)
                .show_y(false)
                .curve(curve);
            ui.add(plot);

            ui.vertical_centered(|ui| {
                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory().reset_areas();
                }
            });
        });

        // resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }

    fn setup(&mut self, ctx: &egui::CtxRef) {
        // setup fonts
        let mut fonts = egui::FontDefinitions::default();
        fonts.family_and_size.insert(
            egui::TextStyle::Small,
            (egui::FontFamily::Proportional, 16.0),
        );
        fonts.family_and_size.insert(
            egui::TextStyle::Body,
            (egui::FontFamily::Proportional, 22.0),
        );
        fonts.family_and_size.insert(
            egui::TextStyle::Button,
            (egui::FontFamily::Proportional, 24.0),
        );
        fonts.family_and_size.insert(
            egui::TextStyle::Heading,
            (egui::FontFamily::Proportional, 24.0),
        );
        ctx.set_fonts(fonts);
    }
}

fn make_request() -> Result<json::JsonValue, String> {
    let out_bytes = match Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"curl -G "{}" --data-urlencode "api_key={}""#,
            DB_URL, API_KEY
        ))
        .output()
    {
        Ok(v) => v.stdout,
        Err(e) => {
            return Err(format!(
                "Error running curl, check database URL and the API key: {}",
                e
            ));
        }
    };

    match String::from_utf8(out_bytes) {
        Ok(s) => match json::parse(&s) {
            Ok(obj) => Ok(obj),
            Err(_) => return Err("Cannot parse JSON".into()),
        },
        Err(_) => {
            return Err("Cannot format command output to utf-8 string".to_string());
        }
    }
}

fn request_and_parse() -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let mut values = vec![];
    for obj in make_request()?
        .entries()
        .skip(1)
        .next()
        .unwrap()
        .1
        .members()
    {
        if let JsonValue::Object(v) = obj {
            let val = v.get("field1").unwrap().as_str().unwrap();
            values.push(val.parse::<f32>()?)
        }
    }
    Ok(values)
}
