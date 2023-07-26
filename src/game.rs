use crate::chip8::Chip8;
use crate::interface;
use interface::Action;
use std::time::{Duration, Instant};

pub fn run(file: &str) -> Result<(), anyhow::Error> {
    let mut game_context = interface::initialize_sdl(8)?;
    let time_per_frame = Duration::new(1, 0) / 60; // 60 fps


    let mut chip8 = Chip8::new();
    // Cargar ROM
    chip8.load_rom(file)?;

    'game: loop {
        let now = Instant::now();

        //  manejar eventos
        match interface::check_input(&mut game_context.event_pump) {
            Some(Action::Quit) => break 'game,
            Some(Action::Press(key)) => {
                println!("Pressed: {}", key);
                chip8.key_press(key);
            },
            Some(Action::Release(key)) => {
                println!("Released: {}", key);

                chip8.key_release(key);
            },
            None => {},
        }

        // decrease timers
        chip8.decrease_delay_timer();
        chip8.decrease_sound_timer();

        // play sounds
        if chip8.sound_timer() > 0 {
            interface::play_sound(&game_context.sound_device);
        } else {
            interface::stop_sound(&game_context.sound_device);
        }


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
        std::thread::sleep(time_per_frame.saturating_sub(now.elapsed()));
    }

    Ok(())
}
