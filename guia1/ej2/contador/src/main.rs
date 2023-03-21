use crate::contador::Contador;

mod contador;

fn main() {
    let contador = Contador::new("src/archivo_prueba.txt");

    let mapeo = contador.contador_palabras;
    
    for (key, value) in mapeo {
        println!("{} -> {}", key, value);
    }
}
