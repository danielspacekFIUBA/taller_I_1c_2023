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

    pub fn pieza_negra_puede_capturar(&self) -> bool {
        let casilla_pieza_negra = self.tablero.get_casilla_pieza_negra();
        let casilla_pieza_blanca = self.tablero.get_casilla_pieza_blanca();
        casilla_pieza_negra.puedo_capturar(casilla_pieza_blanca)
    }

    pub fn ganan_todos(&self) -> bool {
        self.pieza_blanca_puede_capturar() && self.pieza_negra_puede_capturar()
    }

    pub fn pierden_todos(&self) -> bool {
        !self.pieza_blanca_puede_capturar() && !self.pieza_negra_puede_capturar()
    }
}

#[test]
fn test_ajedrez() {
    test_ajedrez_gana_blanca();
    test_ajedrez_gana_negra();
    test_ajedrez_ganan_todos();
    test_ajedrez_pierden_todos();
}

#[test]
fn test_ajedrez_gana_blanca() {
    let ajedrez1 = Ajedrez::new(String::from("src/test_files/test1.txt"));
    let gana_blanca1 = ajedrez1.pieza_blanca_puede_capturar();
    assert!(gana_blanca1);

    let ajedrez2 = Ajedrez::new(String::from("src/test_files/test2.txt"));
    let gana_blanca2 = ajedrez2.pieza_blanca_puede_capturar();
    assert!(!gana_blanca2);

    let ajedrez3 = Ajedrez::new(String::from("src/test_files/test4.txt"));
    let gana_blanca3 = ajedrez3.pieza_blanca_puede_capturar();
    assert!(!gana_blanca3);
}

#[test]
fn test_ajedrez_gana_negra() {
    let ajedrez1 = Ajedrez::new(String::from("src/test_files/test1.txt"));
    let gana_negra1 = ajedrez1.pieza_negra_puede_capturar();
    assert!(!gana_negra1);

    let ajedrez2 = Ajedrez::new(String::from("src/test_files/test2.txt"));
    let gana_negra2 = ajedrez2.pieza_negra_puede_capturar();
    assert!(!gana_negra2);

    let ajedrez3 = Ajedrez::new(String::from("src/test_files/test4.txt"));
    let gana_negra3 = ajedrez3.pieza_negra_puede_capturar();
    assert!(gana_negra3);
}

#[test]
fn test_ajedrez_ganan_todos() {
    let ajedrez1 = Ajedrez::new(String::from("src/test_files/test5.txt"));
    let ganan_todos1 = ajedrez1.ganan_todos();
    assert!(ganan_todos1);

    let ajedrez2 = Ajedrez::new(String::from("src/test_files/test6.txt"));
    let ganan_todos2 = ajedrez2.ganan_todos();
    assert!(ganan_todos2);

    let ajedrez3 = Ajedrez::new(String::from("src/test_files/test4.txt"));
    let no_ganan_todos = ajedrez3.ganan_todos();
    assert!(!no_ganan_todos);
}

#[test]
fn test_ajedrez_pierden_todos() {
    let ajedrez1 = Ajedrez::new(String::from("src/test_files/test2.txt"));
    let pierden_todos1 = ajedrez1.pierden_todos();
    assert!(pierden_todos1);

    let ajedrez2 = Ajedrez::new(String::from("src/test_files/test3.txt"));
    let pierden_todos2 = ajedrez2.pierden_todos();
    assert!(pierden_todos2);

    let ajedrez3 = Ajedrez::new(String::from("src/test_files/test4.txt"));
    let no_pierden_todos = ajedrez3.pierden_todos();
    assert!(!no_pierden_todos);
}
