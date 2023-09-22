#[derive(Debug, PartialEq)]
pub enum Proximity {
    Neighbors(usize),
    Radius(f32),
}
