#[derive(Clone)]
pub struct Bird {
    pub r: [f64; 2],
    pub v: [f64; 2],
    pub v0: f64,
}

impl Bird {
    pub fn new(r: [f64; 2], v: [f64; 2], v0: f64) -> Bird {
        Bird {
            r,
            v,
            v0,
        }
    }

    pub fn update_direction(&mut self, birds: &[Bird], c_r:f64, noise: [f64; 2]) {
        let c_r2 = c_r * c_r;
        let mut dx = 0.;
        let mut dy = 0.;

        for b in birds.iter() {
            let d2 = self.dist2(b);
            // also sum over yourself
            if d2 < c_r2 {
                dx += b.v[0];
                dy += b.v[1];
            }
        }
        let norm = (dx.powi(2) + dy.powi(2)).sqrt();
        self.v = [dx/norm + noise[0], dy/norm + noise[1]];
    }

    pub fn update_r(&mut self) {
        self.r[0] += self.v0 * self.v[0];
        self.r[1] += self.v0 * self.v[1];

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
