use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::{Read, Error, Seek};
use std::ops::{Index, Range, IndexMut};

/// Memory struct. Since chip16 maps ROM into memory this struct
/// represents both ROM and RAM and implements ROM related functions as well.
pub struct Memory {
    mem: [u8; 65536],

    /// ROM file header
    rom_header: [u8; 16],
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
        self.read_rom_header(&mut rom)?;

        rom.seek(std::io::SeekFrom::Start(16))?;
        rom.read(self.mem.as_mut_slice())?;

        Ok(())
    }

    fn read_rom_header(&mut self, rom: &mut File) -> Result<(), MemoryError> {
        rom.seek(std::io::SeekFrom::Start(0))?;
        rom.read(self.rom_header.as_mut_slice())?;

        match &self.rom_header[..4] {
            [b'C', b'H', b'1', b'6'] => Ok(()),
            _ => return Err(
                MemoryError {
                    kind: String::from("RomNotRecognized"),
                    message: String::from("Can't recognize ROM format"),
                }
            ),
        }
    }

    pub fn initial_pc(&self) -> u16 {
        LittleEndian::read_u16(&self.rom_header[10..13])
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            mem: [0; 65536],
            rom_header: [0; 16],
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

impl Index<Range<usize>> for Memory {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.mem[index]
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.mem[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mem[index]
    }
}