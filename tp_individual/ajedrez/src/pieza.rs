
#[derive(Debug, PartialEq)]
pub enum Pieza{
    Rey(Colores),
    Reina(Colores),
    Peon(Colores),
    Alfil(Colores),
    Torre(Colores),
    Caballo(Colores)
}

#[derive(Debug, PartialEq)]
pub enum Colores{
    Blanco,
    Negro
}

pub fn obtener_pieza(texto_casilla: &str) -> Option<Pieza> {
    match texto_casilla {
        "R" => Some(Pieza::Rey(Colores::Blanco)),
        "Q" => Some(Pieza::Reina(Colores::Blanco)),
        "P" => Some(Pieza::Peon(Colores::Blanco)),
        "A" => Some(Pieza::Alfil(Colores::Blanco)),
        "T" => Some(Pieza::Torre(Colores::Blanco)),
        "C" => Some(Pieza::Caballo(Colores::Blanco)),
        "r" => Some(Pieza::Rey(Colores::Negro)),
        "q" => Some(Pieza::Reina(Colores::Negro)),
        "p" => Some(Pieza::Peon(Colores::Negro)),
        "a" => Some(Pieza::Alfil(Colores::Negro)),
        "t" => Some(Pieza::Torre(Colores::Negro)),
        "c" => Some(Pieza::Caballo(Colores::Negro)),
        _ => None
    }
}

#[test]
fn test_pieza() {
    let pieza1 = obtener_pieza("R");
    assert!(pieza1.is_some());
    assert_ne!(pieza1, Some(Pieza::Rey(Colores::Negro)));
    assert_eq!(pieza1, Some(Pieza::Rey(Colores::Blanco)));

    let pieza2 = obtener_pieza("r");
    assert!(pieza2.is_some());
    assert_ne!(pieza2, Some(Pieza::Rey(Colores::Blanco)));
    assert_eq!(pieza2, Some(Pieza::Rey(Colores::Negro)));

    let pieza3 = obtener_pieza("Q");
    assert!(pieza3.is_some());
    assert_ne!(pieza3, Some(Pieza::Torre(Colores::Blanco)));
    assert_eq!(pieza3, Some(Pieza::Reina(Colores::Blanco)));

    let pieza4 = obtener_pieza("t");
    assert!(pieza4.is_some());
    assert_ne!(pieza4, Some(Pieza::Torre(Colores::Blanco)));
    assert_eq!(pieza4, Some(Pieza::Torre(Colores::Negro)));

    let pieza5 = obtener_pieza("_");
    assert!(pieza5.is_none());
}
