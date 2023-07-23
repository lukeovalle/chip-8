use crate::chip8::Chip8;
use crate::interface;

pub fn run() -> Result<(), anyhow::Error> {
    let mut game_context = interface::initialize_sdl(8)?;


    let mut chip8 = Chip8::new();

    // Cargar ROM
    chip8.load_rom()?;

    'game: loop {
    //ciclo:
    //  manejar eventos
    //
    //  if delay_timer > 0 : delay_timer--
    //  if sond_timre > 0 : sound_timer--
    //
    //  avanzar emulación (correr 8 veces por iteración)
    chip8.step();

    //  dibujar pantalla
    if let Err(e) = interface::render_window(
        &mut game_context,
        &chip8.screen
    ) {
        eprintln!("{}", e);
    }
    //  esperar 16ms (??)
    //fin del ciclo
    }

    Ok(())
}
