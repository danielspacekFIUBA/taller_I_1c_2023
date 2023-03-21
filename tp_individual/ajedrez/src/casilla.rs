#[derive(Debug)]
pub struct Casilla {
    x: u8,
    y: u8,
}

impl Casilla {
    pub fn new(x: u8, y: u8) -> Self {
        Casilla { x, y }
    }
}
