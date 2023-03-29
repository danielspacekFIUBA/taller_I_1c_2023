mod ajedrez;
mod archivo_tablero;
mod casilla;
mod errores;
mod parse_args;
mod pieza;
mod tablero;
mod tipo_pieza;

use errores::AjedrezError;

use crate::ajedrez::Ajedrez;

pub fn resultado_ajedrez(args: Vec<String>) -> Result<String, AjedrezError> {
    let filename = parse_args::parse_args(args)?;
    let ajedrez = Ajedrez::new(filename)?;
    let resultado = ajedrez.resultado()?;
    let display_text = resultado.to_string();
    Ok(display_text)
}
