mod entidades;
mod imagenes;
mod input;

use entidades::Juego;
use input::leer_tecla;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Invaders".to_string(), // CAMBIO: antes decía "Space Invaders Web"
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut juego = Juego::new();

    loop {
        let dt = get_frame_time();
        juego.actualizar(dt);

        if let Some(tecla) = leer_tecla() {
            juego.procesar_tecla(tecla);
        }

        juego.dibujar();
        next_frame().await;
    }
}
