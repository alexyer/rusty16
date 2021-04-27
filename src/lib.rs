#![feature(array_methods)]

mod cpu;
mod memory;

pub struct Rusty16 {
    cpu: cpu::CPU,
    memory: memory::Memory,
}

impl Rusty16 {
    pub fn new() -> Self {
        Rusty16 {
            cpu: cpu::CPU::default(),
            memory: memory::Memory::default(),
        }
    }

    pub fn run_rom(&mut self, filename: &str) {
        // TODO(alexyer): Implement proper error handling.
        self.memory.load_rom(filename).unwrap();
    }
}