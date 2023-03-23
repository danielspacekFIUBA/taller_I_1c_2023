mod archivo_tablero;
mod casilla;
mod tablero;
mod pieza;
use crate::tablero::Tablero;

fn main() {
    //let tablero = ArchivoTablero::new("src/tablero.txt".to_string());
    let ajedrez = Tablero::new("src/tablero.txt".to_string());
    let casillas_ocupadas = ajedrez.get_casillas_ocupadas();
    println!("TP Ajedrez");

    //println!("{:?}", ajedrez);
    println!("{:?}", casillas_ocupadas);
}
