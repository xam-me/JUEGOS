use crate::imagenes::Imagenes; // CAMBIO: imágenes en carpeta aparte
use crate::input::Tecla;
use macroquad::prelude::*;
use ::rand::Rng;

const ANCHO_VENTANA: f32 = 800.0;
const ALTO_VENTANA: f32 = 600.0;
const FILAS_ENEMIGOS: usize = 3;
const COLUMNAS_ENEMIGOS: usize = 8;
const MAX_DISPAROS: usize = 50;

// ==================== POSICIÓN ====================
#[derive(Clone, Copy, Debug)]
pub struct Posicion {
    pub x: f32,
    pub y: f32,
}

// ==================== DISPARO ====================
#[derive(Clone, Copy)]
pub struct Disparo {
    pub pos: Posicion,
    pub direccion: f32,
    pub velocidad: f32,
}

// ==================== ENEMIGO ====================
#[derive(Clone, Copy, PartialEq)]
pub enum TipoEnemigo {
    Basico,
    Medio,
    Elite,
}

#[derive(Clone, Copy)]
pub struct Enemigo {
    pub pos: Posicion,
    pub activo: bool,
    pub tipo: TipoEnemigo,
    pub puntuacion: u32,
    pub contador_movimiento: f32,
    pub salud: u32,
}
// ==================== VECTOR DE DISPAROS ====================
pub struct VectorDisparos {
    pub dimension: usize,
    pub elementos: [Disparo; MAX_DISPAROS],
}

impl VectorDisparos {
    pub fn new() -> Self {
        Self {
            dimension: 0,
            elementos: [Disparo {
                pos: Posicion { x: 0.0, y: 0.0 },
                direccion: 0.0,
                velocidad: 1.0,
            }; MAX_DISPAROS],
        }
    }

    pub fn dim(&self) -> usize {
        self.dimension
    }

    pub fn add(&mut self, disparo: Disparo) {
        if self.dimension < MAX_DISPAROS {
            self.elementos[self.dimension] = disparo;
            self.dimension += 1;
        }
    }

    pub fn actualizar(&mut self) {
        let mut i = 0;
        while i < self.dimension {
            let mut disparo = self.elementos[i];
            disparo.pos.y += disparo.direccion * disparo.velocidad * get_frame_time();

            if disparo.pos.y < 0.0 || disparo.pos.y > ALTO_VENTANA {
                for j in i..self.dimension.saturating_sub(1) {
                    self.elementos[j] = self.elementos[j + 1];
                }
                self.dimension -= 1;
            } else {
                self.elementos[i] = disparo;
                i += 1;
            }
        }
    }

    pub fn obtener_disparos(&self) -> &[Disparo] {
        &self.elementos[..self.dimension]
    }
}

// ==================== JUGADOR ====================
pub struct Jugador {
    pub pos: Posicion,
    pub vidas: u32,
    pub puntuacion: u32,
    pub velocidad: f32,
    pub invulnerable: bool,
    pub contador_invulnerable: f32,
    pub nivel: u8,
    pub ancho: f32,
}

impl Jugador {
    pub fn new() -> Self {
        Self {
            pos: Posicion {
                x: ANCHO_VENTANA / 2.0,
                y: ALTO_VENTANA - 60.0,
            },
            vidas: 3,
            puntuacion: 0,
            velocidad: 450.0,  
            invulnerable: false,
            contador_invulnerable: 0.0,
            nivel: 1,
            ancho: 30.0,
        }
    }

    pub fn mover(&mut self, tecla: Tecla) {
        let dt = get_frame_time();
        match tecla {
            Tecla::FlechaIzquierda => {
                if self.pos.x - self.ancho / 2.0 > 0.0 {
                    self.pos.x -= self.velocidad * dt;
                }
            }
            Tecla::FlechaDerecha => {
                if self.pos.x + self.ancho / 2.0 < ANCHO_VENTANA {
                    self.pos.x += self.velocidad * dt;
                }
            }
            _ => {}
        }
    }

    pub fn recibir_dano(&mut self) {
        if !self.invulnerable && self.vidas > 0 {
            self.vidas -= 1;
            self.invulnerable = true;
            self.contador_invulnerable = 1.5;
        }
    }

