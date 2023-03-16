const CHAR_INCOGNITA: char = '_';

pub struct Ahorcado{
    pub palabra: String,
    pub letras_intentadas: Vec<char>,
    pub letras_adivinadas: Vec<char>,
    pub letras_equivocadas: Vec<char>,
    pub palabra_oculta_actual: String,
    pub resuelto: bool,
}

impl Ahorcado{
    pub fn new(palabra: String) -> Self {
        let palabra = palabra.to_string();
        let letras_intentadas: Vec<char> = [].to_vec();
        let letras_adivinadas: Vec<char> = [].to_vec();
        let letras_equivocadas: Vec<char> = [].to_vec();
        let palabra_oculta_actual: String = "".to_string();
        let resuelto = false;
        
        let mut ahorcado = Ahorcado { palabra, letras_intentadas, letras_adivinadas, letras_equivocadas, palabra_oculta_actual, resuelto };
        ahorcado.completar_palabra();
        ahorcado
    }

    pub fn agregar_letra(&mut self, letra: String) -> bool{
        let caracteres: Vec<char> = letra.chars().collect();
        let caracter= caracteres.first().expect("No ingresó ninguna letra válida");

        if self.letras_intentadas.contains(caracter) { return false; }
        self.letras_intentadas.push(*caracter);
        self.completar_palabra();
        if !self.letras_adivinadas.contains(caracter){
            self.letras_equivocadas.push(*caracter);
        }
        true
    }

    fn completar_palabra(&mut self) {
        let letras_palabra: Vec<char>= self.palabra.chars().collect();
        let mut letras_adivinadas: Vec<char> = [].to_vec();
        let mut resuelto = true;
        let resultado_incognita: Vec<String> = letras_palabra.iter().map(|&x| {
            if self.letras_intentadas.contains(&x){
                if !letras_adivinadas.contains(&x){
                    letras_adivinadas.push(x);
                }
                return String::from(x);
            }else {
                resuelto = false;
                return String::from(CHAR_INCOGNITA);
            }
        }).collect::<Vec<_>>();
        self.palabra_oculta_actual = resultado_incognita.join(" ");
        self.letras_adivinadas = letras_adivinadas;
        self.resuelto = resuelto;
    }
}