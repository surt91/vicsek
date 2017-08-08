use std::collections::HashSet;
use std::cmp::{max, Ordering};

use super::bird::Bird;

pub struct CellList {
    pub l: usize,
    list: Vec<HashSet<usize>>,
}

impl CellList {
    pub fn new(l: usize) -> CellList {
        let list = vec![HashSet::new(); l.pow(2)];

        CellList {
            l,
            list,
        }
    }

    pub fn add(&mut self, r: [f64; 2], idx: usize) {
        let x = (r[0] * self.l as f64) as usize;
        let y = (r[1] * self.l as f64) as usize;
        self.list[x*self.l + y].insert(idx);
    }

    pub fn remove(&mut self, r: [f64; 2], idx: usize) {
        let x = (r[0] * self.l as f64) as usize;
        let y = (r[1] * self.l as f64) as usize;
        self.list[x*self.l + y].remove(&idx);
    }

    pub fn clear(&mut self) {
        self.list = vec![HashSet::new(); self.l.pow(2)];
    }

    // TODO make into an iterator
    pub fn adjacent_level(&self, bird: &Bird, n: i64, pos: &[Bird]) -> Vec<usize>
    {
        let r = bird.r;
        let mut tmp = Vec::new();
        let x_idx = (r[0] * self.l as f64) as i64;
        let y_idx = (r[1] * self.l as f64) as i64;
        let l = self.l as i64;
        for mut x in (x_idx-n)..(x_idx+n+1) {
            if x < 0 {
                x += l
            }
            if x >= l  {
                x -= l;
            }
            for mut y in (y_idx-n)..(y_idx+n+1) {
                if y < 0 {
                    y += l
                }
                if y >= l  {
                    y -= l;
                }
                // only use, if maximum norm is equal n
                if max((x-x_idx).abs(), (y-y_idx).abs()) == n {
                    for idx in self.list[(x*l + y) as usize].clone() {
                        tmp.push(idx);
                    }
                }
            }
        }
        // sort by distance to r
        tmp.sort_by(|&a, &b| pos[a].dist2(bird).partial_cmp(&pos[b].dist2(bird)).unwrap_or(Ordering::Greater));
        tmp
    }

    pub fn print(&self) {
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
