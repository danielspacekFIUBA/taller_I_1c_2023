use std::{error::Error, fmt};

#[derive(Debug)]
pub enum AjedrezError {
    NoArgumentError,
    NoResultError,
}

impl Error for AjedrezError {}

impl fmt::Display for AjedrezError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AjedrezError::NoArgumentError => {
                write!(f, "ERROR No se especificó el nombre del archivo.")
            }
            AjedrezError::NoResultError => {
                write!(f, "ERROR No se obtuvo ningún resultado válido.")
            }
        }
    }
}
