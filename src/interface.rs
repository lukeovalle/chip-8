use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use anyhow::anyhow;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use crate::chip8;

pub struct SdlContext {
    _sdl_context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pixel_size: u32
}

pub fn initialize_sdl(pixel_size: u32) -> Result<SdlContext, anyhow::Error> {
    let sdl_context = sdl2::init().map_err(|e| anyhow!(e))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow!(e))?;

    let window = video_subsystem
        .window("Sudoku", 64 * pixel_size, 32 * pixel_size)
        .position_centered()
        //.resizable()
        .build()
        .map_err(|e| anyhow!(e))?;

    let canvas = window.into_canvas().build().map_err(|e| anyhow!(e))?;
    let event_pump = sdl_context.event_pump().map_err(|e| anyhow!(e))?;

    Ok(SdlContext {
        _sdl_context: sdl_context,
        _video_subsystem: video_subsystem,
        canvas,
        event_pump,
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
    for x in 0..64 {
        for y in 0..32 {
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






