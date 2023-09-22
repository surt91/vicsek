use std::f32::consts::PI;

use egui::{self, Pos2};

/// convert a hsv color representations into a rgb representation
pub fn hsv2rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    // https://de.wikipedia.org/wiki/HSV-Farbraum#Umrechnung_HSV_in_RGB
    let hi = (h * 6.).floor() as u32;
    let f = h * 6. - hi as f32;
    let p = v * (1. - s);
    let q = v * (1. - s * f);
    let t = v * (1. - s * (1. - f));

    match hi {
        0 | 6 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => (0., 0., 0.),
    }
}

pub fn angle_to_color(angle: f32) -> egui::Color32 {
    let (r, g, b) = hsv2rgb((angle + 2. * PI) % (2. * PI) / PI / 2., 1., 1.);
    egui::Color32::from_rgb((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8)
}

pub fn transform(pos: Pos2, angle: f32, scale: f32) -> Pos2 {
    let Pos2 { x, y } = pos;
    Pos2::new(
        (x * angle.cos() - y * angle.sin()) * scale,
        (x * angle.sin() + y * angle.cos()) * scale,
    )
}
