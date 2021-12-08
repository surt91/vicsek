extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate fps_counter;
extern crate rand;

use std::cmp::min;
use std::f64::consts::PI;

use self::graphics::*;
use self::glutin_window::GlutinWindow as Window;
use self::piston::window::WindowSettings;
use self::piston::input::keyboard::Key::*;

use self::opengl_graphics::{GlGraphics, OpenGL};
use self::piston::event_loop::{Events, EventSettings};
use self::piston::input::{Button, Input};
use self::fps_counter::FPSCounter;

use super::vicsek_model::Vicsek;
use super::bird::Bird;

/// convert a hsv color representations into a rgb representation
fn hsv2rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    // https://de.wikipedia.org/wiki/HSV-Farbraum#Umrechnung_HSV_in_RGB
    let hi = (h * 6.).floor() as u32;
    let f = h * 6. - hi as f64;
    let p = v*(1.-s);
    let q = v*(1.-s*f);
    let t = v*(1.-s*(1.-f));

    match hi {
        0 | 6 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => (0., 0., 0.)
    }
}

fn angle_to_color(angle: f64) -> types::Color {
    let (r, g, b) = hsv2rgb((angle + 2.*PI)%(2.*PI)/PI/2., 1., 1.);
    [r as f32, g as f32, b as f32, 1.]
}

fn draw_arrow<G>(pos: (f64, f64), angle: f64, scale: f64, c: &Context, gfx: &mut G)
    where G: Graphics
{
    // TODO: get color from angle
    let color = angle_to_color(angle);
    let trafo = c.transform.trans(pos.0, pos.1).rot_rad(angle).scale(scale, scale);

    let p1: types::Polygon = &[
        [0.,  1.],
        [2.,  0.],
        [0., -1.],
    ];
    let p2: types::Polygon = &[
        [ 0., 0. ],
        [ 0., 1. ],
        [-1., 1.5],
    ];
    let p3: types::Polygon = &[
        [ 0.,  0. ],
        [ 0., -1. ],
        [-1., -1.5],
    ];
    polygon(color, p1, trafo, gfx);
    polygon(color, p2, trafo, gfx);
    polygon(color, p3, trafo, gfx);
}

pub fn show(size: (u32, u32), vicsek: &mut Vicsek) {
    let mut window: Window = WindowSettings::new("Vicsek", [size.0, size.1])
                                            .samples(4)
                                            .exit_on_esc(true)
                                            .decorated(false)
                                            .srgb(false)
                                            .build()
                                            .unwrap();

    let mut gfx = GlGraphics::new(OpenGL::V3_2);
    let mut fps = FPSCounter::new();
    let mut sweeps_per_second = 100.;
    let mut rate = 0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(args) => {
                rate = fps.tick();

                gfx.draw(args.viewport(), |c, gfx| {
                    vicsek.render(&c, gfx, &size);
                });
            }

            Input::Press(Button::Keyboard(key)) => {
                match key {
                    F => println!("{} FPS", rate),
                    Up => {
                        sweeps_per_second *= 1.2;
                        println!("{:.0} sweeps per second", sweeps_per_second);
                    }
                    Down => {
                        sweeps_per_second /= 1.2;
                        println!("{:.0} sweeps per second", sweeps_per_second);
                    }
                    _ => ()
                };
            }

            Input::Update(args) => {
                vicsek.sweep((args.dt * sweeps_per_second).ceil() as u64);
            }

            _ => {}
        }
    }
}

pub trait Renderable {
    fn render<G>(&self, c: &Context, gfx: &mut G, size: &(u32, u32))
        where G: Graphics;
}

impl Renderable for Vicsek {
    fn render<G>(&self, c: &Context, gfx: &mut G, size: &(u32, u32))
        where G: Graphics
    {
        clear(color::hex("000000"), gfx);

        for b in &self.birds {
            b.render(c, gfx, size);
        }
    }
}

impl Renderable for Bird {
    fn render<G>(&self, c: &Context, gfx: &mut G, size: &(u32, u32))
        where G: Graphics
    {
        let size = min(size.0, size.1) as f64;
        let scale = 3.;

        let pos = (self.r[0] * size, self.r[1] * size);
        let angle= self.v[1].atan2(self.v[0]);

        draw_arrow(pos, angle, scale, c, gfx);
    }
}
