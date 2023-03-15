use std::io;

use crate::ahorcado::Ahorcado;

mod words;
mod ahorcado;

const MAXIMO_INTENTOS_ERRONEOS: u8 = 5;

fn main() {
    println!("Bienvenido al ahorcado de FIUBA!");
    let palabras = words::leer_palabras("src/words.txt");
    let palabra = words::elegir_palabra(palabras);

    let mut ahorcado = Ahorcado::new(palabra.to_string());
    let mut intentos: u8 = 0;

    loop{
         println!("La palabra hasta el momento es: {}", ahorcado.palabra_oculta_actual);

        let letras_adivinadas = ahorcado.letras_adivinadas.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        println!("Adivinaste las siguientes letras: {}", letras_adivinadas);
        println!("Te quedan {} intentos.", MAXIMO_INTENTOS_ERRONEOS - intentos);

        println!("Ingresa una letra: ");
        let mut letra = String::new();
        io::stdin().read_line(&mut letra).expect("Error leyendo la letra.");

        let letra_acertada = ahorcado.agregar_letra(letra);
        
        if !letra_acertada { intentos = intentos + 1; }

        if MAXIMO_INTENTOS_ERRONEOS == intentos || ahorcado.resuelto { break; }
    }

    println!("La palabra oculta era: {}", palabra);

    if ahorcado.resuelto {
        println!("¡FELICITACIONES!");
    } else {
        println!("PERDISTE!");
    }

}
