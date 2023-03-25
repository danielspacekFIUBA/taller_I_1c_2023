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
    let pieza_negra_gana = ajedrez.pieza_negra_puede_capturar();
    let ganan_todos = ajedrez.ganan_todos();
    let pierden_todos = ajedrez.pierden_todos();
    println!("TP Ajedrez");

    //println!("{:?}", ajedrez);
    println!("{:?}", casillas_ocupadas);
    println!("Pieza Blanca gana? {:?}", pieza_blanca_gana);
    println!("Pieza Negra gana? {:?}", pieza_negra_gana);
    println!("Ganan Todos? {:?}", ganan_todos);
    println!("Pierden todos? {:?}", pierden_todos);
}
