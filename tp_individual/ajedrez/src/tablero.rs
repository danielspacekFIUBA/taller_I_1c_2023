use crate::casilla::Casilla;

#[derive(Debug)]
pub struct Tablero {
    pub casillas: Vec<Vec<Casilla>>,
}

impl Tablero {
    pub fn new() -> Self {
        let mut casillas: Vec<Vec<Casilla>> = Vec::with_capacity(8);
        for i in 0..8 {
            let mut fila: Vec<Casilla> = Vec::with_capacity(8);
            for j in 0..8 {
                let casilla = Casilla::new(i, j);
                fila.push(casilla);
            }
            casillas.push(fila);
        }
        Tablero { casillas }
    }
}
