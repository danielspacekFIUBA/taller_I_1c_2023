use crate::{errores::AjedrezError, tablero::Tablero};
use std::{fmt};

#[derive(Debug)]
pub struct Ajedrez {
    pub tablero: Tablero,
}

#[derive(Debug)]
pub enum ResultadoAjedrez {
    GanaBlanca,
    GanaNegra,
    Empate,
    PierdenTodos,
}

impl fmt::Display for ResultadoAjedrez {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResultadoAjedrez::GanaBlanca => {
                write!(f, "B")
            },
            ResultadoAjedrez::GanaNegra => {
                write!(f, "N")
            },
            ResultadoAjedrez::Empate => {
                write!(f, "E")
            },
            ResultadoAjedrez::PierdenTodos => {
                write!(f, "P")
            }
        }
    }
}


/// Represeta al tablero de ajedrez y sus fichas
impl Ajedrez {
    pub fn new(filename: String) -> Result<Self, AjedrezError> {
        let tablero = Tablero::new(filename);
        Ok(Ajedrez { tablero })
    }

    /// Devuelve TRUE si la pieza blanca puede capturar a la pieza negra
    /// En caso contrario retorna FALSE
    fn pieza_blanca_puede_capturar(&self) -> bool {
        let casilla_pieza_blanca = self.tablero.get_casilla_pieza_blanca();
        let casilla_pieza_negra = self.tablero.get_casilla_pieza_negra();
        casilla_pieza_blanca.puedo_capturar(casilla_pieza_negra)
    }

    /// Devuelve TRUE si la pieza negra puede capturar a la pieza blanca
    /// En caso contrario retorna FALSE
    fn pieza_negra_puede_capturar(&self) -> bool {
        let casilla_pieza_negra = self.tablero.get_casilla_pieza_negra();
        let casilla_pieza_blanca = self.tablero.get_casilla_pieza_blanca();
        casilla_pieza_negra.puedo_capturar(casilla_pieza_blanca)
    }

    /// Devuelve TRUE si ambas piezas pueden capturarse mutuamente
    /// En caso contrario retorna FALSE
    fn ganan_todos(&self) -> bool {
        self.pieza_blanca_puede_capturar() && self.pieza_negra_puede_capturar()
    }

    /// Devuelve TRUE si ninguna pieza puede capturar a la otra pieza del tablero
    /// En caso contrario retorna FALSE
    fn pierden_todos(&self) -> bool {
        !self.pieza_blanca_puede_capturar() && !self.pieza_negra_puede_capturar()
    }

    /// Retorna el resultado del Juego
    /// En caso que no se pueda determinar un resultado retorna None
    pub fn resultado(&self) -> Result<ResultadoAjedrez, AjedrezError> {
        if self.pierden_todos() {
            return Ok(ResultadoAjedrez::PierdenTodos);
        }
        if self.ganan_todos() {
            return Ok(ResultadoAjedrez::Empate);
        }
        if self.pieza_blanca_puede_capturar() {
            return Ok(ResultadoAjedrez::GanaBlanca);
        }
        if self.pieza_negra_puede_capturar() {
            return Ok(ResultadoAjedrez::GanaNegra);
        }
        Err(AjedrezError::NoResultError)
    }
}

#[test]
fn test_ajedrez() {
    test_ajedrez_gana_blanca();
    test_ajedrez_gana_negra();
    test_ajedrez_ganan_todos();
    test_ajedrez_pierden_todos();
}

#[test]
fn test_ajedrez_gana_blanca() {
    // if let Ok(ajedrez1) = Ajedrez::new(String::from("src/test_files/test1.txt")) {
    //     let gana_blanca1 = ajedrez1.pieza_blanca_puede_capturar();
    //     assert!(gana_blanca1);
    // }

    // if let Ok(ajedrez2) = Ajedrez::new(String::from("src/test_files/test2.txt")) {
    //     let gana_blanca2 = ajedrez2.pieza_blanca_puede_capturar();
    //     assert!(!gana_blanca2);
    // }
    // if let Ok(ajedrez3) = Ajedrez::new(String::from("src/test_files/test4.txt")) {
    //     let gana_blanca3 = ajedrez3.pieza_blanca_puede_capturar();
    //     assert!(!gana_blanca3);
    // }
}

#[test]
fn test_ajedrez_gana_negra() {
    //let ajjj = Ajedrez::new(String::from("src/test_files/test1.txt")?

    // if let Ok(ajedrez1) = Ajedrez::new(String::from("src/test_files/test1.txt")) {
    //     let gana_negra1 = ajedrez1.pieza_negra_puede_capturar();
    //     assert!(!gana_negra1);
    // } else {
    //     assert!()
    // }
    // if let Ok(ajedrez2) = Ajedrez::new(String::from("src/test_files/test2.txt")) {
    //     let gana_negra2 = ajedrez2.pieza_negra_puede_capturar();
    //     assert!(!gana_negra2);
    // }
    // if let Ok(ajedrez3) = Ajedrez::new(String::from("src/test_files/test4.txt")) {
    //     let gana_negra3 = ajedrez3.pieza_negra_puede_capturar();
    //     assert!(gana_negra3);
    // }
}

#[test]
fn test_ajedrez_ganan_todos() {
    // if let Ok(ajedrez1) = Ajedrez::new(String::from("src/test_files/test5.txt")) {
    //     let ganan_todos1 = ajedrez1.ganan_todos();
    //     assert!(ganan_todos1);
    // }

    // if let Ok(ajedrez2) = Ajedrez::new(String::from("src/test_files/test6.txt")) {
    //     let ganan_todos2 = ajedrez2.ganan_todos();
    //     assert!(ganan_todos2);
    // }

    // if let Ok(ajedrez3) = Ajedrez::new(String::from("src/test_files/test4.txt")) {
    //     let no_ganan_todos = ajedrez3.ganan_todos();
    //     assert!(!no_ganan_todos);
    // }
}

#[test]
fn test_ajedrez_pierden_todos() {
    // if let Ok(ajedrez1) = Ajedrez::new(String::from("src/test_files/test2.txt")) {
    //     let pierden_todos1 = ajedrez1.pierden_todos();
    //     assert!(pierden_todos1);
    // }

    // if let Ok(ajedrez2) = Ajedrez::new(String::from("src/test_files/test3.txt")) {
    //     let pierden_todos2 = ajedrez2.pierden_todos();
    //     assert!(pierden_todos2);
    // }

    // if let Ok(ajedrez3) = Ajedrez::new(String::from("src/test_files/test4.txt")) {
    //     let no_pierden_todos = ajedrez3.pierden_todos();
    //     assert!(!no_pierden_todos);
    // }
}
