use std::collections::HashSet;

use super::bird::Bird;

pub(crate) struct CellList {
    pub l: usize,
    list: Vec<HashSet<usize>>,
}

impl CellList {
    pub fn new(l: usize) -> CellList {
        let list = vec![HashSet::new(); l.pow(2)];

        CellList { l, list }
    }

    pub(crate) fn add(&mut self, r: [f32; 2], idx: usize) {
        // the clamping avoids problems if x or y are exactly 1.0
        let x = ((r[0] * self.l as f32) as usize).clamp(0, self.l - 1);
        let y = ((r[1] * self.l as f32) as usize).clamp(0, self.l - 1);
        self.list[x * self.l + y].insert(idx);
    }

    pub(crate) fn clear(&mut self) {
        self.list = vec![HashSet::new(); self.l.pow(2)];
    }

    pub(crate) fn adjacent_level(&self, bird: &Bird, n: i64) -> Vec<usize> {
        let r = bird.r;
        let mut tmp = Vec::new();
        let x_idx = (r[0] * self.l as f32) as i64;
        let y_idx = (r[1] * self.l as f32) as i64;
        let l = self.l as i64;
        for mut x in (x_idx - n)..=(x_idx + n) {
            if x < 0 {
                x += l
            }
            if x >= l {
                x -= l;
            }
            for mut y in (y_idx - n)..=(y_idx + n) {
                if y < 0 {
                    y += l
                }
                if y >= l {
                    y -= l;
                }
                for idx in self.list[(x * l + y) as usize].clone() {
                    tmp.push(idx);
                }
            }
        }

        tmp
    }

    pub(crate) fn distance_to_level(&self, r: f32) -> i64 {
        (r / self.l as f32).ceil() as i64 + 1
    }
}