    pub fn actualizar_invulnerabilidad(&mut self, dt: f32) {
        if self.invulnerable {
            self.contador_invulnerable -= dt;
            if self.contador_invulnerable <= 0.0 {
                self.invulnerable = false;
            }
        }
    }

    pub fn agregar_puntuacion(&mut self, puntos: u32) {
        self.puntuacion += puntos;
        self.nivel = (self.puntuacion / 5300) as u8 + 1;
    }

    pub fn esta_vivo(&self) -> bool {
        self.vidas > 0
    }
}

// ==================== MENÚ ====================
pub struct Menu {
    pub opcion_seleccionada: usize,
    pub opciones: Vec<String>,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            opcion_seleccionada: 0,
            opciones: vec![
                "▶  INICIAR JUEGO".to_string(),
                "   SALIR".to_string(),
            ],
        }
    }

    pub fn navegar(&mut self, tecla: Tecla) {
        match tecla {
            Tecla::FlechaArriba => {
                if self.opcion_seleccionada > 0 {
                    self.opcion_seleccionada -= 1;
                    self.actualizar_opciones();
                }
            }
            Tecla::FlechaAbajo => {
                if self.opcion_seleccionada < self.opciones.len() - 1 {
                    self.opcion_seleccionada += 1;
                    self.actualizar_opciones();
                }
            }
            _ => {}
        }
    }

    fn actualizar_opciones(&mut self) {
        for i in 0..self.opciones.len() {
            if i == self.opcion_seleccionada {
                if !self.opciones[i].starts_with('▶') {
                    self.opciones[i] = format!("▶ {}", &self.opciones[i][3..]);
                }
            } else {
                if self.opciones[i].starts_with('▶') {
                    self.opciones[i] = format!("   {}", &self.opciones[i][3..]);
                }
            }
        }
    }

    pub fn seleccionar(&self) -> usize {
        self.opcion_seleccionada
    }
}

// ==================== JUEGO PRINCIPAL ====================
pub struct Juego {
    pub jugador: Jugador,
    pub disparos_jugador: VectorDisparos,
    pub disparos_enemigos: VectorDisparos,
    pub enemigos: [Enemigo; FILAS_ENEMIGOS * COLUMNAS_ENEMIGOS],
    pub num_enemigos: usize,
    pub direccion_enemigos: f32,
    pub pausado: bool,
    pub game_over: bool,
    pub contador_tiempo: f32,
    pub contador_disparos: f32,
    pub en_pantalla_titulo: bool,
    pub menu: Menu,
    pub estrellas: Vec<(f32, f32, f32)>,
    pub imagenes: Imagenes, // CAMBIO: imágenes del juego
}

impl Juego {
    pub fn new() -> Self {
        let mut juego = Self {
            jugador: Jugador::new(),
            disparos_jugador: VectorDisparos::new(),
            disparos_enemigos: VectorDisparos::new(),
            enemigos: [Enemigo {
                pos: Posicion { x: 0.0, y: 0.0 },
                activo: false,
                tipo: TipoEnemigo::Basico,
                puntuacion: 100,
                contador_movimiento: 0.0,
                salud: 0,
            }; FILAS_ENEMIGOS * COLUMNAS_ENEMIGOS],
            num_enemigos: 0,
            direccion_enemigos: 1.0,
            pausado: false,
            game_over: false,
            contador_tiempo: 0.0,
            contador_disparos: 0.0,
            en_pantalla_titulo: true,
            menu: Menu::new(),
            estrellas: Vec::new(),
            imagenes: Imagenes::new(), // CAMBIO
        };
        juego.generar_estrellas();
        juego
    }

    fn generar_estrellas(&mut self) {
        let mut rng = ::rand::thread_rng();
        for _ in 0..150 {
            let x = rng.gen_range(0.0..ANCHO_VENTANA);
            let y = rng.gen_range(0.0..ALTO_VENTANA);
            let brillo = rng.gen_range(0.3..1.0);
            self.estrellas.push((x, y, brillo));
        }
    }

    pub fn iniciar_juego(&mut self) {
        self.en_pantalla_titulo = false;
        self.crear_enemigos();
    }

