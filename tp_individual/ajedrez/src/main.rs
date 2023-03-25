mod ajedrez;
mod archivo_tablero;
mod casilla;
mod pieza;
mod tablero;
mod tipo_pieza;
use crate::ajedrez::Ajedrez;

fn main() {
    //let tablero = ArchivoTablero::new("src/tablero.txt".to_string());
    let ajedrez = Ajedrez::new("src/tablero.txt".to_string());
    let casillas_ocupadas = ajedrez.tablero.get_casillas_ocupadas();
    let pieza_blanca_gana = ajedrez.pieza_blanca_puede_capturar();
    println!("TP Ajedrez");

    //println!("{:?}", ajedrez);
    println!("{:?}", casillas_ocupadas);
    println!("Pieza Blanca gana? {:?}", pieza_blanca_gana);
}
