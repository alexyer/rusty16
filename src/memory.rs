use std::fs;
use std::fs::File;
use std::io::{Read, Error};

/// Memory struct. Since chip16 maps ROM into memory this struct
/// represents both ROM and RAM and implements ROM related functions as well.
pub struct Memory {
    mem: [u8; 65536],
}

#[derive(Debug)]
pub struct MemoryError {
    kind: String,
    message: String,
}

impl Memory {
    // TODO(alexyer): Implement ROM size and checksum checks.
    pub fn load_rom(&mut self, rom_path: &str) -> Result<(), MemoryError> {
        let mut rom = File::open(rom_path)?;

        let mut rom_header = [0; 16];
        rom.read(rom_header.as_mut_slice())?;

        match &rom_header[..4] {
            [b'C', b'H', b'1', b'6'] => (),
            _ => return Err(
                MemoryError {
                    kind: String::from("RomNotRecognized"),
                    message: String::from("Can't recognize ROM format"),
                }
            ),
        };

        rom.read(self.mem.as_mut_slice())?;

        Ok(())
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            mem: [0; 65536],
        }
    }
}

impl From<std::io::Error> for MemoryError {
    fn from(err: Error) -> Self {
        MemoryError {
            kind: String::from("RomNotFound"),
            message: err.to_string(),
        }
    }
}