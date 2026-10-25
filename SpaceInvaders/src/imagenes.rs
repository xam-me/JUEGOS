// ==================== IMÁGENES DEL JUEGO ====================
// CAMBIO (nuevo archivo): aquí se cargan TODAS las imágenes del juego.
// Las imágenes están en la carpeta "imagenes/" y se incrustan en el .exe
// al compilar (include_bytes!). Para cambiar el diseño: reemplaza el PNG
// con el MISMO nombre (puede tener otra resolución) y vuelve a compilar.
use macroquad::prelude::*;

pub struct Imagenes {
    pub nave: Texture2D,
    pub disparo_jugador: Texture2D,
    pub disparo_enemigo: Texture2D,
    pub enemigo_basico: Texture2D,
    pub enemigo_medio: Texture2D,
    pub enemigo_elite: Texture2D,
    pub estrella: Texture2D,
    // CAMBIO: fondos y paneles (también son imágenes editables)
    pub fondo_juego: Texture2D,
    pub fondo_inicio: Texture2D,
    pub fondo_game_over: Texture2D,
    pub panel_pausa: Texture2D,
    pub barra_hud: Texture2D,
}

impl Imagenes {
    pub fn new() -> Self {
        Self {
            nave: Self::cargar(include_bytes!("../imagenes/nave.png")),
            disparo_jugador: Self::cargar(include_bytes!("../imagenes/disparo_jugador.png")),
            disparo_enemigo: Self::cargar(include_bytes!("../imagenes/disparo_enemigo.png")),
            enemigo_basico: Self::cargar(include_bytes!("../imagenes/enemigo_basico.png")),
            enemigo_medio: Self::cargar(include_bytes!("../imagenes/enemigo_medio.png")),
            enemigo_elite: Self::cargar(include_bytes!("../imagenes/enemigo_elite.png")),
            estrella: Self::cargar(include_bytes!("../imagenes/estrella.png")),
            fondo_juego: Self::cargar(include_bytes!("../imagenes/fondo_juego.png")),
            fondo_inicio: Self::cargar(include_bytes!("../imagenes/fondo_inicio.png")),
            fondo_game_over: Self::cargar(include_bytes!("../imagenes/fondo_game_over.png")),
            panel_pausa: Self::cargar(include_bytes!("../imagenes/panel_pausa.png")),
            barra_hud: Self::cargar(include_bytes!("../imagenes/barra_hud.png")),
        }
    }

    fn cargar(bytes: &[u8]) -> Texture2D {
        Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png))
    }

    // CAMBIO: dibuja una imagen en un rectángulo (x, y, ancho, alto)
    pub fn dibujar_rect(textura: &Texture2D, x: f32, y: f32, ancho: f32, alto: f32) {
        draw_texture_ex(
            textura,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(ancho, alto)),
                ..Default::default()
            },
        );
    }

    // Dibuja una imagen centrada en (cx, cy) con el tamaño indicado.
    pub fn dibujar_centrada(textura: &Texture2D, cx: f32, cy: f32, ancho: f32, alto: f32, color: Color) {
        draw_texture_ex(
            textura,
            cx - ancho / 2.0,
            cy - alto / 2.0,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(ancho, alto)),
                ..Default::default()
            },
        );
    }
}
