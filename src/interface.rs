use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use anyhow::anyhow;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::chip8;
use crate::chip8::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::collections::HashMap;

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };

            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}


pub enum Action {
    Quit,
    Press(u8),
    Release(u8)
}

pub struct SdlContext {
    _sdl_context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    _audio_subsystem: sdl2::AudioSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    _desired_spec: AudioSpecDesired,
    pub sound_device: AudioDevice<SquareWave>,
    pixel_size: u32,
}

pub fn initialize_sdl(pixel_size: u32) -> Result<SdlContext, anyhow::Error> {
    let sdl_context = sdl2::init().map_err(|e| anyhow!(e))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow!(e))?;
    let audio_subsystem = sdl_context.audio().map_err(|e| anyhow!(e))?;

    let window = video_subsystem
        .window("CHIP-8",
            SCREEN_WIDTH as u32 * pixel_size,
            SCREEN_HEIGHT as u32 * pixel_size)
        .position_centered()
        //.resizable()
        .build()
        .map_err(|e| anyhow!(e))?;

    let canvas = window.into_canvas().build().map_err(|e| anyhow!(e))?;
    let event_pump = sdl_context.event_pump().map_err(|e| anyhow!(e))?;

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1),  // mono
        samples: None,      // default sample size
    };

    let sound_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        SquareWave {
            phase_inc: 800.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25,
        }
    }).map_err(|e| anyhow!(e))?;

    Ok(SdlContext {
        _sdl_context: sdl_context,
        _video_subsystem: video_subsystem,
        _audio_subsystem: audio_subsystem,
        canvas,
        event_pump,
        _desired_spec: desired_spec,
        sound_device,
        pixel_size
    })
}


pub fn render_window(
    sdl: &mut SdlContext,
    screen: &chip8::Screen
) -> Result<(), anyhow::Error> {
    let white = Color::WHITE;
    let black = Color::BLACK;

    sdl.canvas.set_draw_color(black);
    sdl.canvas.clear();


    //dibujar todo
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let color = match screen.get_pixel((x, y)) {
                true => white,
                false => black,
            };

            let x = x as u32;
            let y = y as u32;

            //esquinas del pixel
            let x_1 = (x * sdl.pixel_size) as i16;
            let x_2 = (x * sdl.pixel_size + sdl.pixel_size) as i16;
            let y_1 = (y * sdl.pixel_size) as i16;
            let y_2 = (y * sdl.pixel_size + sdl.pixel_size) as i16;

            sdl.canvas
                .box_(x_1, y_1, x_2, y_2, color)
                .map_err(|e| anyhow!(e))?;
        }
    }

    sdl.canvas.present();
    
    Ok(())
}

pub fn check_input(event_pump: &mut sdl2::EventPump) -> Option<Action> {
    let keys: HashMap<Keycode, u8> = HashMap::from([
        (Keycode::Num1, 0x1),
        (Keycode::Num2, 0x2),
        (Keycode::Num3, 0x3),
        (Keycode::Num4, 0xC),
        (Keycode::Q, 0x4),
        (Keycode::W, 0x5),
        (Keycode::E, 0x6),
        (Keycode::R, 0xD),
        (Keycode::A, 0x7),
        (Keycode::S, 0x8),
        (Keycode::D, 0x9),
        (Keycode::F, 0xE),
        (Keycode::Z, 0xA),
        (Keycode::X, 0x0),
        (Keycode::C, 0xB),
        (Keycode::V, 0xF)
    ]);


    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Some(Action::Quit)
            },
            Event::KeyDown { keycode: Some(key), .. } => {
                return check_key_down(&keys, key)
            }
            Event::KeyUp { keycode: Some(key), .. } => {
                return check_key_up(&keys, key)
            }
            _ => {}
        }
    }
    None
}

fn check_key_down(keys: &HashMap<Keycode, u8>, key: Keycode) -> Option<Action> {
    keys.get(&key).map(|&x| Action::Press(x))
}

fn check_key_up(keys: &HashMap<Keycode, u8>, key: Keycode) -> Option<Action> {
    keys.get(&key).map(|&x| Action::Release(x))
}

pub fn play_sound(sound_device: &AudioDevice<SquareWave>) {
    sound_device.resume();
}

pub fn stop_sound(sound_device: &AudioDevice<SquareWave>) {
    sound_device.pause();
}

