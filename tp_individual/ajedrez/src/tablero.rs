
use crate::{archivo_tablero::ArchivoTablero, casilla::Casilla};
use crate::pieza::{self};

#[derive(Debug)]
pub struct Tablero {
    pub casillas: Vec<Vec<Casilla>>,
}

impl Tablero {
    pub fn new(filename: String) -> Self {
        let archivo_tablero = ArchivoTablero::new(filename);
        let mut casillas: Vec<Vec<Casilla>> = Vec::with_capacity(8);
        for i in 0..8 {
            let mut fila: Vec<Casilla> = Vec::with_capacity(8);
            for j in 0..8 {
                let pieza = pieza::obtener_pieza(&archivo_tablero.texto_casilla[i][j]);
                let casilla = Casilla::new(i, j, pieza);
                fila.push(casilla);
            }
            casillas.push(fila);
        }
        Tablero { casillas }
    }

    pub fn get_casillas_ocupadas(&self) -> Vec<&Casilla> {
        let mut casillas_ocupadas: Vec<&Casilla> = Vec::new();
        for fila in &self.casillas {
            for casilla in fila{
                if casilla.pieza.is_some() { 
                    casillas_ocupadas.push(&casilla); 
                }
            }
        }
        casillas_ocupadas
    }
}

#[test]
fn test_tableros() {
    let tablero2 = Tablero::new(String::from("src/test_files/test3.txt"));
    let casillas_ocupadas2 = tablero2.get_casillas_ocupadas();
    assert_eq!(casillas_ocupadas2.len(), 2);
}
