///! two dimensional vicsek model

extern crate rand;
use rand::distributions::{Normal, IndependentSample};
use rand::Rng;

use std::fs::{create_dir, File};
use std::io::prelude::*;

use std::f64::consts::PI;

struct Vicsek {
    n: u64,
    birds: Vec<Bird>,
    c_r: f64,
    eta: f64,
    rng: rand::StdRng,
}

impl Vicsek {
    fn new(n: u64) -> Vicsek {
        // { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
        let mut rng = rand::StdRng::new().unwrap();

        let mut birds = Vec::new();
        for i in 0..n {
            let theta = rng.gen::<f64>() * 2. * PI;
            let r = [rng.gen::<f64>(), rng.gen::<f64>()];
            let v0 = 0.001;
            birds.push(Bird::new(theta, r, v0));
        }

        Vicsek {
            n,
            birds,
            c_r: 0.01,
            eta: 0.1,
            rng,
        }
    }

    fn sweep(&mut self, n:u64) {
        let normal = Normal::new(0., self.eta);
        for _ in 0..n {
            // clone the birds: no borrow conflict -> synchrone update
            let cloned_birds = self.birds.clone();
            for mut b in self.birds.iter_mut() {
                let noise = normal.ind_sample(&mut self.rng);
                b.update_theta(&cloned_birds, self.c_r, noise);
                b.update_r();
            }
        }
    }

    fn save(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        write!(file, "# plot with gnuplot: p \"{}\" u 1:2:($3*40):($4*40) with vectors\n", filename);
        for b in self.birds.iter() {
            write!(file, "{} {} {} {} {}\n",
                   b.r[0],
                   b.r[1],
                   b.v0*b.theta.cos(),
                   b.v0*b.theta.sin(),
                   b.theta,
            );
        }
    }
}

#[derive(Clone)]
struct Bird {
    theta: f64,
    r: [f64; 2],
    v0: f64,
}

impl Bird {
    fn new(theta: f64, r: [f64; 2], v0: f64) -> Bird {
        Bird {
            theta,
            r,
            v0,
        }
    }

    fn update_theta(&mut self, birds: &[Bird], c_r:f64, noise: f64) {
        let c_r2 = c_r * c_r;
        let mut theta_x = 0.;
        let mut theta_y = 0.;
        for b in birds.iter() {
            let d2 = self.dist2(b);
            // also sum over yourself
            if d2 < c_r2 {
                theta_x += b.theta.cos();
                theta_y += b.theta.sin();
            }
        }
        self.theta = theta_y.atan2(theta_x) + noise;
    }

    fn update_r(&mut self) {
        self.r[0] += self.v0 * self.theta.cos();
        self.r[1] += self.v0 * self.theta.sin();

        // periodic boundaries
        if self.r[0] > 1. {
            self.r[0] -= 1.
        } else if self.r[0] < 0. {
            self.r[0] += 1.
        }
        if self.r[1] > 1. {
            self.r[1] -= 1.
        } else if self.r[1] < 0. {
            self.r[1] += 1.
        }
    }

    fn dist2(&self, other: &Bird) -> f64 {
        return (self.r[0] - other.r[0]).powi(2) + (self.r[1] - other.r[1]).powi(2)
    }
}

fn main() {
    let mut v = Vicsek::new(300);

    create_dir("data");
    create_dir("img");

    let mut file = File::create("plot.gp").unwrap();
    write!(file, "set terminal pngcairo\n");
    write!(file, "set xr [0:1]\n");
    write!(file, "set yr [0:1]\n");
    write!(file, "set size square\n");
    write!(file, "unset tics\n");

    for i in 0..500 {
        let filename = format!("data/test_{}.dat", i);
        v.save(&filename);

        write!(file, "set output 'img/test_{}.png'\n", i);
        write!(file, "p '{}'  u 1:2:($3*40):($4*40) with vectors\n", filename);

        v.sweep(20);
    }
}
