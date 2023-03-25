use crate::{casilla::Coordenadas, pieza::Color};

#[derive(Debug, PartialEq)]
pub enum TipoPieza {
    Rey,
    Reina,
    Peon,
    Alfil,
    Torre,
    Caballo,
}

impl TipoPieza {
    pub fn puede_capturar(
        &self,
        _color: &Color,
        origen: &Coordenadas,
        destino: &Coordenadas,
    ) -> bool {
        if self == &TipoPieza::Rey {
            captura_rey(origen, destino)
        } else if self == &TipoPieza::Reina {
            captura_reina(origen, destino)
        } else if self == &TipoPieza::Alfil {
            captura_alfil(origen, destino)
        } else if self == &TipoPieza::Torre {
            captura_torre(origen, destino)
        } else {
            false
        }
    }
}

fn captura_rey(origen: &Coordenadas, destino: &Coordenadas) -> bool {
    if origen == destino {
        return false;
    }
    let x_destinos_capturados = vec![origen.x - 1, origen.x, origen.x + 1];
    let y_destinos_capturados = vec![origen.y - 1, origen.y, origen.y + 1];
    if x_destinos_capturados.contains(&destino.x) && y_destinos_capturados.contains(&destino.y) {
        return true;
    }
    false
}

fn captura_reina(origen: &Coordenadas, destino: &Coordenadas) -> bool {
    let diagonal = captura_diagonal(origen, destino);
    let recto = captura_recto(origen, destino);
    diagonal || recto
}

fn captura_alfil(origen: &Coordenadas, destino: &Coordenadas) -> bool {
    captura_diagonal(origen, destino)
}

fn captura_torre(origen: &Coordenadas, destino: &Coordenadas) -> bool {
    captura_recto(origen, destino)
}

fn captura_diagonal(origen: &Coordenadas, destino: &Coordenadas) -> bool {
    if origen == destino {
        return false;
    }
    if origen.x - origen.y == destino.x - destino.y {
        return true;
    }
    if origen.x + origen.y == destino.x + destino.y {
        return true;
    }
    false
}

fn captura_recto(origen: &Coordenadas, destino: &Coordenadas) -> bool {
    if origen == destino {
        return false;
    }
    if origen.x == destino.x {
        return true;
    }
    if origen.y == destino.y {
        return true;
    }
    false
}

#[test]
fn test_captura_pieza() {
    test_captura_pieza_rey();
    test_captura_pieza_reina();
    test_captura_pieza_alfil();
    test_captura_pieza_torre();
    test_captura_pieza_otros();
}

#[test]
fn test_captura_pieza_rey() {
    let rey = TipoPieza::Rey;
    let blanco = Color::Blanco;

    let puede_capturar1 = rey.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 1, y: 1 },
    );
    assert!(puede_capturar1);

    let puede_capturar2 = rey.puede_capturar(
        &blanco,
        &Coordenadas { x: 6, y: 3 },
        &Coordenadas { x: 5, y: 2 },
    );
    assert!(puede_capturar2);

    let negro = Color::Negro;
    let no_puede_capturar1 = rey.puede_capturar(
        &negro,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 2, y: 1 },
    );
    assert!(!no_puede_capturar1);

    let negro = Color::Negro;
    let no_puede_capturar2 = rey.puede_capturar(
        &negro,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 0, y: 0 },
    );
    assert!(!no_puede_capturar2);
}

#[test]
fn test_captura_pieza_reina() {
    let reina = TipoPieza::Reina;
    let blanco = Color::Blanco;
    let negro = Color::Negro;

    let puede_capturar1 = reina.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 1, y: 1 },
    );
    assert!(puede_capturar1);

    let puede_capturar2 = reina.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 0, y: 5 },
    );
    assert!(puede_capturar2);

    let puede_capturar3 = reina.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 5, y: 5 },
    );
    assert!(puede_capturar3);

    let no_puede_capturar = reina.puede_capturar(
        &negro,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 6, y: 1 },
    );
    assert!(!no_puede_capturar);
}

#[test]
fn test_captura_pieza_alfil() {
    let alfil = TipoPieza::Alfil;
    let blanco = Color::Blanco;
    let negro = Color::Negro;

    let puede_capturar1 = alfil.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 1, y: 1 },
    );
    assert!(puede_capturar1);

    let no_puede_capturar1 = alfil.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 0, y: 5 },
    );
    assert!(!no_puede_capturar1);

    let puede_capturar2 = alfil.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 5, y: 5 },
    );
    assert!(puede_capturar2);

    let puede_capturar3 = alfil.puede_capturar(
        &blanco,
        &Coordenadas { x: 5, y: 5 },
        &Coordenadas { x: 4, y: 6 },
    );
    assert!(puede_capturar3);

    let no_puede_capturar = alfil.puede_capturar(
        &negro,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 6, y: 1 },
    );
    assert!(!no_puede_capturar);
}

#[test]
fn test_captura_pieza_torre() {
    let torre = TipoPieza::Torre;
    let blanco = Color::Blanco;
    let negro = Color::Negro;

    let puede_capturar1 = torre.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 1, y: 0 },
    );
    assert!(puede_capturar1);

    let puede_capturar2 = torre.puede_capturar(
        &blanco,
        &Coordenadas { x: 4, y: 0 },
        &Coordenadas { x: 4, y: 5 },
    );
    assert!(puede_capturar2);

    let puede_capturar3 = torre.puede_capturar(
        &blanco,
        &Coordenadas { x: 3, y: 6 },
        &Coordenadas { x: 7, y: 6 },
    );
    assert!(puede_capturar3);

    let no_puede_capturar1 = torre.puede_capturar(
        &negro,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 6, y: 1 },
    );
    assert!(!no_puede_capturar1);

    let no_puede_capturar2 = torre.puede_capturar(
        &negro,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 6, y: 6 },
    );
    assert!(!no_puede_capturar2);

    let no_puede_capturar3 = torre.puede_capturar(
        &negro,
        &Coordenadas { x: 2, y: 0 },
        &Coordenadas { x: 6, y: 1 },
    );
    assert!(!no_puede_capturar3);
}

#[test]
fn test_captura_pieza_otros() {
    let peon = TipoPieza::Peon;
    let blanco = Color::Blanco;
    let negro = Color::Negro;

    let no_puede_capturar1 = peon.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 1, y: 1 },
    );
    assert!(!no_puede_capturar1);

    let no_puede_capturar2 = peon.puede_capturar(
        &negro,
        &Coordenadas { x: 5, y: 0 },
        &Coordenadas { x: 1, y: 1 },
    );
    assert!(!no_puede_capturar2);
}
