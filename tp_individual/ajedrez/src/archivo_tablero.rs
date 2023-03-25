use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct ArchivoTablero {
    pub texto_casilla: Vec<Vec<String>>,
}

impl ArchivoTablero {
    pub fn new(filename: String) -> ArchivoTablero {
        let file = File::open(filename).expect("No existe el archivo");
        let buf = BufReader::new(file);
        let lineas: Vec<String> = buf
            .lines()
            .map(|l| l.expect("No se puede parsear la linea"))
            .collect();

        let mut texto_casilla: Vec<Vec<String>> = Vec::with_capacity(8);
        for linea in lineas {
            let fila: Vec<String> = linea.split(' ').map(|s| s.to_string()).collect();
            texto_casilla.push(fila);
        }

        ArchivoTablero { texto_casilla }
    }
}

#[test]
fn test_archivos_tableros() {
    let archivo_test1 = ArchivoTablero::new(String::from("src/test_files/test1.txt"));
    assert_eq!(archivo_test1.texto_casilla[0][0], "_");
    assert_eq!(archivo_test1.texto_casilla[1][0], "Q");
    assert_eq!(archivo_test1.texto_casilla[4][3], "r");
    assert_eq!(archivo_test1.texto_casilla[7][7], "_");

    let archivo_test1 = ArchivoTablero::new(String::from("src/test_files/test2.txt"));
    assert_eq!(archivo_test1.texto_casilla[0][0], "_");
    assert_eq!(archivo_test1.texto_casilla[4][1], "R");
    assert_eq!(archivo_test1.texto_casilla[4][3], "r");
    assert_eq!(archivo_test1.texto_casilla[7][7], "_");
}
