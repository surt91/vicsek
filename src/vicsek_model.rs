extern crate rand;
use rand::distributions::{Normal, IndependentSample};
use rand::{thread_rng, Rng};

extern crate rayon;
use self::rayon::prelude::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufWriter};

use std::f64::consts::PI;

use bird::Bird;
use cell_list::CellList;
use parse_cl::Proximity;

pub struct Vicsek {
    pub birds: Vec<Bird>,
    proximity: Proximity,
    eta: f64,
    rng: rand::StdRng,
    cell_list: CellList, //< cell list with indecies of the birds
}

impl Vicsek {
    pub fn new(n: u64, proximity: Proximity) -> Vicsek {
        // let x = 1;
        // let mut rng: rand::StdRng = { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) };
        let mut rng = rand::StdRng::new().unwrap();
        let l = (n as f64).sqrt() as usize;
        let mut cell_list = CellList::new(l);

        let mut birds = Vec::new();
        for idx in 0..n as usize {
            let theta = rng.gen::<f64>() * 2. * PI;
            let v = [theta.cos(), theta.sin()];
            let r = [rng.gen::<f64>(), rng.gen::<f64>()];
            let v0 = 0.001;
            birds.push(Bird::new(r, v, v0));
            cell_list.add(r, idx);
        }

        Vicsek {
            birds,
            proximity,
            eta: 0.1,
            rng,
            cell_list,
        }
    }

    fn update_direction_neighbors(&self, b: &mut Bird, neighbors: usize, noise: [f64; 2]) {
        let mut candidates = Vec::new();
        let mut level = 0;
        'outer: loop {
            for i in &self.cell_list.adjacent_level(b, level, &self.birds) {
                candidates.push(self.birds[*i].clone());
                if candidates.len() >= neighbors {
                    break 'outer
                }
            }
            level += 1;
        }

        b.update_direction(&candidates, noise);
    }

    fn update_direction_disk(&self, b: &mut Bird, r: f64, noise: [f64; 2]) {
        let r2 = r*r;
        let mut candidates = Vec::new();
        let max_level = (r / self.cell_list.l as f64).ceil() as i64 + 1;
        for level in 0..max_level {
            for i in &self.cell_list.adjacent_level(b, level, &self.birds) {
                if self.birds[*i].dist2(b) < r2 {
                    candidates.push(self.birds[*i].clone());
                }
            }
        }

        b.update_direction(&candidates, noise);
    }

    pub fn sweep(&mut self, n:u64) {
        let normal = Normal::new(0., self.eta);
        for _ in 0..n {
            // clone the birds: no borrow conflict -> synchrone update
            let mut cloned_birds = self.birds.clone();

            cloned_birds.par_iter_mut().for_each(|mut b| {
                let noise = [normal.ind_sample(&mut thread_rng()), normal.ind_sample(&mut thread_rng())];

                match self.proximity {
                    Proximity::Neighbors(n) => self.update_direction_neighbors(b, n, noise),
                    Proximity::Radius(r) => self.update_direction_disk(b, r, noise),
                }

                b.update_r();
            });
            self.birds = cloned_birds;

            // recreate Cell list
            self.cell_list.clear();
            for (idx, b) in self.birds.iter().enumerate() {
                self.cell_list.add(b.r, idx)
            }
        }
    }

    pub fn save(&self, filename: &str) -> io::Result<()> {
        let mut file = BufWriter::new(File::create(filename).unwrap());
        for b in &self.birds {
            write!(file, "{:.4} {:.4} {:.4} {:.4} {:.4}\n",
                   b.r[0],
                   b.r[1],
                   b.v[0],
                   b.v[1],
                   b.v[1].atan2(b.v[0]),
            )?;
        }
        Ok(())
    }
}
