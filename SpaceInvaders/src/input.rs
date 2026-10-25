use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tecla {
    FlechaArriba,
    FlechaAbajo,
    FlechaIzquierda,
    FlechaDerecha,
    Espacio,
    Enter,
    P,
    Q,
    Otro,
}

pub fn leer_tecla() -> Option<Tecla> {
    if let Some(key) = get_last_key_pressed() {
        return match key {
            KeyCode::Up => Some(Tecla::FlechaArriba),
            KeyCode::Down => Some(Tecla::FlechaAbajo),
            KeyCode::Left => Some(Tecla::FlechaIzquierda),
            KeyCode::Right => Some(Tecla::FlechaDerecha),
            KeyCode::A => Some(Tecla::FlechaIzquierda),
            KeyCode::D => Some(Tecla::FlechaDerecha),
            KeyCode::Space => Some(Tecla::Espacio),
            KeyCode::Enter => Some(Tecla::Enter),
            KeyCode::P => Some(Tecla::P),
            KeyCode::Q => Some(Tecla::Q),
            _ => Some(Tecla::Otro),
        };
    }
    None
}