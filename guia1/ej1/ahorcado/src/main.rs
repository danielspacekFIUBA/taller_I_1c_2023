use std::io;

use crate::ahorcado::Ahorcado;

mod ahorcado;
mod words;

const MAXIMO_INTENTOS_ERRONEOS: usize = 5;

fn main() {
    println!("Bienvenido al ahorcado de FIUBA!");
    let palabras = words::leer_palabras("src/words.txt");
    let palabra = words::elegir_palabra(palabras);

    let mut ahorcado = Ahorcado::new(palabra.to_string());
    //let mut intentos: u8 = 0;

    loop {
        println!(
            "La palabra hasta el momento es: {}",
            ahorcado.palabra_oculta_actual
        );

        let letras_adivinadas = ahorcado
            .letras_adivinadas
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!("Adivinaste las siguientes letras: {}", letras_adivinadas);
        let letras_equivocadas = ahorcado
            .letras_equivocadas
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!("Erraste las siguientes letras: {}", letras_equivocadas);
        println!(
            "Te quedan {} intentos.",
            MAXIMO_INTENTOS_ERRONEOS - ahorcado.letras_equivocadas.len()
        );

        println!("Ingresa una letra: ");
        let mut letra = String::new();
        io::stdin()
            .read_line(&mut letra)
            .expect("Error leyendo la letra.");

        let letra_no_repetida = ahorcado.agregar_letra(letra);

        if !letra_no_repetida {
            println!("Esa letra ya la eligió, elija otra");
        }

        if MAXIMO_INTENTOS_ERRONEOS == ahorcado.letras_equivocadas.len() || ahorcado.resuelto {
            break;
        }
    }

    println!("La palabra oculta era: {}", palabra);

    if ahorcado.resuelto {
        println!("¡FELICITACIONES!");
    } else {
        println!("PERDISTE!");
    }
}