    fn crear_enemigos(&mut self) {
        self.num_enemigos = 0;
        let inicio_x = 80.0;
        let espaciado_x = 70.0;
        let espaciado_y = 60.0;
        let margen_superior = 60.0;

        for fila in 0..FILAS_ENEMIGOS {
            let tipo = match fila {
                0 => TipoEnemigo::Elite,
                1 => TipoEnemigo::Medio,
                _ => TipoEnemigo::Basico,
            };
            let puntuacion = match tipo {
                TipoEnemigo::Elite => 300,
                TipoEnemigo::Medio => 200,
                TipoEnemigo::Basico => 100,
            };
            let y = margen_superior + fila as f32 * espaciado_y;
            for col in 0..COLUMNAS_ENEMIGOS {
                let x = inicio_x + col as f32 * espaciado_x;
                self.enemigos[self.num_enemigos] = Enemigo {
                    pos: Posicion { x, y },
                    activo: true,
                    tipo,
                    puntuacion,
                    contador_movimiento: 0.0,
                    salud: 2,
                };
                self.num_enemigos += 1;
            }
        }
    }
    pub fn procesar_teclas_mantenidas(&mut self) {
        if self.en_pantalla_titulo || self.game_over || self.pausado {
            return;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.jugador.mover(Tecla::FlechaIzquierda);
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.jugador.mover(Tecla::FlechaDerecha);
        }
    }

    pub fn actualizar(&mut self, dt: f32) {
        self.procesar_teclas_mantenidas();

        if self.en_pantalla_titulo || self.game_over || self.pausado {
            return;
        }

        self.contador_tiempo += dt;
        self.contador_disparos += dt;
        self.jugador.actualizar_invulnerabilidad(dt);

        self.disparos_jugador.actualizar();
        self.disparos_enemigos.actualizar();

        if self.contador_disparos > 1.5 {
            self.contador_disparos = 0.0;
            self.disparar_enemigo();
        }

        self.mover_enemigos(dt);
        self.verificar_colisiones();
        self.verificar_derrota();
    }

    fn disparar_enemigo(&mut self) {
        let mut rng = ::rand::thread_rng();
        let mut indices_activos = Vec::new();
        for i in 0..self.num_enemigos {
            if self.enemigos[i].activo {
                indices_activos.push(i);
            }
        }
        if !indices_activos.is_empty() {
            let idx = indices_activos[rng.gen_range(0..indices_activos.len())];
            let enemigo = &self.enemigos[idx];
            self.disparos_enemigos.add(Disparo {
                pos: Posicion {
                    x: enemigo.pos.x,
                    y: enemigo.pos.y + 20.0,
                },
                direccion: 1.1,
                velocidad: 200.0,
            });
        }
    }

    fn mover_enemigos(&mut self, dt: f32) {
        let mut borde = false;

        for i in 0..self.num_enemigos {
            if self.enemigos[i].activo {
                let nueva_x = self.enemigos[i].pos.x + self.direccion_enemigos * 80.0 * dt;
                if nueva_x < 20.0 || nueva_x > ANCHO_VENTANA - 20.0 {
                    borde = true;
                    break;
                }
            }
        }

        if borde {
            self.direccion_enemigos *= -1.0;
            for i in 0..self.num_enemigos {
                if self.enemigos[i].activo {
                    self.enemigos[i].pos.y += 15.0;
                    if self.enemigos[i].pos.y > self.jugador.pos.y - 40.0 {
                        self.game_over = true;
                    }
                }
            }
        } else {
            for i in 0..self.num_enemigos {
                if self.enemigos[i].activo {
                    self.enemigos[i].pos.x += self.direccion_enemigos * 80.0 * dt;

                    if self.enemigos[i].tipo == TipoEnemigo::Elite {
                        self.enemigos[i].contador_movimiento += dt;
                        if self.enemigos[i].contador_movimiento > 0.2 {
                            let oscilacion = (self.enemigos[i].contador_movimiento * 15.0).sin() * 30.0;
                            self.enemigos[i].pos.y += oscilacion * dt;
                            self.enemigos[i].contador_movimiento = 0.0;
                        }
                    }
                }
            }
        }
    }

