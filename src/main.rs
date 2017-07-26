///! two dimensional vicsek model

extern crate rand;
use rand::distributions::{Normal, IndependentSample};
use rand::Rng;

use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io;

use std::collections::HashSet;

use std::f64::consts::PI;

struct CellList {
    l: usize,
    list: Vec<HashSet<usize>>,
}

impl CellList {
    fn new(l: usize) -> CellList {
        let list = vec![HashSet::new(); l.pow(2)];

        CellList {
            l,
            list,
        }
    }

    fn add(&mut self, r: [f64; 2], idx: usize) {
        let x = (r[0] * self.l as f64) as usize;
        let y = (r[1] * self.l as f64) as usize;
        self.list[x*self.l + y].insert(idx);
    }

    fn remove(&mut self, r: [f64; 2], idx: usize) {
        let x = (r[0] * self.l as f64) as usize;
        let y = (r[1] * self.l as f64) as usize;
        self.list[x*self.l + y].remove(&idx);
    }

    /// return all indices contained in adjacent cells
    fn adjacent(&self, r: [f64; 2]) -> Vec<usize> {
        let mut tmp = Vec::new();
        let x_idx = (r[0] * self.l as f64) as i64;
        let y_idx = (r[1] * self.l as f64) as i64;
        let l = self.l as i64;
        for mut x in (x_idx-1)..(x_idx+1) {
            if x < 0 {
                x = l + x
            }
            if x >= l  {
                x = x - l;
            }
            for mut y in (y_idx-1)..(y_idx+1) {
                if y < 0 {
                    y = l + y
                }
                if y >= l  {
                    y = y - l;
                }
                for idx in self.list[(x*l + y) as usize].clone() {
                    tmp.push(idx);
                }
            }
        }
        tmp
    }

    fn print(&self) {
        for i in 0..self.l {
            for j in 0..self.l {
                print!("[");
                for x in self.list[i*self.l + j].iter() {
                    print!("{}, ", x);
                }
                print!("], ");
            }
            print!("\n");
        }
        print!("\n");
        print!("\n");
    }
}

struct Vicsek {
    birds: Vec<Bird>,
    c_r: f64,
    eta: f64,
    rng: rand::StdRng,
    cell_list: CellList, //< cell list with indecies of the birds
}

impl Vicsek {
    fn new(n: u64) -> Vicsek {
        // { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
        let mut rng = rand::StdRng::new().unwrap();
        let c_r = 0.01;
        let l = (1./c_r) as usize;
        let mut cell_list = CellList::new(l);

        let mut birds = Vec::new();
        for idx in 0..n as usize {
            let theta = rng.gen::<f64>() * 2. * PI;
            let r = [rng.gen::<f64>(), rng.gen::<f64>()];
            let v0 = 0.001;
            birds.push(Bird::new(theta, r, v0));
            cell_list.add(r, idx);
        }

        Vicsek {
            birds,
            c_r,
            eta: 0.1,
            rng,
            cell_list,
        }
    }

    fn sweep(&mut self, n:u64) {
        let normal = Normal::new(0., self.eta);
        for _ in 0..n {
            // clone the birds: no borrow conflict -> synchrone update
            let cloned_birds = self.birds.clone();
            // TODO: this loop can be parallized by rayon
            for (idx, mut b) in self.birds.iter_mut().enumerate() {
                let noise = normal.ind_sample(&mut self.rng);

                let mut candidates = Vec::new();
                for i in self.cell_list.adjacent(b.r).iter() {
                    candidates.push(cloned_birds[*i].clone());
                }

                b.update_theta(&candidates, self.c_r, noise);

                // remove from cell list before update
                self.cell_list.remove(b.r, idx);
                b.update_r();
                // add to new cell after update
                self.cell_list.add(b.r, idx);
            }
        }
    }

    fn save(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename).unwrap();
        write!(file, "# plot with gnuplot: p \"{}\" u 1:2:($3*40):($4*40) with vectors\n", filename)?;
        for b in self.birds.iter() {
            write!(file, "{} {} {} {} {}\n",
                   b.r[0],
                   b.r[1],
                   b.v0*b.theta.cos(),
                   b.v0*b.theta.sin(),
                   b.theta,
            )?;
        }
        Ok(())
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

    /// return squared eucledian dist respecting periodic boundaries
    fn dist2(&self, other: &Bird) -> f64 {
        // take the image bird nearest to you
        let dx = self.dist_x(other);
        let dy = self.dist_y(other);
        // since the space is 1x1, just take min(x, x-1) for periodic boundaries
        let dx = if dx < (dx-1.).abs() {dx} else {dx-1.};
        let dy = if dy < (dy-1.).abs() {dy} else {dy-1.};

        dx.powi(2) + dy.powi(2)
    }

    fn dist_x(&self, other: &Bird) -> f64 {
        (self.r[0] - other.r[0])
    }

    fn dist_y(&self, other: &Bird) -> f64 {
        (self.r[1] - other.r[1])
    }
}

// TODO pass arguments
fn run() -> io::Result<()> {
    let mut v = Vicsek::new(300);

    create_dir_all("data")?;
    create_dir_all("img")?;

    let mut file = File::create("plot.gp")?;
    write!(file, "set terminal pngcairo size 1080, 1080\n")?;
    write!(file, "set xr [0:1]\n")?;
    write!(file, "set yr [0:1]\n")?;
    write!(file, "set size square\n")?;
    write!(file, "unset tics\n")?;
    write!(file, "unset key\n")?;
    write!(file, "set style arrow 1 head filled size screen 0.025, 30, 45 ls 1\n")?;

    for i in 0..500 {
        let filename = format!("data/test_{:04}.dat", i);
        v.save(&filename)?;

        write!(file, "set output 'img/test_{:04}.png'\n", i)?;
        write!(file, "p '{}'  u 1:2:($3*40):($4*40) with vectors arrowstyle 1\n", filename)?;

        v.sweep(20);
    }

    // TODO call the gnuplot script in parallel
    Ok(())
}

fn main() {
    // TODO CLAP
    run().expect("IO error");
}
