use std::time::Duration;

use eframe::epaint::PathShape;
use egui::{pos2, Shape, Stroke};

use crate::{
    utils::{angle_to_color, transform},
    vicsek_model::{Proximity, Vicsek},
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct VicsekApp {
    radius: f32,
    num_neighbors: usize,
    num_birds: usize,

    #[serde(skip)]
    vicsek: Vicsek,
}

impl Default for VicsekApp {
    fn default() -> Self {
        let radius = 0.05;
        let num_neighbors = 5;
        let num_birds = 100;
        let vicsek = Vicsek::new(
            num_birds,
            crate::vicsek_model::Proximity::Radius(radius),
            42,
        );

        Self {
            num_birds,
            num_neighbors,
            radius,
            vicsek,
        }
    }
}

impl VicsekApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for VicsekApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            num_birds,
            num_neighbors,
            radius,
            vicsek,
        } = self;

        // A window to change the parameters of the model
        egui::Window::new("Parameters").show(ctx, |ui| {
            ui.add(egui::Slider::new(num_birds, 12..=1000).text("number of birds"));
            vicsek.set_num_birds(*num_birds);

            ui.add(egui::Slider::new(&mut vicsek.eta, 0.0..=1.0).text("noise"));

            ui.horizontal_wrapped(|ui| {
                ui.radio_value(
                    &mut vicsek.proximity,
                    Proximity::Neighbors(*num_neighbors),
                    "number of neighbors",
                );
                ui.radio_value(&mut vicsek.proximity, Proximity::Radius(*radius), "radius");
            });

            match &mut vicsek.proximity {
                Proximity::Radius(r) => {
                    ui.add(egui::Slider::new(r, 0.01..=1.).text("radius"));
                    *radius = *r;
                }
                Proximity::Neighbors(n) => {
                    ui.add(egui::Slider::new(n, 2..=10).text("number of neighbors"));
                    *num_neighbors = *n;
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut painter_size = ui.available_size_before_wrap();
            if !painter_size.is_finite() {
                painter_size = egui::vec2(500.0, 500.0);
            }
            let size = if painter_size.x < painter_size.y {
                painter_size.x
            } else {
                painter_size.y
            };
            let scale = size / 200.;

            let (_res, painter) = ui.allocate_painter(painter_size, egui::Sense::hover());

            // update position of all birds
            vicsek.sweep(1);

            ui.ctx()
                .request_repaint_after(Duration::new(0, (1. / 60. * 1e9) as u32));

            for bird in &vicsek.birds {
                let position = egui::Pos2::new(bird.r[0] * size, bird.r[1] * size);
                let direction = egui::vec2(bird.v[0], bird.v[1]);
                let angle = direction.angle();
                let color = angle_to_color(angle);

                let points = vec![
                    transform(pos2(1., 0.), angle, scale),
                    transform(pos2(-1., -1.), angle, scale),
                    transform(pos2(-1., 1.), angle, scale),
                ];
                let path = PathShape::convex_polygon(points, color, Stroke::default());
                let mut path = Shape::Path(path);
                path.translate(position.to_vec2());

                painter.add(path);
            }

            egui::warn_if_debug_build(ui);
        });
    }
}
