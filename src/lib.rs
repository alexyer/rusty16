#![feature(array_methods)]

#[macro_use]
extern crate enum_primitive;
extern crate sdl2;
use log::info;
use crate::surface::SdlSurface;
use std::{thread, time};
use std::time::Duration;

#[macro_use]
mod macros;

pub mod cpu;
mod flags;
pub mod instruction;
pub mod memory;
mod opcode;
mod screen;
mod surface;

pub struct Rusty16<'a> {
    cpu: cpu::Cpu,
    memory: memory::Memory,
    screen: screen::Screen<SdlSurface>,

    rom_path: &'a str,
}

impl<'a> Rusty16<'a> {
    pub fn new() -> Self {
        Rusty16 {
            cpu: cpu::Cpu::default(),
            memory: memory::Memory::default(),
            screen: screen::Screen::<SdlSurface>::new(),
            rom_path: "",
        }
    }

    pub fn rom_path(&mut self, rom_path: &'a str) -> &mut Self {
        self.rom_path = rom_path;
        self
    }

    pub fn run(&mut self) {
        info!("Loading ROM: {}", self.rom_path);

        // TODO(alexyer): Implement proper error handling.
        self.memory.load_rom(self.rom_path).unwrap();

        info!("Initializing CPU");
        self.cpu.set_pc(self.memory.initial_pc());

        info!("Initializing Screen");
        self.screen.init();

        info!("Starting execution");

        loop {
            // TODO (alexyer):Implement proper timer.
            let start = time::Instant::now();
            while time::Instant::now().duration_since(start) < time::Duration::from_secs(1) {
                let start_screen = time::Instant::now();
                while time::Instant::now().duration_since(start_screen) < time::Duration::from_micros(16666) {
                    self.step();
                }
                self.screen.poll_events();
                self.screen.update_frame();
            }
            // let mut s = String::new();
            // stdin().read_line(&mut s).unwrap();
        }

    }

    pub fn step(&mut self) {
        self.cpu.exec_instruction(&mut self.memory, &mut self.screen);
    }
}