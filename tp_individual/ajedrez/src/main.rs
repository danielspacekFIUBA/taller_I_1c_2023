mod casilla;
mod tablero;
use crate::tablero::Tablero;

fn main() {
    let ajedrez = Tablero::new();
    println!("TP Ajedrez");

    println!("{:?}", ajedrez);
}
