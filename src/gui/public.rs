use crate::{db::r#impl::Db, geo::color::ColorType};
use eframe::egui::{self, Color32, Stroke};
use egui_plot::{Plot, PlotPoints, Polygon};
pub struct ToPlot {
    pub x: Db,
    pub version: usize,
    pub min: usize,
    pub max: usize,
}

impl eframe::App for ToPlot {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut v = self.version;
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut v, 0..=600_000).text(" version"));
            });

            ui.horizontal(|ui| {
                ui.add(egui::Checkbox::new(&mut false, "hi"));
                ui.add(egui::widgets::Spinner::new());
                ui.add(egui::widgets::Label::new("hihi"));
            });

            let polygons: Vec<_> = self
                .x
                .slice(crate::db::version_controller::VersionId(v as u32))
                .rects
                .iter()
                .map(|(name, geo)| {
                    let ll = egui_plot::PlotPoint::new(geo.geo.ll.x, geo.geo.ll.y);
                    let lr = egui_plot::PlotPoint::new(geo.geo.ur.x, geo.geo.ll.y);
                    let ur = egui_plot::PlotPoint::new(geo.geo.ur.x, geo.geo.ur.y);
                    let ul = egui_plot::PlotPoint::new(geo.geo.ll.x, geo.geo.ur.y);
                    let pps = PlotPoints::Owned(vec![ll, lr, ur, ul]);
                    let mut res = Polygon::new(pps).name(String::from_utf8_lossy(name.as_ref()));
                    if let Some(c) = geo.color {
                        // res = res.fill_color(Color32::from_rgb(c.r / 4 * 3,c.g / 4 * 3,c.b / 4 * 3));
                        res = res.stroke(Stroke::new(0.0, Color32::from_rgb(c.r, c.g, c.b))).fill_color(Color32::from_rgb(c.r, c.g, c.b));
                        res = res.highlight(true);
                    };
                    res
                })
                .collect();

            let lines: Vec<_> = self
                .x
                .slice(crate::db::version_controller::VersionId(v as u32))
                .lines
                .iter()
                .map(|(name, geo)| {
                    let from = egui_plot::PlotPoint::new(geo.geo.ll.x, geo.geo.ll.y);
                    let to = egui_plot::PlotPoint::new(geo.geo.ur.x, geo.geo.ur.y);
                    let pps = PlotPoints::Owned(vec![from, to]);
                    let mut res = egui_plot::Line::new(pps).name(String::from_utf8_lossy(name.as_ref()));
                    if let Some(c) = geo.color {
                        res = res.stroke(Stroke::new(0.0, Color32::from_rgb(c.r, c.g, c.b)));
                        res = res.highlight(true);
                    };
                    res
                })
                .collect();

            Plot::new("plot")
                .allow_boxed_zoom(true)
                .show(ui, |plot_ui| {
                    for polygon in polygons {
                        plot_ui.polygon(polygon);
                    }
                    for line in lines {
                        plot_ui.line(line);
                    }
                });


                
            self.version = v;
        });
    }
}

#[derive(Default)]
pub struct Config {
    pub color: ColorType,
}

pub fn run_gui(db: Db) {
    let gui = ToPlot {
        x: db,
        version: 0,
        min: 0,
        max: 1000,
    };
    eframe::run_native(
        "geo_git",
        eframe::NativeOptions::default(),
        Box::new(|_| Box::new(gui)),
    )
    .unwrap();
}
