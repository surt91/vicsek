use std::collections::HashSet;

pub struct CellList {
    l: usize,
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

    /// return all indices contained in adjacent cells
    pub fn adjacent(&self, r: [f64; 2]) -> Vec<usize> {
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
