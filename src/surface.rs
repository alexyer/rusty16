use sdl2::{pixels, EventPump};
use sdl2::render::WindowCanvas;
use crate::screen::{SCREEN_WIDTH, SCREEN_HEIGHT};
use enum_primitive::FromPrimitive;
use sdl2::rect::Point;
use sdl2::pixels::PixelFormat;

// FIXME: better name
pub trait Surface {
    fn new() -> Self;
    fn init(&mut self);
    fn cls(&mut self, bg: &Color);
    fn update_frame(&mut self);
    fn present(&mut self, buffer: &[[u8; SCREEN_HEIGHT]; SCREEN_WIDTH]);
}

pub struct TestSurface;
pub struct SdlSurface {
    canvas: WindowCanvas,
    events: EventPump,
}

impl Surface for TestSurface {
    fn new() -> Self {
        TestSurface {}
    }
    fn init(&mut self) {}
    fn cls(&mut self, bg: &Color) {}
    fn update_frame(&mut self) {}
    fn present(&mut self, buffer: &[[u8; SCREEN_HEIGHT]; SCREEN_WIDTH]) {}
}

impl Surface for SdlSurface {
    fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "Rusty16",
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut events = sdl_context.event_pump().unwrap();

        SdlSurface {
            canvas,
            events,
        }
    }


    fn init(&mut self) {
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }

    fn cls(&mut self, bg: &Color) {
        let (r, g, b) = bg.to_tuple();
        self.canvas.set_draw_color(pixels::Color::RGB(r, g, b));
    }

    fn update_frame(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                _ => {}
            }
        }
    }

    fn present(&mut self, buffer: &[[u8; SCREEN_HEIGHT]; SCREEN_WIDTH]) {
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                let (r, g, b) =  Color::from_u8(buffer[i][j]).unwrap_or_else(
                    || { panic!("Unknown Color: {:X}", buffer[i][j]) }
                ).to_tuple();

                self.canvas.set_draw_color(pixels::Color::RGB(r, g, b));
                self.canvas.draw_point(Point::new(i as i32, j as i32));
            }
        }
        self.canvas.present();
    }
}

// TODO (alexyer): Support dynamic palette
enum_from_primitive! {
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
    }
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

        }
    }

    pub fn to_tuple(&self) -> (u8, u8, u8) {
        let rgb = self.rgb();

        let r = ((rgb & 0xff0000) >> 16) as u8;
        let g = ((rgb & 0x00ff00) >> 8) as u8;
        let b = (rgb & 0x0000ff) as u8;

        (r, g, b)
    }
}
