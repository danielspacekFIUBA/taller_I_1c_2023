use crate::errores::AjedrezError;
use crate::pieza::{self};
use crate::{archivo_tablero::ArchivoTablero, casilla::Casilla};

#[derive(Debug)]
pub struct Tablero {
    pub casillas: Vec<Vec<Casilla>>,
}

/// Representa al tablero con sus correspondientes casillas
impl Tablero {
    pub fn new(filename: String) -> Result<Self, AjedrezError> {
        let archivo_tablero = ArchivoTablero::new(filename)?;
        let mut casillas: Vec<Vec<Casilla>> = Vec::with_capacity(8);
        let mut i: i8 = 0;
        let mut j: i8 = 0;
        while i < 8 {
            let mut fila: Vec<Casilla> = Vec::with_capacity(8);
            while j < 8 {
                let pieza =
                    pieza::factory_pieza(&archivo_tablero.texto_casilla[i as usize][j as usize]);
                let casilla = Casilla::new(i, j, pieza);
                fila.push(casilla);
                j += 1;
            }
            casillas.push(fila);
            j = 0;
            i += 1;
        }
        let tablero = Tablero { casillas };
        tablero.valid()?;
        Ok(tablero)
    }

    /// Retorna TRUE si el tableo es válido
    /// Para ser válido tiene que tener 2 piezas
    /// Una de cada color
    fn valid(&self) -> Result<bool, AjedrezError> {
        if self.get_casillas_ocupadas().len() < 2 {
            return Err(AjedrezError::FaltanPiezas);
        }
        if self.get_casillas_ocupadas().len() > 2 {
            return Err(AjedrezError::SobranPiezas);
        }
        self.get_casilla_pieza_blanca()?;
        self.get_casilla_pieza_negra()?;
        Ok(true)
    }

    /// Retorna las casillas que tienen piezas
    pub fn get_casillas_ocupadas(&self) -> Vec<&Casilla> {
        let mut casillas_ocupadas: Vec<&Casilla> = Vec::new();
        for fila in &self.casillas {
            for casilla in fila {
                if casilla.pieza.is_some() {
                    casillas_ocupadas.push(casilla);
                }
            }
        }
        casillas_ocupadas
    }

    /// Retorna la casilla que contiene a la pieza blanca
    pub fn get_casilla_pieza_blanca(&self) -> Result<&Casilla, AjedrezError> {
        let casillas_ocupadas = self.get_casillas_ocupadas();
        for casilla in casillas_ocupadas {
            if let Some(pieza) = &casilla.pieza {
                if pieza.es_blanca() {
                    return Ok(casilla);
                }
            } else {
                return Err(AjedrezError::FaltanPiezas);
            }
        }
        Err(AjedrezError::FaltanPiezas)
    }

    /// Retorna la casilla que contiene a la pieza negra
    pub fn get_casilla_pieza_negra(&self) -> Result<&Casilla, AjedrezError> {
        let casillas_ocupadas = self.get_casillas_ocupadas();
        for casilla in casillas_ocupadas {
            if let Some(pieza) = &casilla.pieza {
                if pieza.es_negra() {
                    return Ok(casilla);
                }
            } else {
                return Err(AjedrezError::FaltanPiezas);
            }
        }
        Err(AjedrezError::FaltanPiezas)
    }
}

#[test]
fn test_tableros() {
    test_casillas_ocupadas();
    test_casilla_pieza_blanca();
    test_casilla_pieza_negra();
}

#[test]
fn test_casillas_ocupadas() {
    let tablero_result = Tablero::new(String::from("src/test_files/test3.txt"));
    assert!(tablero_result.is_ok());

    let tablero = tablero_result.unwrap();
    let casillas_ocupadas2 = tablero.get_casillas_ocupadas();
    assert_eq!(casillas_ocupadas2.len(), 2);
}

#[test]
fn test_casilla_pieza_blanca() {
    let tablero_result = Tablero::new(String::from("src/test_files/test3.txt"));
    assert!(tablero_result.is_ok());

    let tablero = tablero_result.unwrap();
    let casillas_pieza_blanca_result = tablero.get_casilla_pieza_blanca();
    assert!(casillas_pieza_blanca_result.is_ok());

    let casillas_pieza_blanca = casillas_pieza_blanca_result.unwrap();
    assert_eq!(casillas_pieza_blanca.coordenadas.x, 0);
    assert_eq!(casillas_pieza_blanca.coordenadas.y, 0);
}

#[test]
fn test_casilla_pieza_negra() {
    let tablero_result = Tablero::new(String::from("src/test_files/test3.txt"));
    assert!(tablero_result.is_ok());

    let tablero = tablero_result.unwrap();
    let casillas_pieza_negra_result = tablero.get_casilla_pieza_negra();
    assert!(casillas_pieza_negra_result.is_ok());

    let casillas_pieza_negra = casillas_pieza_negra_result.unwrap();
    assert_eq!(casillas_pieza_negra.coordenadas.x, 4);
    assert_eq!(casillas_pieza_negra.coordenadas.y, 3);
}
