use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub enum AjedrezError {
    NoArgument,
    NoExisteArchivo,
    FormatFile,
    NoResult,
    FaltanPiezas,
    SobranPiezas,
}

impl Error for AjedrezError {}

impl fmt::Display for AjedrezError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AjedrezError::NoArgument => {
                write!(f, "ERROR: No se especificó el nombre del archivo.")
            }
            AjedrezError::NoResult => {
                write!(f, "ERROR: No se obtuvo ningún resultado válido.")
            }
            AjedrezError::NoExisteArchivo => {
                write!(f, "ERROR: No existe el archivo especificado.")
            }
            AjedrezError::FormatFile => {
                write!(f, "ERROR: Formato de archivo incompatible.")
            }
            AjedrezError::FaltanPiezas => {
                write!(f, "ERROR: Piezas faltantes.")
            }
            AjedrezError::SobranPiezas => {
                write!(f, "ERROR: Piezas sobrantes.")
            }
        }
    }
}
