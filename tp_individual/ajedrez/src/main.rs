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
    let resultado = ajedrez.resultado();

    println!("TP Ajedrez");

    //println!("{:?}", ajedrez);
    println!("Resultado? {:?}", resultado);
}
