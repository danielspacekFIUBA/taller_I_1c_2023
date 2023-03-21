use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn leer_palabras(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No existe el archivo");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("No se puede parsear la linea"))
        .collect()
}

pub fn elegir_palabra(palabras: Vec<String>) -> String {
    String::from(&palabras[0])
}
