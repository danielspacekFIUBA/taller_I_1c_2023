use std::{
    fs::File,
    io::{prelude::*, BufReader, Cursor},
    path::Path,
    collections::HashMap
};


pub struct Contador{
    pub contador_palabras: HashMap<String, u32>
}

impl Contador{
    pub fn new(filename: impl AsRef<Path>) -> Self {
        let contador_palabras: HashMap<String, u32> = HashMap::new();
        let mut ret = Contador { contador_palabras};
        ret.contar(filename);
        ret
    }

    fn contar(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("No existe el archivo");
        let buf = BufReader::new(file);
        buf.lines()
            .for_each(|l| {
                let linea = l.expect("No se puede parsear la linea");
                let linea_parseada = Contador::parsear_linea(&linea);
                self.completar_contador(linea_parseada)
            });
    }

    fn contar_leyedo_por_linea(&mut self, filename: impl AsRef<Path>) {
        let file = File::open(filename).expect("No existe el archivo");
        let mut buf = BufReader::new(file);
        let mut linea = String::new();
        let mut num_bytes = buf.read_line(&mut linea);

        while Some(num_bytes) > 0 {
            let linea_parseada = Contador::parsear_linea(&linea);
            self.completar_contador(linea_parseada);
            num_bytes = buf.read_line(&mut linea);
        }
    }

    fn completar_contador(&mut self, linea_parseada: Vec<&str>) {
        for palabra in linea_parseada {
            let palabra_minuscula = palabra.trim().to_lowercase();
            if !palabra_minuscula.is_empty() { self.contar_palabra(palabra_minuscula); }
        }
    }

    fn contar_palabra(&mut self, palabra: String) {
        let cantidad = self.contador_palabras.entry(palabra).or_insert(0);
        *cantidad += 1
    }

    fn parsear_linea(linea: &String) -> Vec<&str> {
        let palabras: Vec<&str> = linea.split(&[' ', ',', ';', '.']).collect();
        palabras
    }

}

