use std::collections::HashMap;

use crate::{casilla::Coordenadas, pieza::Color};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TipoPieza {
    Rey,
    Reina,
    Peon,
    Alfil,
    Torre,
    Caballo,
}

/// Representa al tipo de pieza de ajedrez
impl TipoPieza {
    /// Devuelve TRUE si la pieza ubicada en las coordenadas de origen
    /// pueden capturar a cualquier pieza ubicada en la coordenada de destino
    /// según el color que le corresponda
    ///
    /// # Argumentos
    /// * `color` - Color de la pieza
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
    pub fn puede_capturar(
        &self,
        color: &Color,
        origen: &Coordenadas,
        destino: &Coordenadas,
    ) -> bool {
        let mapeo = TipoPieza::hash_piezas(color, origen, destino);
        let puede_capturar_mapeo = mapeo.get(self);
        if let Some(puede_capturar) = puede_capturar_mapeo {
            *puede_capturar
        } else {
            false
        }
    }

    /// Mapeo de las piezas y los métodos de captura
    ///
    /// # Argumentos
    /// * `color` - Color de la pieza
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
    fn hash_piezas(
        color: &Color,
        origen: &Coordenadas,
        destino: &Coordenadas,
    ) -> HashMap<TipoPieza, bool> {
        let mut mapeo = HashMap::new();
        mapeo.insert(TipoPieza::Rey, TipoPieza::captura_rey(origen, destino));
        mapeo.insert(TipoPieza::Reina, TipoPieza::captura_reina(origen, destino));
        mapeo.insert(
            TipoPieza::Peon,
            TipoPieza::captura_peon(color, origen, destino),
        );
        mapeo.insert(
            TipoPieza::Caballo,
            TipoPieza::captura_caballo(origen, destino),
        );
        mapeo.insert(TipoPieza::Alfil, TipoPieza::captura_alfil(origen, destino));
        mapeo.insert(TipoPieza::Torre, TipoPieza::captura_torre(origen, destino));
        mapeo
    }

    /// Devuelve TRUE si el rey ubicado en las coordenadas de origen
    /// pueden capturar a cualquier pieza ubicada en la coordenada de destino
    /// según el color que le corresponda
    ///
    /// # Argumentos
    /// * `color` - Color de la pieza
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
    fn captura_rey(origen: &Coordenadas, destino: &Coordenadas) -> bool {
        if origen == destino {
            return false;
        }
        let x_destinos_capturados = vec![origen.x - 1, origen.x, origen.x + 1];
        let y_destinos_capturados = vec![origen.y - 1, origen.y, origen.y + 1];
        if x_destinos_capturados.contains(&destino.x) && y_destinos_capturados.contains(&destino.y)
        {
            return true;
        }
        false
    }

    /// Devuelve TRUE si la reina ubicada en las coordenadas de origen
    /// pueden capturar a cualquier pieza ubicada en la coordenada de destino
    /// según el color que le corresponda
    ///
    /// # Argumentos
    /// * `color` - Color de la pieza
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
    fn captura_reina(origen: &Coordenadas, destino: &Coordenadas) -> bool {
        let diagonal = TipoPieza::captura_diagonal(origen, destino);
        let recto = TipoPieza::captura_recto(origen, destino);
        diagonal || recto
    }

    /// Devuelve TRUE si el alfil ubicado en las coordenadas de origen
    /// pueden capturar a cualquier pieza ubicada en la coordenada de destino
    /// según el color que le corresponda
    ///
    /// # Argumentos
    /// * `color` - Color de la pieza
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
    fn captura_alfil(origen: &Coordenadas, destino: &Coordenadas) -> bool {
        TipoPieza::captura_diagonal(origen, destino)
    }

    /// Devuelve TRUE si la torre ubicada en las coordenadas de origen
    /// pueden capturar a cualquier pieza ubicada en la coordenada de destino
    /// según el color que le corresponda
    ///
    /// # Argumentos
    /// * `color` - Color de la pieza
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
    fn captura_torre(origen: &Coordenadas, destino: &Coordenadas) -> bool {
        TipoPieza::captura_recto(origen, destino)
    }

    /// Devuelve TRUE si el caballo ubicado en las coordenadas de origen
    /// pueden capturar a cualquier pieza ubicada en la coordenada de destino
    /// según el color que le corresponda
    ///
    /// # Argumentos
    /// * `color` - Color de la pieza
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
    fn captura_caballo(origen: &Coordenadas, destino: &Coordenadas) -> bool {
        let delta_x = (origen.x - destino.x).abs();
        let delta_y = (origen.y - destino.y).abs();

        let captura_x = delta_x == 2 && delta_y == 1;
        let captura_y = delta_x == 1 && delta_y == 2;

        captura_x || captura_y
    }

    /// Devuelve TRUE si el peón ubicado en las coordenadas de origen
    /// pueden capturar a cualquier pieza ubicada en la coordenada de destino
    /// según el color que le corresponda
    ///
    /// # Argumentos
    /// * `color` - Color de la pieza
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
    fn captura_peon(color: &Color, origen: &Coordenadas, destino: &Coordenadas) -> bool {
        if *color == Color::Blanco {
            return TipoPieza::captura_peon_blanco(origen, destino);
        }
        if *color == Color::Negro {
            return TipoPieza::captura_peon_negro(origen, destino);
        }
        false
    }

    /// Devuelve TRUE si el peón blanco ubicado en las coordenadas de origen
    /// pueden capturar a cualquier pieza ubicada en la coordenada de destino
    /// según el color que le corresponda
    ///
    /// # Argumentos
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
    fn captura_peon_blanco(origen: &Coordenadas, destino: &Coordenadas) -> bool {
        // captura para arriba, o sea, disminuyendo el eje x
        let movimiento_x = origen.x - destino.x;
        let delta_y = (origen.y - destino.y).abs();
        movimiento_x == 1 && delta_y == 1
    }

