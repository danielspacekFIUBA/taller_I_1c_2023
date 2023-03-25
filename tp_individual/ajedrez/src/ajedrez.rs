use crate::tablero::Tablero;

#[derive(Debug)]
pub struct Ajedrez {
    pub tablero: Tablero,
}

impl Ajedrez {
    pub fn new(filename: String) -> Self {
        let tablero = Tablero::new(filename);
        Ajedrez { tablero }
    }

    pub fn pieza_blanca_puede_capturar(&self) -> bool {
        let casilla_pieza_blanca = self.tablero.get_casilla_pieza_blanca();
        let casilla_pieza_negra = self.tablero.get_casilla_pieza_negra();
        casilla_pieza_blanca.puedo_capturar(casilla_pieza_negra)
    }
}

#[test]
fn test_ajedrez() {
    let ajedrez1 = Ajedrez::new(String::from("src/test_files/test1.txt"));
    let gana_blanca1 = ajedrez1.pieza_blanca_puede_capturar();
    assert!(gana_blanca1);

    let ajedrez2 = Ajedrez::new(String::from("src/test_files/test2.txt"));
    let gana_blanca2 = ajedrez2.pieza_blanca_puede_capturar();
    assert!(!gana_blanca2);

}