    fn verificar_colisiones(&mut self) {
        // Disparos del jugador vs enemigos
        let mut i = 0;
        while i < self.disparos_jugador.dim() {
            let disparo = self.disparos_jugador.elementos[i];
            let mut impacto = false;
            for j in 0..self.num_enemigos {
                if self.enemigos[j].activo {
                    let dx = disparo.pos.x - self.enemigos[j].pos.x;
                    let dy = disparo.pos.y - self.enemigos[j].pos.y;
                    if dx.abs() < 20.0 && dy.abs() < 20.0 {
                        // Restar salud
                        if self.enemigos[j].salud > 0 {
                            self.enemigos[j].salud -= 1;
                        }
                        if self.enemigos[j].salud == 0 {
                            self.enemigos[j].activo = false;
                            self.jugador.agregar_puntuacion(self.enemigos[j].puntuacion);
                        }
                        impacto = true;
                        break;
                    }
                }
            }
            if impacto {
                for k in i..self.disparos_jugador.dim().saturating_sub(1) {
                    self.disparos_jugador.elementos[k] = self.disparos_jugador.elementos[k + 1];
                }
                self.disparos_jugador.dimension -= 1;
            } else {
                i += 1;
            }
        }

        // Disparos enemigos vs jugador
        let mut i = 0;
        while i < self.disparos_enemigos.dim() {
            let disparo = self.disparos_enemigos.elementos[i];
            let dx = disparo.pos.x - self.jugador.pos.x;
            let dy = disparo.pos.y - self.jugador.pos.y;
            if dx.abs() < 20.0 && dy.abs() < 20.0 {
                self.jugador.recibir_dano();
                for k in i..self.disparos_enemigos.dim().saturating_sub(1) {
                    self.disparos_enemigos.elementos[k] = self.disparos_enemigos.elementos[k + 1];
                }
                self.disparos_enemigos.dimension -= 1;
                if !self.jugador.esta_vivo() {
                    self.game_over = true;
                }
            } else {
                i += 1;
            }
        }

        // Colisión directa enemigo - jugador
        for i in 0..self.num_enemigos {
            if self.enemigos[i].activo {
                let dx = self.enemigos[i].pos.x - self.jugador.pos.x;
                let dy = self.enemigos[i].pos.y - self.jugador.pos.y;
                if dx.abs() < 25.0 && dy.abs() < 25.0 {
                    self.jugador.recibir_dano();
                    self.enemigos[i].activo = false;
                    if !self.jugador.esta_vivo() {
                        self.game_over = true;
                    }
                }
            }
        }
    }

    fn verificar_derrota(&mut self) {
        let mut enemigos_activos = 0;
        for i in 0..self.num_enemigos {
            if self.enemigos[i].activo {
                enemigos_activos += 1;
            }
        }
        if enemigos_activos == 0 {
            self.jugador.agregar_puntuacion(500);
            self.crear_enemigos();
        }
    }
    // ==================== DIBUJO ====================

