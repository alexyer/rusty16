#[derive(Default)]
pub struct CPU {
    pc: u16,
    sp: u16,
    r: [i16; 16],
    flags: u8,
}
