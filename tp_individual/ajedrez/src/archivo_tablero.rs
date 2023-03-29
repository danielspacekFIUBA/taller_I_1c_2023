use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::errores::AjedrezError;

#[derive(Debug, PartialEq)]
pub struct ArchivoTablero {
    pub texto_casilla: Vec<Vec<String>>,
}

/// Representa a cada casilla del tablero
/// Leyendo desde el archivo pasado por parametro
impl ArchivoTablero {
    pub fn new(filename: String) -> Result<ArchivoTablero, AjedrezError> {
        if let Ok(file) = File::open(filename) {
            let buf = BufReader::new(file);
            let lineas = ArchivoTablero::parsear_archivo(buf)?;
            if lineas.len() != 8 {
                return Err(AjedrezError::FormatFile);
            }
            let mut texto_casilla: Vec<Vec<String>> = Vec::with_capacity(8);
            for linea in lineas {
                let fila: Vec<String> = linea.split(' ').map(|s| s.to_string()).collect();
                if fila.len() != 8 {
                    return Err(AjedrezError::FormatFile);
                }
                texto_casilla.push(fila);
            }
            Ok(ArchivoTablero { texto_casilla })
        } else {
            Err(AjedrezError::NoExisteArchivo)
        }
    }

    fn parsear_archivo(buf: BufReader<File>) -> Result<Vec<String>, AjedrezError> {
        let buf_lineas = buf.lines();
        let mut hay_errores = false;
        let lineas: Vec<String> = buf_lineas
            .map(|l| {
                if let Ok(linea) = l {
                    linea
                } else {
                    hay_errores = false;
                    "".to_string()
                }
            })
            .collect();
        if hay_errores {
            return Err(AjedrezError::FormatFile);
        }
        Ok(lineas)
    }
}

#[test]
fn test_archivos_tableros() {
    test_archivos_tableros_validos();
    test_archivos_tableros_invalidos();
}

#[test]
fn test_archivos_tableros_validos() {
    let archivo_test1_result = ArchivoTablero::new(String::from("src/test_files/test1.txt"));
    assert!(archivo_test1_result.is_ok());
    let archivo_test1 = archivo_test1_result.unwrap();
    assert_eq!(archivo_test1.texto_casilla[0][0], "_");
    assert_eq!(archivo_test1.texto_casilla[1][0], "Q");
    assert_eq!(archivo_test1.texto_casilla[4][3], "r");
    assert_eq!(archivo_test1.texto_casilla[7][7], "_");

    let archivo_test2_result = ArchivoTablero::new(String::from("src/test_files/test2.txt"));
    assert!(archivo_test2_result.is_ok());
    let archivo_test2 = archivo_test2_result.unwrap();
    assert_eq!(archivo_test2.texto_casilla[0][0], "_");
    assert_eq!(archivo_test2.texto_casilla[4][1], "R");
    assert_eq!(archivo_test2.texto_casilla[4][3], "r");
    assert_eq!(archivo_test2.texto_casilla[7][7], "_");
}

#[test]
fn test_archivos_tableros_invalidos() {
    let archivo_error1 = ArchivoTablero::new(String::from("src/test_files/test7.txt"));
    assert!(archivo_error1.is_err());
    assert_eq!(archivo_error1, Err(AjedrezError::FormatFile));

    let archivo_error2 = ArchivoTablero::new(String::from("src/test_files/test8.txt"));
    assert!(archivo_error2.is_err());
    assert_eq!(archivo_error2, Err(AjedrezError::FormatFile));

    let archivo_error3 = ArchivoTablero::new(String::from("src/test_files/test9.txt"));
    assert!(archivo_error3.is_err());
    assert_eq!(archivo_error3, Err(AjedrezError::FormatFile));

    let archivo_error4 = ArchivoTablero::new(String::from("src/test_files/test_no_existe.txt"));
    assert!(archivo_error4.is_err());
    assert_eq!(archivo_error4, Err(AjedrezError::NoExisteArchivo));
}
