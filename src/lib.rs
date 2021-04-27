#![feature(array_methods)]

#[macro_use]
extern crate enum_primitive;
use log::info;

#[macro_use]
mod macros;

mod cpu;
mod instruction;
mod memory;
mod opcode;

pub struct Rusty16<'a> {
    cpu: cpu::Cpu,
    memory: memory::Memory,

    rom_path: &'a str,
}

impl<'a> Rusty16<'a> {
    pub fn new() -> Self {
        Rusty16 {
            cpu: cpu::Cpu::default(),
            memory: memory::Memory::default(),
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

        info!("Starting execution");
        loop {
            self.step();
        }

    }

    pub fn step(&mut self) {
        self.cpu.exec_instruction(&mut self.memory);
    }
}