extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate fps_counter;
extern crate rand;

use std::cmp::min;

use self::graphics::*;
use self::sdl2_window::Sdl2Window as Window;
use self::piston::window::WindowSettings;
use self::piston::input::keyboard::Key::*;

use self::opengl_graphics::{GlGraphics, OpenGL};
use self::piston::event_loop::{Events, EventSettings};
use self::piston::input::{Button, Input};
use self::fps_counter::FPSCounter;

use super::vicsek_model::Vicsek;
use super::bird::Bird;

fn draw_arrow<G>(pos: (f64, f64), angle: f64, scale: f64, c: &Context, gfx: &mut G)
    where G: Graphics
{
    // TODO: get color from angle
    let color = color::hex("ffffff");
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
                        println!("{} sweeps per second", sweeps_per_second);
                    }
                    Down => {
                        sweeps_per_second /= 1.2;
                        println!("{} sweeps per second", sweeps_per_second);
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
