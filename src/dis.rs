extern crate rusty16;

use env_logger::Env;
use std::env;
use rusty16::cpu::INSTRUCTION_SIZE;
use rusty16::instruction::Instruction;
use std::convert::TryInto;

fn main () {
    let log_env = Env::default()
        .filter_or("RUSTY16_LOG_LEVEL", "trace")
        .write_style_or("RUSTY16_LOG_STYLE", "always");

    env_logger::init_from_env(log_env);

    // TODO(alexyer): Implement proper cli.
    let filename = match env::var("RUSTY16_ROM") {
        Ok(filename) => filename,
        Err(err) => panic!("{:?}", err),
    };

    let mut mem = rusty16::memory::Memory::default();
    mem.load_rom(&filename);

    for i in (0..mem.rom_size()).step_by(4) {
        let instr = Instruction(mem[i as usize..i as usize + INSTRUCTION_SIZE ].try_into().expect(""));
        println!("0x{:04X}: {}", i, instr.to_asm_str());
    }
}