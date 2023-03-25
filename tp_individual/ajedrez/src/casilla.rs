use crate::pieza::Pieza;

#[derive(Debug)]
pub struct Casilla {
    pub coordenadas: Coordenadas,
    pub pieza: Option<Pieza>,
}

#[derive(Debug, PartialEq)]
pub struct Coordenadas {
    pub x: i8,
    pub y: i8,
}

impl Casilla {
    pub fn new(x: i8, y: i8, pieza: Option<Pieza>) -> Self {
        let coordenadas = Coordenadas { x, y };
        Casilla { coordenadas, pieza }
    }

    pub fn puedo_capturar(&self, casilla_destino: &Casilla) -> bool {
        let pieza_propia = &self
            .pieza
            .as_ref()
            .expect("Error al obtener la pieza")
            .tipo_pieza;
        let color = &self
            .pieza
            .as_ref()
            .expect("Error al obtener la pieza")
            .color;
        let origen = &self.coordenadas;
        let destino = &casilla_destino.coordenadas;

        pieza_propia.puede_capturar(color, origen, destino)
    }
}

#[test]
fn test_casilla() {
    test_casilla_gana_blanca();
    test_casilla_gana_negra();
}

#[test]
fn test_casilla_gana_blanca() {
    let reina_blanca = Pieza::new(crate::tipo_pieza::TipoPieza::Reina, crate::pieza::Color::Blanco);
    let rey_negro = Pieza::new(crate::tipo_pieza::TipoPieza::Rey, crate::pieza::Color::Negro);

    let casilla_reina_blanca = Casilla::new(0, 0, Some(reina_blanca));
    let casilla_rey_negro = Casilla::new(6, 6, Some(rey_negro));

    let gana_blanca = casilla_reina_blanca.puedo_capturar(&casilla_rey_negro);
    assert!(gana_blanca);
}

#[test]
fn test_casilla_gana_negra() {
    let torre_blanca = Pieza::new(crate::tipo_pieza::TipoPieza::Torre, crate::pieza::Color::Blanco);
    let rey_negro = Pieza::new(crate::tipo_pieza::TipoPieza::Rey, crate::pieza::Color::Negro);

    let casilla_torre_blanca = Casilla::new(2, 2, Some(torre_blanca));
    let casilla_rey_negro = Casilla::new(3, 3, Some(rey_negro));

    let gana_negro = casilla_rey_negro.puedo_capturar(&casilla_torre_blanca);
    assert!(gana_negro);
}