    pub fn dibujar(&self) {
        clear_background(BLACK);

        // CAMBIO: fondo como imagen (inicio o juego); antes solo era negro
        let fondo = if self.en_pantalla_titulo {
            &self.imagenes.fondo_inicio
        } else {
            &self.imagenes.fondo_juego
        };
        Imagenes::dibujar_rect(fondo, 0.0, 0.0, ANCHO_VENTANA, ALTO_VENTANA);

        for (x, y, brillo) in &self.estrellas {
            let color = Color::new(1.0, 1.0, 1.0, *brillo);
            // CAMBIO: antes draw_circle(*x, *y, 1.5, color)
            Imagenes::dibujar_centrada(&self.imagenes.estrella, *x, *y, 3.0, 3.0, color);
        }

        if self.en_pantalla_titulo {
            self.dibujar_menu();
            return;
        }

        if self.game_over {
            self.dibujar_game_over();
            return;
        }

        if self.pausado {
            self.dibujar_pausa();
        }

        self.dibujar_hud();
        self.dibujar_jugador();

        for disparo in self.disparos_jugador.obtener_disparos() {
            // CAMBIO: antes draw_rectangle(... GREEN)
            draw_texture_ex(&self.imagenes.disparo_jugador, disparo.pos.x - 2.0, disparo.pos.y - 10.0, WHITE,
                DrawTextureParams { dest_size: Some(vec2(4.0, 15.0)), ..Default::default() });
        }

        for disparo in self.disparos_enemigos.obtener_disparos() {
            // CAMBIO: antes draw_rectangle(... RED)
            draw_texture_ex(&self.imagenes.disparo_enemigo, disparo.pos.x - 3.0, disparo.pos.y, WHITE,
                DrawTextureParams { dest_size: Some(vec2(6.0, 15.0)), ..Default::default() });
        }

        for i in 0..self.num_enemigos {
            if self.enemigos[i].activo {
                // CAMBIO: antes se dibujaban círculos de colores con ojos; ahora se usa la imagen de cada tipo
                let textura = match self.enemigos[i].tipo {
                    TipoEnemigo::Basico => &self.imagenes.enemigo_basico,
                    TipoEnemigo::Medio => &self.imagenes.enemigo_medio,
                    TipoEnemigo::Elite => &self.imagenes.enemigo_elite,
                };
                let tam = match self.enemigos[i].tipo {
                    TipoEnemigo::Basico => 15.0,
                    TipoEnemigo::Medio => 18.0,
                    TipoEnemigo::Elite => 22.0,
                };
                Imagenes::dibujar_centrada(textura, self.enemigos[i].pos.x, self.enemigos[i].pos.y, tam * 2.0, tam * 2.0, WHITE);
            }
        }
    }

    fn dibujar_hud(&self) {
        // CAMBIO: barra de imagen detrás del HUD
        Imagenes::dibujar_rect(&self.imagenes.barra_hud, 0.0, 0.0, ANCHO_VENTANA, 44.0);
        let texto_vidas = format!("Vidas: {}", self.jugador.vidas);
        let texto_puntos = format!("Puntos: {:04}", self.jugador.puntuacion);
        let texto_nivel = format!("Nivel: {}", self.jugador.nivel);
        let texto_enemigos = format!("Enemigos: {:02}", self.num_enemigos);

        draw_text(&texto_vidas, 20.0, 30.0, 25.0, WHITE);
        draw_text(&texto_puntos, 200.0, 30.0, 25.0, WHITE);
        draw_text(&texto_nivel, 400.0, 30.0, 25.0, WHITE);
        draw_text(&texto_enemigos, 550.0, 30.0, 25.0, WHITE);
    }

    fn dibujar_jugador(&self) {
        let pos = self.jugador.pos;
        let inv = self.jugador.invulnerable;
        let parpadeo = inv && (self.jugador.contador_invulnerable * 10.0).floor() as i32 % 2 == 0;
        if !parpadeo {
            // CAMBIO: antes se dibujaban un triángulo azul y un rectángulo gris; ahora la imagen de la nave
            draw_texture_ex(&self.imagenes.nave, pos.x - 15.0, pos.y - 20.0, WHITE,
                DrawTextureParams { dest_size: Some(vec2(30.0, 30.0)), ..Default::default() });
        }
    }

    fn dibujar_menu(&self) {
        let centro_x = ANCHO_VENTANA / 2.0;
        let centro_y = ALTO_VENTANA / 2.0;

        let titulo = "SPACE INVADERS ";
        let ancho_titulo = measure_text(titulo, None, 40, 1.0).width;
        draw_text(titulo, centro_x - ancho_titulo / 2.0, centro_y - 120.0, 40.0, WHITE);

        for (i, opcion) in self.menu.opciones.iter().enumerate() {
            let y = centro_y - 20.0 + i as f32 * 40.0;
            let ancho_op = measure_text(opcion, None, 25, 1.0).width;
            draw_text(opcion, centro_x - ancho_op / 2.0, y, 25.0, YELLOW);
        }

        let instrucciones = "↑ ↓  Navegar    ENTER  Seleccionar    Q  Salir";
        let ancho_inst = measure_text(instrucciones, None, 20, 1.0).width;
        draw_text(instrucciones, centro_x - ancho_inst / 2.0, centro_y + 100.0, 20.0, GRAY);
    }

