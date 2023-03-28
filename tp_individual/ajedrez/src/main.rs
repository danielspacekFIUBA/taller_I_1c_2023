mod ajedrez;
mod archivo_tablero;
mod casilla;
mod errores;
mod parse_args;
mod pieza;
mod tablero;
mod tipo_pieza;

use std::env;

use ajedrez::ResultadoAjedrez;
use errores::AjedrezError;

use crate::ajedrez::Ajedrez;

fn main() {
    let args: Vec<String> = env::args().collect();
    match resultado_ajedrez(args) {
        Ok(resultado) => {
            println!("{}", resultado);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn resultado_ajedrez(args: Vec<String>) -> Result<ResultadoAjedrez, AjedrezError> {
    let filename = parse_args::parse_args(args)?;
    let ajedrez = Ajedrez::new(filename)?;
    let resultado = ajedrez.resultado()?;
    Ok(resultado)
}
