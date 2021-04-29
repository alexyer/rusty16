use crate::surface::{Surface, Color};
use crate::memory::Memory;
use enum_primitive::FromPrimitive;
use std::panic::panic_any;

pub const SCREEN_WIDTH: usize = 320;
pub const SCREEN_HEIGHT: usize = 240;

pub struct Screen<T: Surface> {
    surface: T,

    buffer: [[u8; SCREEN_HEIGHT]; SCREEN_WIDTH],
    spritew: u8,
    spriteh: u8,
    bg: Color,
    vblank: bool,
}

impl<T: Surface> Screen<T> {
    pub fn new() -> Screen<T> {
        Screen{
            surface: T::new(),
            buffer: [[0; SCREEN_HEIGHT]; SCREEN_WIDTH],
            spritew: 0,
            spriteh: 0,
            bg: Color::Transparent,
            vblank: false,
        }
    }

    pub fn init(&mut self) {
        self.cls();
        self.surface.init();
    }

    pub fn set_vblank(&mut self) {
        self.vblank = true;
    }

    pub fn clear_vblank(&mut self) {
        self.vblank = false;
    }

    pub fn vblank(&self) -> bool {
        self.vblank
    }

    pub fn update_frame(&mut self) {
        self.surface.update_frame();
        self.surface.present(&self.buffer);
        self.set_vblank();
    }

    pub fn cls(&mut self) {
        for pixel in self.buffer.iter_mut().flat_map(|i| i.iter_mut()) {
            *pixel = 0;
        }

        self.surface.cls(&self.bg);
    }

    pub fn spr(&mut self, w: u8, h: u8) {
        self.spritew = w;
        self.spriteh = h;
    }

    // TODO (alexyer): Implement boundary checks and flips
    pub fn drw(&mut self, x: i16, y: i16, mut src: u16, mem: &Memory) {
        let mut spritew = self.spritew.wrapping_mul(2);
        let mut spriteh = self.spritew;

        if spritew >= SCREEN_WIDTH as u8 {
            spritew -= 1;
        }

        if spriteh >= SCREEN_HEIGHT as u8 {
            spriteh -= 1;
        }

        for j in y as usize..(y as u8 + spriteh) as usize {
            for i in (x as usize..(x as u8 + spritew) as usize).step_by(2) {
                self.buffer[i+1][j] = mem[src as usize] & 0x0f;
                self.buffer[i][j] = (mem[src as usize] & 0xf0) >> 4;
                src += 1;
            }
        }
    }

    pub fn bgc(&mut self, n: u8) {
        self.bg = Color::from_u8(n).unwrap_or_else(|| {
           panic!("Unknown Color: {:X}", n);
        });
        self.surface.cls(&self.bg);
        self.update_frame();
    }
}

#[cfg(test)]
mod tests {
    use crate::screen::{Screen, SCREEN_WIDTH, SCREEN_HEIGHT};
    use crate::surface::TestSurface;
    use crate::memory::Memory;

    #[test]
    fn test_cls() {
        let mut screen = Screen::<TestSurface>::new();
        for pixel in screen.buffer.iter_mut().flat_map(|i| i.iter_mut()) {
            *pixel = 1;
        }

        screen.cls();

        for pixel in screen.buffer.iter_mut().flat_map(|i| i.iter_mut()) {
            assert_eq!(*pixel, 0);
        }
    }

    #[test]
    fn test_drw() {
        let mut screen = Screen::<TestSurface>::new();
        screen.spriteh = 3;
        screen.spritew = 1;

        let mut mem = Memory::default();
        mem[42] = 0xba;
        mem[43] = 0xdc;
        mem[44] = 0xfe;

        screen.drw(3, 4, 42, &mem);


        assert_eq!(screen.buffer[3][4], 0x0b);
        assert_eq!(screen.buffer[4][4], 0x0a);
        assert_eq!(screen.buffer[3][5], 0x0d);
        assert_eq!(screen.buffer[4][5], 0x0c);
        assert_eq!(screen.buffer[3][6], 0x0f);
        assert_eq!(screen.buffer[4][6], 0x0e);
    }
}