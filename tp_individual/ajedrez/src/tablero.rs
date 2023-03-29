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
        if !tablero.is_valid() {
            return Err(AjedrezError::FaltanPiezas);
        }
        Ok(tablero)
    }

    /// Retorna TRUE si el tableo es válido
    fn is_valid(&self) -> bool {
        self.get_casillas_ocupadas().len() == 2
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
    pub fn get_casilla_pieza_blanca(&self) -> &Casilla {
        let casillas_ocupadas = self.get_casillas_ocupadas();
        let casilla_default = &self.casillas[0][0];
        for casilla in casillas_ocupadas {
            let es_blanca = &casilla
                .pieza
                .as_ref()
                .expect("Error al obtener la pieza")
                .es_blanca();
            if *es_blanca {
                return casilla;
            }
        }
        casilla_default
    }

    /// Retorna la casilla que contiene a la pieza negra
    pub fn get_casilla_pieza_negra(&self) -> &Casilla {
        let casillas_ocupadas = self.get_casillas_ocupadas();
        let casilla_default = &self.casillas[0][0];
        for casilla in casillas_ocupadas {
            let es_blanca = &casilla
                .pieza
                .as_ref()
                .expect("Error al obtener la pieza")
                .es_blanca();
            if !*es_blanca {
                return casilla;
            }
        }
        casilla_default
    }
}

//#[test]
// fn test_tableros() {
//     test_casillas_ocupadas();
//     test_casilla_pieza_blanca();
//     test_casilla_pieza_negra();
// }

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
    let casillas_pieza_blanca = tablero.get_casilla_pieza_blanca();
    assert_eq!(casillas_pieza_blanca.coordenadas.x, 0);
    assert_eq!(casillas_pieza_blanca.coordenadas.y, 0);
}

#[test]
fn test_casilla_pieza_negra() {
    let tablero_result = Tablero::new(String::from("src/test_files/test3.txt"));
    assert!(tablero_result.is_ok());

    let tablero = tablero_result.unwrap();
    let casillas_pieza_negra = tablero.get_casilla_pieza_negra();
    assert_eq!(casillas_pieza_negra.coordenadas.x, 4);
    assert_eq!(casillas_pieza_negra.coordenadas.y, 3);
}
