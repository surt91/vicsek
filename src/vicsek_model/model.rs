use itertools::{self, Itertools};
use rand::prelude::*;
use rand_distr::Normal;
use rand_pcg::Pcg64;

use std::cmp;
use std::f32::consts::PI;

use super::bird::Bird;
use super::cell_list::CellList;
use super::proximity::Proximity;

pub struct Vicsek {
    pub birds: Vec<Bird>,
    pub proximity: Proximity,
    pub eta: f64,
    rng: Pcg64,
    cell_list: CellList, //< cell list with indicies of the birds
}

impl Vicsek {
    pub fn new(n: usize, proximity: Proximity, seed: u64) -> Vicsek {
        // let x = 1;
        // let mut rng: rand::StdRng = { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) };
        let rng = Pcg64::seed_from_u64(seed);
        let l = (n as f32).sqrt() as usize;
        let cell_list = CellList::new(l);

        let birds = Vec::new();

        let mut vicsek = Vicsek {
            birds,
            proximity,
            eta: 0.1,
            rng,
            cell_list,
        };

        for _ in 0..n {
            vicsek.add_bird();
        }

        vicsek
    }

    fn add_bird(&mut self) {
        let theta = self.rng.gen::<f32>() * 2. * PI;
        let v = [theta.cos(), theta.sin()];
        let r = [self.rng.gen::<f32>(), self.rng.gen::<f32>()];
        let v0 = 0.001;
        self.cell_list.add(r, self.birds.len());
        self.birds.push(Bird::new(r, v, v0));
    }

    pub fn set_num_birds(&mut self, n: usize) {
        match n.cmp(&self.birds.len()) {
            cmp::Ordering::Equal => (),
            cmp::Ordering::Greater => (self.birds.len()..n).for_each(|_| self.add_bird()),
            cmp::Ordering::Less => {
                self.birds.truncate(n);
                self.recreate_cell_list();
            }
        }
    }

    fn update_direction_neighbors(&self, b: &mut Bird, neighbors: usize, noise: [f32; 2]) {
        let mut candidates = Vec::new();
        let mut level = 0;
        'outer: loop {
            for i in self.cell_list.adjacent_level(b, level).iter().sorted() {
                candidates.push(self.birds[*i].clone());
                if candidates.len() >= neighbors {
                    break 'outer;
                }
            }
            level += 1;
        }

        b.update_direction(&candidates, noise);
    }

    fn update_direction_disk(&self, b: &mut Bird, r: f32, noise: [f32; 2]) {
        let r2 = r * r;
        let mut candidates = Vec::new();
        let level = self.cell_list.distance_to_level(r);
        for i in &self.cell_list.adjacent_level(b, level) {
            if self.birds[*i].dist2(b) < r2 {
                candidates.push(self.birds[*i].clone());
            }
        }

        b.update_direction(&candidates, noise);
    }

    fn recreate_cell_list(&mut self) {
        self.cell_list.clear();
        for (idx, b) in self.birds.iter().enumerate() {
            self.cell_list.add(b.r, idx)
        }
    }

    pub fn sweep(&mut self, n: u64) {
        let normal = Normal::new(0., self.eta).expect("failed to build a normal distribution");
        for _ in 0..n {
            // clone the birds: no borrow conflict -> synchrone update
            let mut cloned_birds = self.birds.clone();

            for b in &mut cloned_birds.iter_mut() {
                let noise = [
                    normal.sample(&mut self.rng) as f32,
                    normal.sample(&mut self.rng) as f32,
                ];

                match self.proximity {
                    Proximity::Neighbors(n) => self.update_direction_neighbors(b, n, noise),
                    Proximity::Radius(r) => self.update_direction_disk(b, r, noise),
                }

                b.update_r();
            }
            self.birds = cloned_birds;

            self.recreate_cell_list();
        }
    }
}
