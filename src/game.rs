use crate::chip8::Chip8;
use crate::interface;
use interface::Action;

pub fn run(file: &str) -> Result<(), anyhow::Error> {
    let mut game_context = interface::initialize_sdl(8)?;


    let mut chip8 = Chip8::new();

    // Cargar ROM
    chip8.load_rom(file)?;

    'game: loop {
        //  manejar eventos
        match interface::check_input(&mut game_context.event_pump) {
            Some(Action::Quit) => break 'game,
            Some(Action::Press(key)) => {
                chip8.key_press(key);
            }
            Some(Action::Release(key)) => {
                chip8.key_release(key);
            }
            None => {}
        }

        //  if delay_timer > 0 : delay_timer--
        chip8.decrease_delay_timer();
        //  if sond_timre > 0 : sound_timer--
        chip8.decrease_sound_timer();


        //  avanzar emulación (correr 8 veces por iteración)
        for _ in 0..8{
            chip8.step();
        }

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