    fn dibujar_pausa(&self) {
        // CAMBIO: panel de fondo para el texto de pausa
        Imagenes::dibujar_rect(&self.imagenes.panel_pausa, ANCHO_VENTANA / 2.0 - 240.0, ALTO_VENTANA / 2.0 - 70.0, 480.0, 140.0);
        let texto = "=== PAUSADO ===";
        let ancho = measure_text(texto, None, 40, 1.0).width;
        draw_text(texto, ANCHO_VENTANA / 2.0 - ancho / 2.0, ALTO_VENTANA / 2.0, 40.0, WHITE);
        let sub = "Presiona P para continuar";
        let ancho_sub = measure_text(sub, None, 20, 1.0).width;
        draw_text(sub, ANCHO_VENTANA / 2.0 - ancho_sub / 2.0, ALTO_VENTANA / 2.0 + 40.0, 20.0, GRAY);
    }

    fn dibujar_game_over(&self) {
        // CAMBIO: antes draw_rectangle(... negro 70%); ahora es la imagen fondo_game_over
        Imagenes::dibujar_rect(&self.imagenes.fondo_game_over, 0.0, 0.0, ANCHO_VENTANA, ALTO_VENTANA);

        let texto = "GAME OVER";
        let ancho = measure_text(texto, None, 50, 1.0).width;
        draw_text(texto, ANCHO_VENTANA / 2.0 - ancho / 2.0, ALTO_VENTANA / 2.0 - 60.0, 50.0, RED);

        let puntos = format!("Puntuación final: {:04}", self.jugador.puntuacion);
        let ancho_p = measure_text(&puntos, None, 30, 1.0).width;
        draw_text(&puntos, ANCHO_VENTANA / 2.0 - ancho_p / 2.0, ALTO_VENTANA / 2.0, 30.0, WHITE);

        let reiniciar = "Presiona ESPACIO para reiniciar    Q para salir";
        let ancho_r = measure_text(reiniciar, None, 20, 1.0).width;
        draw_text(reiniciar, ANCHO_VENTANA / 2.0 - ancho_r / 2.0, ALTO_VENTANA / 2.0 + 60.0, 20.0, GRAY);
    }

    // ==================== PROCESAMIENTO DE TECLAS ====================

    pub fn procesar_tecla(&mut self, tecla: Tecla) {
        if self.en_pantalla_titulo {
            match tecla {
                Tecla::FlechaArriba | Tecla::FlechaAbajo => {
                    self.menu.navegar(tecla);
                }
                Tecla::Enter => {
                    if self.menu.seleccionar() == 0 {
                        self.iniciar_juego();
                    } else {
                        self.en_pantalla_titulo = true;
                        self.menu.opcion_seleccionada = 0;
                        self.menu.actualizar_opciones();
                    }
                }
                Tecla::Q => {
                    self.en_pantalla_titulo = true;
                    self.menu.opcion_seleccionada = 0;
                    self.menu.actualizar_opciones();
                }
                _ => {}
            }
            return;
        }

        if self.game_over {
            match tecla {
                Tecla::Espacio => {
                    *self = Juego::new();
                    self.iniciar_juego();
                }
                Tecla::Q => {
                    *self = Juego::new();
                    self.en_pantalla_titulo = true;
                }
                _ => {}
            }
            return;
        }

        if self.pausado {
            if tecla == Tecla::P {
                self.pausado = false;
            }
            return;
        }
        match tecla {
            Tecla::Espacio => {
                if self.disparos_jugador.dim() < 5 {
                    let pos = self.jugador.pos;
                    self.disparos_jugador.add(Disparo {
                        pos: Posicion {
                            x: pos.x,
                            y: pos.y - 30.0,
                        },
                        direccion: -1.0,
                        velocidad: 350.0,
                    });
                }
            }
            Tecla::P => {
                self.pausado = !self.pausado;
            }
            Tecla::Q => {
                self.en_pantalla_titulo = true;
                self.pausado = false;
                self.game_over = false;
                *self = Juego::new();
            }
            _ => {}
        }
    }
}