    /// Devuelve TRUE si el peón negro ubicado en las coordenadas de origen
    /// pueden capturar a cualquier pieza ubicada en la coordenada de destino
    /// según el color que le corresponda
    ///
    /// # Argumentos
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
    fn captura_peon_negro(origen: &Coordenadas, destino: &Coordenadas) -> bool {
        // captura para abajo, o sea, aumentando el eje x
        let movimiento_x = origen.x - destino.x;
        let delta_y = (origen.y - destino.y).abs();
        movimiento_x == -1 && delta_y == 1
    }

    /// Devuelve TRUE si la pieza ubicada en las coordenadas de origen
    /// se encuentra en la misma diagonal que la coordenada de destino
    ///
    /// # Argumentos
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
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

    /// Devuelve TRUE si la pieza ubicada en las coordenadas de origen
    /// se encuentra en la misma fila o columna que la coordenada de destino
    ///
    /// # Argumentos
    /// * `origen` - Coordenada de origen de la pieza
    /// * `destino` - Coordenada de destino de la pieza
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
}

#[test]
fn test_captura_pieza() {
    test_captura_pieza_rey();
    test_captura_pieza_reina();
    test_captura_pieza_alfil();
    test_captura_pieza_torre();
    test_captura_pieza_caballo();
    test_captura_pieza_peon_blanco();
    test_captura_pieza_peon_negro();
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
fn test_captura_pieza_caballo() {
    let caballo = TipoPieza::Caballo;
    let blanco = Color::Blanco;
    let negro = Color::Negro;

    let puede_capturar1 = caballo.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 2, y: 1 },
    );
    assert!(puede_capturar1);

    let puede_capturar2 = caballo.puede_capturar(
        &blanco,
        &Coordenadas { x: 5, y: 4 },
        &Coordenadas { x: 3, y: 5 },
    );
    assert!(puede_capturar2);

    let puede_capturar3 = caballo.puede_capturar(
        &blanco,
        &Coordenadas { x: 4, y: 1 },
        &Coordenadas { x: 5, y: 3 },
    );
    assert!(puede_capturar3);

    let no_puede_capturar1 = caballo.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 1, y: 1 },
    );
    assert!(!no_puede_capturar1);

    let no_puede_capturar2 = caballo.puede_capturar(
        &negro,
        &Coordenadas { x: 5, y: 0 },
        &Coordenadas { x: 1, y: 1 },
    );
    assert!(!no_puede_capturar2);
}

#[test]
fn test_captura_pieza_peon_blanco() {
    let peon = TipoPieza::Peon;
    let blanco = Color::Blanco;

    let puede_capturar1 = peon.puede_capturar(
        &blanco,
        &Coordenadas { x: 4, y: 0 },
        &Coordenadas { x: 3, y: 1 },
    );
    assert!(puede_capturar1);

    let puede_capturar2 = peon.puede_capturar(
        &blanco,
        &Coordenadas { x: 2, y: 4 },
        &Coordenadas { x: 1, y: 3 },
    );
    assert!(puede_capturar2);

    let puede_capturar3 = peon.puede_capturar(
        &blanco,
        &Coordenadas { x: 2, y: 4 },
        &Coordenadas { x: 1, y: 5 },
    );
    assert!(puede_capturar3);

    let no_puede_capturar1 = peon.puede_capturar(
        &blanco,
        &Coordenadas { x: 0, y: 0 },
        &Coordenadas { x: 1, y: 1 },
    );
    assert!(!no_puede_capturar1);

    let no_puede_capturar2 = peon.puede_capturar(
        &blanco,
        &Coordenadas { x: 5, y: 0 },
        &Coordenadas { x: 1, y: 1 },
    );
    assert!(!no_puede_capturar2);

    let no_puede_capturar3 = peon.puede_capturar(
        &blanco,
        &Coordenadas { x: 7, y: 0 },
        &Coordenadas { x: 7, y: 1 },
    );
    assert!(!no_puede_capturar3);
}

#[test]
fn test_captura_pieza_peon_negro() {
    let peon = TipoPieza::Peon;
    let negro = Color::Negro;

    let puede_capturar1 = peon.puede_capturar(
        &negro,
        &Coordenadas { x: 3, y: 1 },
        &Coordenadas { x: 4, y: 0 },
    );
    assert!(puede_capturar1);

    let puede_capturar2 = peon.puede_capturar(
        &negro,
        &Coordenadas { x: 1, y: 3 },
        &Coordenadas { x: 2, y: 4 },
    );
    assert!(puede_capturar2);

    let puede_capturar3 = peon.puede_capturar(
        &negro,
        &Coordenadas { x: 1, y: 5 },
        &Coordenadas { x: 2, y: 4 },
    );
    assert!(puede_capturar3);

    let no_puede_capturar1 = peon.puede_capturar(
        &negro,
        &Coordenadas { x: 1, y: 1 },
        &Coordenadas { x: 0, y: 0 },
    );
    assert!(!no_puede_capturar1);

    let no_puede_capturar2 = peon.puede_capturar(
        &negro,
        &Coordenadas { x: 5, y: 0 },
        &Coordenadas { x: 1, y: 1 },
    );
    assert!(!no_puede_capturar2);

    let no_puede_capturar3 = peon.puede_capturar(
        &negro,
        &Coordenadas { x: 7, y: 1 },
        &Coordenadas { x: 7, y: 0 },
    );
    assert!(!no_puede_capturar3);
}
