use crate::pieza::Pieza;

#[derive(Debug)]
pub struct Casilla {
    pub x: usize,
    pub y: usize,
    pub pieza: Option<Pieza>
}

impl Casilla {
    pub fn new(x: usize, y: usize, pieza: Option<Pieza>) -> Self {
        Casilla { x, y, pieza }
    }
}
