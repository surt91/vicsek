extern crate rand;
use rand::distributions::{Normal, IndependentSample};
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufWriter};

use std::f64::consts::PI;

use bird::Bird;
use cell_list::CellList;


pub struct Vicsek {
    pub birds: Vec<Bird>,
    neighbors: usize,
    eta: f64,
    rng: rand::StdRng,
    cell_list: CellList, //< cell list with indecies of the birds
}

impl Vicsek {
    pub fn new(n: u64, neighbors: usize) -> Vicsek {
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
            neighbors,
            eta: 0.1,
            rng,
            cell_list,
        }
    }

    pub fn sweep(&mut self, n:u64) {
        let normal = Normal::new(0., self.eta);
        // self.cell_list.print();
        for _ in 0..n {
            // clone the birds: no borrow conflict -> synchrone update
            let cloned_birds = self.birds.clone();
            // TODO: this loop can be parallized by rayon
            for (idx, mut b) in self.birds.iter_mut().enumerate() {
                let noise = [normal.ind_sample(&mut self.rng), normal.ind_sample(&mut self.rng)];

                let mut candidates = Vec::new();
                let mut level = 0;
                'outer: loop {
                    for i in self.cell_list.adjacent_level(b, level, &cloned_birds).iter() {
                        candidates.push(cloned_birds[*i].clone());
                        if candidates.len() >= self.neighbors {
                            break 'outer
                        }
                    }
                    level += 1;
                }

                b.update_direction(&candidates, noise);

                // remove from cell list before update
                self.cell_list.remove(b.r, idx);
                b.update_r();
                // add to new cell after update
                self.cell_list.add(b.r, idx);
            }
        }
    }

    pub fn save(&self, filename: &str) -> io::Result<()> {
        let mut file = BufWriter::new(File::create(filename).unwrap());
        for b in self.birds.iter() {
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
