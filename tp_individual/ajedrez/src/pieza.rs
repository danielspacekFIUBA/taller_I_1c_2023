use crate::tipo_pieza::TipoPieza;

#[derive(Debug, PartialEq)]
pub struct Pieza {
    pub tipo_pieza: TipoPieza,
    pub color: Color,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Blanco,
    Negro,
}

/// Representa a una pieza de ajedrez
/// Incluye el tipo de pieza y el color correspondiente
impl Pieza {
    pub fn new(tipo_pieza: TipoPieza, color: Color) -> Self {
        Pieza { tipo_pieza, color }
    }

    /// Devuelve TRUE en caso que la pieza sea de color blanco
    pub fn es_blanca(&self) -> bool {
        self.color == Color::Blanco
    }

    /// Devuelve TRUE en caso que la pieza sea de color negro
    pub fn _es_negra(&self) -> bool {
        self.color == Color::Negro
    }
}

/// Crea la pieza según la letra y el color según si es mayúscula
///
/// # Argumento
/// * `texto_casilla` - Un puntero a str que indique una letra válida
///
/// Las letras válidas son:
/// R: Rey Blanco
/// Q: Reina Blanca
/// P: Peón Blanco
/// A: Alfil Blanco
/// C: Caballo Blanco
/// T: Torre Blanca
/// r: Rey Negro
/// q: Reina Negra
/// p: Peón Negro
/// a: Alfil Negro
/// c: Caballo Negro
/// t: Torre Negra
pub fn factory_pieza(texto_casilla: &str) -> Option<Pieza> {
    match texto_casilla {
        "R" => Some(Pieza::new(TipoPieza::Rey, Color::Blanco)),
        "Q" => Some(Pieza::new(TipoPieza::Reina, Color::Blanco)),
        "P" => Some(Pieza::new(TipoPieza::Peon, Color::Blanco)),
        "A" => Some(Pieza::new(TipoPieza::Alfil, Color::Blanco)),
        "T" => Some(Pieza::new(TipoPieza::Torre, Color::Blanco)),
        "C" => Some(Pieza::new(TipoPieza::Caballo, Color::Blanco)),
        "r" => Some(Pieza::new(TipoPieza::Rey, Color::Negro)),
        "q" => Some(Pieza::new(TipoPieza::Reina, Color::Negro)),
        "p" => Some(Pieza::new(TipoPieza::Peon, Color::Negro)),
        "a" => Some(Pieza::new(TipoPieza::Alfil, Color::Negro)),
        "t" => Some(Pieza::new(TipoPieza::Torre, Color::Negro)),
        "c" => Some(Pieza::new(TipoPieza::Caballo, Color::Negro)),
        _ => None,
    }
}

#[test]
fn test_pieza() {
    let pieza1 = factory_pieza("R");
    assert!(pieza1.is_some());
    assert_ne!(pieza1, Some(Pieza::new(TipoPieza::Rey, Color::Negro)));
    assert_eq!(pieza1, Some(Pieza::new(TipoPieza::Rey, Color::Blanco)));

    let pieza2 = factory_pieza("r");
    assert!(pieza2.is_some());
    assert_ne!(pieza2, Some(Pieza::new(TipoPieza::Rey, Color::Blanco)));
    assert_eq!(pieza2, Some(Pieza::new(TipoPieza::Rey, Color::Negro)));

    let pieza3 = factory_pieza("Q");
    assert!(pieza3.is_some());
    assert_ne!(pieza3, Some(Pieza::new(TipoPieza::Torre, Color::Blanco)));
    assert_eq!(pieza3, Some(Pieza::new(TipoPieza::Reina, Color::Blanco)));

    let pieza4 = factory_pieza("t");
    assert!(pieza4.is_some());
    assert_ne!(pieza4, Some(Pieza::new(TipoPieza::Torre, Color::Blanco)));
    assert_eq!(pieza4, Some(Pieza::new(TipoPieza::Torre, Color::Negro)));

    let pieza5 = factory_pieza("_");
    assert!(pieza5.is_none());
}
