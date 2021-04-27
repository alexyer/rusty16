use log::{debug, info};
use crate::memory::Memory;
use crate::instruction::Instruction;
use std::convert::TryInto;
use crate::opcode::Opcode;

const INSTRUCTION_SIZE: usize = 4;
const STACK_ENTRY_SIZE: usize = 2;

pub struct Cpu {
    pc: u16,
    sp: u16,
    r: [i16; 16],

    // TODO(alexyer): Refactor into separate struct
    flags: u8,
}

impl Cpu {
    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
        info!("Initial program counter address set to: {:#X}", self.pc);
    }

    pub fn exec_instruction(&mut self, mem: &mut Memory) {
        let instruction = self.read_instruction(mem);
        // println!("OP: {:?}, I: {:X?}, PC: {:#X?}, SP: {:#X?}, R: {:X?}", instruction.opcode(), instruction, self.pc, self.sp, self.r);
        match instruction.opcode() {
            Opcode::NOP=> (),
            Opcode::CLS => { debug!("Missing opcode CLS, PC: {:#X?}", self.pc); self.inc_pc() },
            Opcode::SPR => { debug!("Missing opcode SPR, PC: {:#X?}", self.pc); self.inc_pc() },
            Opcode::LDI => self.ldi(instruction.x() as usize, instruction.ll(), instruction.hh()),
            Opcode::CALL_HHLL => self.call_hhll(instruction.ll(), instruction.hh(), mem),
            Opcode::LDM_R => self.ldm_r(instruction.x(), instruction.y()),
            Opcode::ANDI => self.andi(instruction.x(), instruction.ll(), instruction.hh()),
            Opcode::JMP => self.jmp(instruction.ll(), instruction.hh()),
            Opcode::JX => self.jx(instruction.x(), instruction.ll(), instruction.hh()),
            Opcode::RET => self.ret(mem),
        };
    }

    fn read_instruction<'a>(&self, mem: &'a mut Memory) -> Instruction<'a> {
        Instruction(mem[self.pc as usize..self.pc as usize + INSTRUCTION_SIZE ].try_into().expect(""))
    }

    fn set_z(&mut self) {
        self.flags |= 0x4;
    }

    fn clear_z(&mut self) {
        self.flags ^= 0x4;
    }

    fn z(&self) -> bool {
        self.flags & 0x4 > 0
    }

    fn set_n(&mut self) {
        self.flags |= 0x40;
    }

    fn clear_n(&mut self) {
        self.flags ^= 0x40;
    }

    fn n(&self) -> bool {
        self.flags & 0x40 > 0
    }

    #[inline(always)]
    fn inc_pc(&mut self) {
        self.pc += INSTRUCTION_SIZE as u16;
    }

    #[inline(always)]
    fn inc_sp(&mut self) {
        self.sp += STACK_ENTRY_SIZE as u16;
    }

    #[inline(always)]
    fn dec_sp(&mut self) {
        self.sp -= STACK_ENTRY_SIZE as u16;
    }

    #[inline(always)]
    fn ldi(&mut self, reg: usize, ll: u8, hh: u8) {
        self.r[reg] = little_endian!(ll, hh) as i16;
        self.inc_pc();
    }

    #[inline(always)]
    fn call_hhll(&mut self, ll: u8, hh: u8, mem: &mut Memory) {
        mem[self.sp as usize] = (self.pc & 0x00ff) as u8;
        mem[self.sp as usize + 1] = (self.pc >> 8) as u8;
        self.inc_sp();
        self.jmp(ll, hh);
    }

    #[inline(always)]
    fn ret(&mut self, mem: &mut Memory) {
        self.dec_sp();

        let ll = mem[self.sp as usize];
        let hh = mem[(self.sp + 1) as usize];

        self.pc = little_endian!(ll, hh) as u16;
    }

    #[inline(always)]
    fn jmp(&mut self, ll: u8, hh: u8) {
        self.pc = little_endian!(ll, hh) as u16;
    }

    #[inline(always)]
    fn jx(&mut self, x: u8, ll: u8, hh: u8) {
        match x {
            0 => if self.z() { self.jmp(ll, hh) } else { self.inc_pc() },
            _ => panic!("Unknown Jx: {}", x)
        }
    }

    #[inline(always)]
    fn ldm_r(&mut self, x: u8, y: u8) {
        self.r[x as usize] = self.r[y as usize];
        self.inc_pc();
    }

    #[inline(always)]
    fn andi(&mut self, x: u8, ll: u8, hh: u8) {
        self.r[x as usize] &= little_endian!(ll, hh) as i16;
        self.check_z(x);
        self.check_n(x);
        self.inc_pc();
    }

    fn check_n(&mut self, x: u8) {
        if self.r[x as usize] < 0 {
            self.set_n();
        } else {
            self.clear_n();
        }
    }

    fn check_z(&mut self, x: u8) {
        if self.r[x as usize] == 0 {
            self.set_z();
        } else {
            self.clear_z();
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            sp: 0xfdf0,
            pc: 0,
            r: [0; 16],
            flags: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{Cpu, INSTRUCTION_SIZE, STACK_ENTRY_SIZE};
    use crate::memory::Memory;

    #[test]
    fn test_inc_pc() {
        let mut cpu = Cpu::default();
        cpu.set_pc(0);
        cpu.inc_pc();
        assert_eq!(cpu.pc, INSTRUCTION_SIZE as u16);
    }

    #[test]
    fn test_inc_sp() {
        let mut cpu = Cpu::default();
        cpu.inc_sp();
        assert_eq!(cpu.sp, 0xfdf0 + STACK_ENTRY_SIZE as u16);
    }

    #[test]
    fn test_dec_sp() {
        let mut cpu = Cpu::default();
        cpu.sp = STACK_ENTRY_SIZE as u16;
        cpu.dec_sp();
        assert_eq!(cpu.sp, 0);
    }

    #[test]
    fn test_ldi() {
        let mut cpu = Cpu::default();

        let cases = vec![
            ((1, 0x00, 0xff), -256),
            ((1, 0xff, 0x00), 255)
        ];

        for case in cases {
            cpu.ldi(case.0.0, case.0.1, case.0.2);
            assert_eq!(cpu.r[case.0.0], case.1);
        }
    }

    #[test]
    fn test_call_hhll() {
        let mut cpu = Cpu::default();
        cpu.pc = 0xffee;

        let mut mem = Memory::default();
        cpu.call_hhll(0xad, 0xde, &mut mem);

        assert_eq!(cpu.pc, 0xdead);
        assert_eq!(cpu.sp, (0xfdf0 + STACK_ENTRY_SIZE) as u16);
        assert_eq!(mem[0xfdf0], 0xee);
        assert_eq!(mem[0xfdf1], 0xff);
    }

    #[test]
    fn test_call_ret() {
        let mut cpu = Cpu::default();
        cpu.pc = 0xffee;

        let mut mem = Memory::default();
        cpu.call_hhll(0xad, 0xde, &mut mem);
        assert_eq!(cpu.pc, 0xdead);

        cpu.ret(&mut mem);
        assert_eq!(cpu.pc, 0xffee);
    }

    #[test]
    fn test_jmp() {
        let mut cpu = Cpu::default();

        cpu.pc = 0xffee;
        cpu.jmp(0xad, 0xde);

        assert_eq!(cpu.pc, 0xdead);
    }

    #[test]
    fn test_jn() {
        let mut cpu = Cpu::default();

        cpu.pc = 0xffee;
        cpu.jx(0,0xad, 0xde);
        assert_eq!(cpu.pc, 0xffee);

        cpu.set_z();
        cpu.jx(0,0xad, 0xde);
        assert_eq!(cpu.pc, 0xdead);
    }

    #[test]
    fn test_ldm_r() {
        let mut cpu = Cpu::default();
        cpu.r[0] = 5;
        cpu.r[1] = 0;

        cpu.ldm_r(1, 0);
        assert_eq!(cpu.r[1], 5);
    }

    #[test]
    fn test_set_z() {
        let mut cpu = Cpu::default();
        cpu.set_z();
        assert_eq!((cpu.flags & 0b00000100) >> 2, 1);
    }

    #[test]
    fn test_clear_z() {
        let mut cpu = Cpu::default();
        cpu.flags = 0xae;
        cpu.clear_z();
        assert_eq!(cpu.flags, 0xaa);
    }

    #[test]
    fn test_z() {
        let mut cpu = Cpu::default();

        cpu.set_z();
        assert!(cpu.z());

        cpu.clear_z();
        assert!(!cpu.z());
    }

    #[test]
    fn test_set_n() {
        let mut cpu = Cpu::default();
        cpu.set_n();
        assert_eq!((cpu.flags & 0b01000000) >> 6, 1);
    }

    #[test]
    fn test_clear_n() {
        let mut cpu = Cpu::default();
        cpu.flags = 0xea;
        cpu.clear_n();
        assert_eq!(cpu.flags, 0xaa);
    }

    #[test]
    fn test_n() {
        let mut cpu = Cpu::default();

        cpu.set_n();
        assert!(cpu.n());

        cpu.clear_n();
        assert!(!cpu.n());
    }

    #[test]
    fn test_andi() {
        let mut cpu = Cpu::default();

        cpu.r[0] = 0x0ead;
        cpu.andi(0, 0xff, 0x00);
        assert_eq!(cpu.r[0], 0xad);

        cpu.r[0] = -8531;
        cpu.andi(0, 0x0, 0x0);
        assert_eq!(cpu.r[0], 0);
        assert!(cpu.z());
        assert!(!cpu.n());

        cpu.r[0] = -8531;
        cpu.andi(0, 0xad, 0x0);
        assert!(!cpu.z());
        assert!(cpu.n());

    }
}
