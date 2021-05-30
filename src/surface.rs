use sdl2::{pixels, EventPump};
use sdl2::render::{WindowCanvas, Texture, TextureCreator, TextureAccess};
use crate::screen::{SCREEN_WIDTH, SCREEN_HEIGHT};
use sdl2::pixels::PixelFormatEnum;
use std::cell::RefCell;
use sdl2::event::EventType;

// FIXME: better name
pub trait Surface {
    fn new() -> Self;
    fn init(&mut self);
    fn cls(&mut self, bg: &Color);
    fn poll_events(&mut self);
    fn present(&mut self, new_buffer: &[[u8; SCREEN_WIDTH]; SCREEN_HEIGHT]);
}

pub struct TestSurface;
pub struct SdlSurface{
    canvas: WindowCanvas,
    events: EventPump,
    texture: RefCell<Texture<'static>>,
}

impl Surface for TestSurface {
    fn new() -> Self {
        TestSurface {}
    }
    fn init(&mut self) {}
    fn cls(&mut self, _bg: &Color) {}
    fn poll_events(&mut self) {}
    fn present(&mut self, _new_buffer: &[[u8; SCREEN_WIDTH]; SCREEN_HEIGHT]) {}
}

impl Surface for SdlSurface {
    fn new() -> Self {
        let width = SCREEN_WIDTH as u32;
        let height = SCREEN_HEIGHT as u32;

        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "Rusty16",
                width as u32,
                height as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut events = sdl_context.event_pump().unwrap();
        events.disable_event(EventType::MouseButtonDown);
        events.disable_event(EventType::MouseButtonUp);
        events.disable_event(EventType::MouseMotion);

        let creator = canvas.texture_creator();
        let texture = creator.create_texture(
            PixelFormatEnum::ARGB8888, TextureAccess::Streaming, width, height).unwrap();

        let texture = unsafe{
            std::mem::transmute::<_,Texture<'static>>(texture)
        };

        SdlSurface {
            canvas,
            events,
            texture: RefCell::new(texture),
        }
    }


    fn init(&mut self) {
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }

    fn cls(&mut self, bg: &Color) {
        let (r, g, b, a) = bg.to_tuple();
        self.canvas.set_draw_color(pixels::Color::RGBA(r, g, b, a));
        self.canvas.clear();
        self.canvas.present();
    }

    fn poll_events(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                _ => ()
            }
        }
    }

    fn present(&mut self, new_buffer: &[[u8; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
        let mut texture = self.texture.borrow_mut();
        texture.with_lock(None, |buffer, pitch| {
           for j in 0..SCREEN_HEIGHT {
               for i in 0..SCREEN_WIDTH {
                   let (r, g, b, a) = Color::from_u8(new_buffer[j][i]).to_tuple();
                   let offset = j * pitch + i * 4;
                   buffer[offset] = b;
                   buffer[offset + 1] = g;
                   buffer[offset + 2] = r;
                   buffer[offset + 3] = a;
               }
           }
        }).unwrap();

        self.canvas.clear();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }
}

// TODO (alexyer): Support dynamic palette
#[derive(Copy, Clone)]
pub enum Color {
    Transparent = 0x0,
    Black = 0x1,
    Gray = 0x2,
    Red = 0x3,
    Pink = 0x4,
    DarkBrown = 0x5,
    Brown = 0x6,
    Orange = 0x7,
    Yellow = 0x8,
    Green = 0x9,
    LightGreen = 0xa,
    DarkBlue = 0xb,
    Blue = 0xc,
    LightBlue = 0xd,
    SkyBlue = 0xe,
    White = 0xf,
    Unknown = 0xff,
}

impl Color {
    pub fn rgb(&self) -> u32 {
        match self {
            Color::Transparent => 0x0,
            Color::Black => 0x0,
            Color::Gray => 0x888888,
            Color::Red => 0xbf3932,
            Color::Pink => 0xde7aae,
            Color::DarkBrown => 0x4c3d21,
            Color::Brown => 0x905f25,
            Color::Orange => 0xe49452,
            Color::Yellow => 0xead979,
            Color::Green => 0x537a3b,
            Color::LightGreen => 0xabd54a,
            Color::DarkBlue => 0x252e38,
            Color::Blue => 0x00467f,
            Color::LightBlue => 0x68abcc,
            Color::SkyBlue => 0xbcdee4,
            Color::White => 0xffffff,
            Color::Unknown => 0x0
        }
    }

    pub fn argb(&self) -> u32 {
        match self {
            Color::Transparent | Color::Unknown => 0x0,
            _ => (0xff << 24) | self.rgb(),
        }
    }

    pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
        let rgb = self.rgb();

        let r = ((rgb & 0xff0000) >> 16) as u8;
        let g = ((rgb & 0x00ff00) >> 8) as u8;
        let b = (rgb & 0x0000ff) as u8;

        let a = match self {
            Color::Transparent => 0,
            _ => 0xff
        };

        (r, g, b, a)
    }

    pub fn from_u8(i: u8) -> Self {
        match i {
            0x0 => Color::Transparent,
            0x1 => Color::Black,
            0x2 => Color::Gray,
            0x3 => Color::Red,
            0x4 => Color::Pink,
            0x5 => Color::DarkBrown,
            0x6 => Color::Brown,
            0x7 => Color::Orange,
            0x8 => Color::Yellow,
            0x9 => Color::Green,
            0xa => Color::LightGreen,
            0xb => Color::DarkBlue,
            0xc => Color::Blue,
            0xd => Color::LightBlue,
            0xe => Color::SkyBlue,
            0xf => Color::White,
            _ => Color::Unknown,
        }
    }
}

impl Into<u8> for Color {
    fn into(self) -> u8 {
        match self {
            Color::Transparent => 0x0,
            Color::Black => 0x1,
            Color::Gray => 0x2,
            Color::Red => 0x3,
            Color::Pink => 0x4,
            Color::DarkBrown => 0x5,
            Color::Brown => 0x6,
            Color::Orange => 0x7,
            Color::Yellow => 0x8,
            Color::Green => 0x9,
            Color::LightGreen => 0xa,
            Color::DarkBlue => 0xb,
            Color::Blue => 0xc,
            Color::LightBlue => 0xd,
            Color::SkyBlue => 0xe,
            Color::White => 0xf,
            Color::Unknown => 0x0,
        }
    }
}
