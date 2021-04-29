use log::{debug, info};
use crate::memory::Memory;
use crate::instruction::Instruction;
use std::convert::TryInto;
use crate::opcode::{Opcode, JMP_TYPE};
use crate::flags::CpuFlags;
use enum_primitive::FromPrimitive;
use crate::screen::Screen;
use crate::surface::SdlSurface;

const INSTRUCTION_SIZE: usize = 4;
const STACK_ENTRY_SIZE: usize = 2;

pub struct Cpu {
    pc: u16,
    sp: u16,
    r: [i16; 16],

    flags: CpuFlags,
}

impl Cpu {
    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
        info!("Initial program counter address set to: {:#X}", self.pc);
    }

    pub fn exec_instruction(&mut self, mem: &mut Memory, screen: &mut Screen<SdlSurface>) {
        let instruction = self.read_instruction(mem);
        // println!("OP: {:<10} I: {:<15}, PC: {:#X?} SP: {:#X?} R: {:X?} F: {}", instruction.opcode(), instruction, self.pc, self.sp, self.r, self.flags);
        match instruction.opcode() {
            Opcode::NOP => { self.inc_pc() },
            Opcode::VBLNK => { self.vblnk(screen) },
            Opcode::CLS => { screen.cls(); self.inc_pc() },
            Opcode::BGC => { screen.bgc(instruction.z()); self.inc_pc() },
            Opcode::SPR => { screen.spr(instruction.ll() as u8, instruction.hh() as u8); self.inc_pc() },
            Opcode::DRW_XY_HHLL => { self.drw(instruction.x(), instruction.y(), instruction.ll(), instruction.hh(), &mem, screen) },
            Opcode::DRW_XYZ => { self.drw_xyz(instruction.x(), instruction.y(), instruction.z(), &mem, screen); self.inc_pc() },
            Opcode::SND2 => { debug!("Unimplemented instruction SND2"); self.inc_pc() },
            Opcode::LDI => self.ldi(instruction.x() as usize, instruction.ll(), instruction.hh()),
            Opcode::CALL_HHLL => self.call_hhll(instruction.ll(), instruction.hh(), mem),
            Opcode::LDM_R => self.ldm_r(instruction.x(), instruction.y(), mem),
            Opcode::LDM_HHLL => self.ldm_hhll(instruction.x(), instruction.ll(), instruction.hh(), mem),
            Opcode::ANDI => self.andi(instruction.x(), instruction.ll(), instruction.hh()),
            Opcode::JMP => self.jmp(instruction.ll(), instruction.hh()),
            Opcode::JX => self.jx(instruction.x(), instruction.ll(), instruction.hh()),
            Opcode::RET => self.ret(mem),
            Opcode::SUBI => self.subi(instruction.x(), instruction.ll(), instruction.hh()),
            Opcode::MULI => self.muli(instruction.x(), instruction.ll(), instruction.hh()),
            Opcode::ADDI => self.addi(instruction.x(), instruction.ll(), instruction.hh()),
            Opcode::ADD_XY => self.add_xy(instruction.x(), instruction.y()),
            Opcode::STM => self.stm(instruction.x(), instruction.ll(), instruction.hh(), mem),
            Opcode::STM_XY => self.stm_xy(instruction.x(), instruction.y(), mem),
            Opcode::AND_XY => self.and_xy(instruction.x(), instruction.y()),
            Opcode::MOV => self.mov(instruction.x(), instruction.y()),
            Opcode::TSTI => self.tsti(instruction.x(), instruction.ll(), instruction.hh()),
            Opcode::DIV_XY => self.div_xy(instruction.x(), instruction.y()),
            Opcode::MUL_XY => self.mul_xy(instruction.x(), instruction.y()),
            Opcode::MUL_XYZ => self.mul_xyz(instruction.x(), instruction.y(), instruction.z()),
            Opcode::XOR_XY => self.xor_xy(instruction.x(), instruction.y()),
            Opcode::OR_XY => self.or_xy(instruction.x(), instruction.y()),
            Opcode::SUB_XY => self.sub_xy(instruction.x(), instruction.y()),
            Opcode::CMPI => self.cmpi(instruction.x(), instruction.ll(), instruction.hh()),
            Opcode::PUSHF => self.pushf(mem),
            Opcode::POP => self.pop(instruction.x(), mem),
            Opcode::SHR => self.shr(instruction.x(), instruction.z()),
            Opcode::SHL => self.shl(instruction.x(), instruction.z()),
        };
    }

    fn read_instruction<'a>(&self, mem: &'a mut Memory) -> Instruction<'a> {
        Instruction(mem[self.pc as usize..self.pc as usize + INSTRUCTION_SIZE ].try_into().expect(""))
    }

    fn vblnk(&mut self, screen: &mut Screen<SdlSurface>) {
        if screen.vblank() {
            screen.clear_vblank();
            self.inc_pc();
        }
    }

    fn drw(&mut self, x: u8, y: u8, ll: u8, hh: u8, mem: &Memory, screen: &mut Screen<SdlSurface>) {
        screen.drw(self.r[x as usize], self.r[y as usize], little_endian!(ll, hh), mem);
        self.inc_pc();
    }

    fn drw_xyz(&mut self, x: u8, y: u8, z: u8, mem: &Memory, screen: &mut Screen<SdlSurface>) {
        screen.drw(self.r[x as usize], self.r[y as usize],self.r[z as usize] as u16, mem);
        self.inc_pc();
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
        self.inc_pc();
    }

    #[inline(always)]
    fn jmp(&mut self, ll: u8, hh: u8) {
        self.pc = little_endian!(ll, hh) as u16;
    }

    #[inline(always)]
    fn jx(&mut self, x: u8, ll: u8, hh: u8) {
        let jmp_type = JMP_TYPE::from_u8(x).unwrap_or_else(|| {
           panic!("Unrecognized JMP Type: {:#04x}", x);
        });

        match jmp_type {
            JMP_TYPE::Z => if self.flags.z() { self.jmp(ll, hh) } else { self.inc_pc() },
            JMP_TYPE::B => if self.flags.c() { self.jmp(ll, hh) } else { self.inc_pc() },
        }
    }

    fn mov(&mut self, x: u8, y: u8) {
        self.r[x as usize] = self.r[y as usize];
        self.inc_pc();
    }

    fn ldm_r(&mut self, x: u8, y: u8, mem: &mut Memory) {
        let src = (self.r[y as usize] as u16 & 0xffff) as usize;
        let res_ll = mem[src];
        let res_hh = mem[src+1];

        self.r[x as usize] = little_endian!(res_ll, res_hh) as i16;
        self.inc_pc();
    }

    fn ldm_hhll(&mut self, x: u8, ll: u8, hh: u8, mem: &mut Memory) {
        let src = little_endian!(ll, hh) as usize;
        let res_ll = mem[src];
        let res_hh = mem[src+1];

        self.r[x as usize] = little_endian!(res_ll, res_hh) as i16;
        self.inc_pc();
    }

    fn stm_op(&mut self, val: u16, dst: usize, mem: &mut Memory) {
        mem[dst] = (val & 0x00ff) as u8;
        mem[dst+1] = ((val & 0xff00) >> 8) as u8;
    }

    fn stm(&mut self, x: u8, ll: u8, hh: u8, mem: &mut Memory) {
        self.stm_op(self.r[x as usize] as u16, little_endian!(ll, hh) as usize, mem);
        self.inc_pc()
    }

    fn stm_xy(&mut self, x: u8, y: u8, mem: &mut Memory) {
        self.stm_op(self.r[x as usize] as u16, self.r[y as usize] as usize, mem);
        self.inc_pc();
    }

    fn and_op(&mut self, a: i16, b: i16) -> i16 {
        let and = a & b;

        self.flags.check_z(and);
        self.flags.check_n(and);

        and
    }

    fn tsti(&mut self, x: u8, ll: u8, hh: u8) {
        self.and_op(self.r[x as usize], little_endian!(ll, hh) as i16);
        self.inc_pc();
    }

    fn andi(&mut self, x: u8, ll: u8, hh: u8) {
        self.r[x as usize] = self.and_op(self.r[x as usize], little_endian!(ll, hh) as i16);
        self.inc_pc();
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        self.r[x as usize] = self.and_op(self.r[x as usize], self.r[y as usize]);
        self.inc_pc();
    }

    fn sub_op(&mut self, a: i16, b: i16) -> i16 {
        let sub = a.wrapping_sub(b);

        self.flags.check_n(sub);
        self.flags.check_z(sub);

        if (a as u16) < (sub as u16) {
            self.flags.set_c();
        } else {
            self.flags.clear_c();
        }

        if (sub > 0 && a < 0 && sub > 0) || (sub < 0 && a > 0 && sub < 0) {
            self.flags.set_o();
        } else {
            self.flags.clear_o();
        }

        sub
    }

    fn subi(&mut self, x: u8, ll: u8, hh: u8) {
        self.r[x as usize] = self.sub_op(self.r[x as usize], little_endian!(ll, hh) as i16);
        self.inc_pc();
    }

    fn cmpi(&mut self, x: u8, ll: u8, hh: u8) {
        self.sub_op(self.r[x as usize], little_endian!(ll, hh) as i16);
        self.inc_pc();
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        self.r[x as usize] = self.sub_op(self.r[x as usize], self.r[y as usize]);
        self.inc_pc();
    }

    fn div_op(&mut self, a: i16, b: i16) -> i16 {
        let div = a.wrapping_div(b);
        let rem = a.wrapping_rem(b);

        if rem != 0 {
            self.flags.set_c();
        } else {
            self.flags.clear_c();
        }

        self.flags.check_n(div);
        self.flags.check_z(div);

        div
    }

    fn div_xy(&mut self, x: u8, y: u8) {
        self.r[x as usize] = self.div_op(self.r[x as usize], self.r[y as usize]);
        self.inc_pc();
    }

    fn add_op(&mut self, a: i16, b: i16) -> i16 {
        let sum = (a as u32  & 0xffff) + (b as u32 & 0xffff);

        self.flags.check_n(sum as i16);
        self.flags.check_z(sum as i16);

        if sum > 0xffff {
            self.flags.set_c();
        } else {
            self.flags.clear_c();
        }

        if (sum as i16 > 0 && a < 0 && b < 0) || ((sum as i16) < 0 && a > 0 && b > 0) {
            self.flags.set_o();
        } else {
            self.flags.clear_o();
        }

        sum as i16
    }

    fn addi(&mut self, x: u8, ll: u8, hh: u8) {
        self.r[x as usize] = self.add_op(self.r[x as usize], little_endian!(ll, hh) as i16);
        self.inc_pc();
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.r[x as usize] = self.add_op(self.r[x as usize], self.r[y as usize]);
        self.inc_pc();
    }

    fn mul_op(&mut self, a: i16, b: i16) -> i16 {
        let a = a as u32 & 0xffff;
        let b = b as u32 & 0xffff;
        let mul = a * b;

        self.flags.check_z(mul as i16);
        self.flags.check_n(mul as i16);

        if mul > 0xffff {
            self.flags.set_c();
        } else {
            self.flags.clear_c();
        }

        mul as i16
    }

    fn muli(&mut self, x: u8, ll: u8, hh: u8) {
        self.r[x as usize] = self.mul_op(self.r[x as usize], little_endian!(ll, hh) as i16);
        self.inc_pc();
    }

    fn mul_xy(&mut self, x: u8, y: u8) {
        self.r[x as usize] = self.mul_op(self.r[x as usize], self.r[y as usize]);
        self.inc_pc();
    }

    fn mul_xyz(&mut self, x: u8, y: u8, z: u8) {
        self.r[z as usize] = self.mul_op(self.r[x as usize], self.r[y as usize]);
        self.inc_pc();
    }

    fn xor_op(&mut self, a: i16, b: i16) -> i16 {
        let xor = a ^ b;

        self.flags.check_z(xor);
        self.flags.check_n(xor);

        xor
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        self.r[x as usize] = self.xor_op(self.r[x as usize], self.r[y as usize]);
        self.inc_pc();
    }

    fn or_op(&mut self, a: i16, b: i16) -> i16 {
        let or = a | b;

        self.flags.check_z(or);
        self.flags.check_n(or);

        or
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        self.r[x as usize] = self.or_op(self.r[x as usize], self.r[y as usize]);
        self.inc_pc();
    }

    fn pushf(&mut self, mem: &mut Memory) {
        mem[self.sp as usize] = self.flags.into();
        mem[self.sp as usize + 1] = 0;
        self.inc_sp();
        self.inc_pc();
    }

    fn pop(&mut self, x: u8, mem: &mut Memory) {
        self.dec_sp();
        let addr = self.sp as usize;
        self.r[x as usize] = (((mem[addr + 1] as u16) << 8) | mem[addr] as u16) as i16;
        self.inc_pc();
    }

    fn shr_op(&mut self, x: i16, n: u8) -> i16 {
        let shr = x.wrapping_shr(n as u32);

        self.flags.check_n(shr);
        self.flags.check_z(shr);

        shr
    }

    fn shr(&mut self, x: u8, n: u8) {
        self.r[x as usize] = self.shr_op(self.r[x as usize], n);
        self.inc_pc();
    }

    fn shl_op(&mut self, x: i16, n: u8) -> i16 {
        let shl = x.wrapping_shl(n as u32);

        self.flags.check_n(shl);
        self.flags.check_z(shl);

        shl
    }

    fn shl(&mut self, x: u8, n: u8) {
        self.r[x as usize] = self.shl_op(self.r[x as usize], n);
        self.inc_pc();
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            sp: 0xfdf0,
            pc: 0,
            r: [0; 16],
            flags: CpuFlags::default(),
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
    fn test_stm() {
        let mut cpu = Cpu::default();
        let mut mem = Memory::default();

        cpu.r[0] = -8531;
        cpu.stm(0, 0xaa, 0xaa, &mut mem);
        assert_eq!(mem[0xaaaa], 0xad);
        assert_eq!(mem[0xaaab], 0xde);
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
        assert_eq!(cpu.pc, 0xffee + INSTRUCTION_SIZE as u16);
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
        assert_eq!(cpu.pc, 0xffee + INSTRUCTION_SIZE as u16);

        cpu.flags.set_z();
        cpu.jx(0,0xad, 0xde);
        assert_eq!(cpu.pc, 0xdead);
    }

    #[test]
    fn test_ldm_r() {
        let mut cpu = Cpu::default();
        let mut mem = Memory::default();

        cpu.r[0] = 5;
        cpu.r[1] = -6;
        mem[0xfffa] = 0xad;
        mem[0xfffb] = 0xde;

        cpu.ldm_r(0, 1, &mut mem);
        assert_eq!(cpu.r[0], -8531);
    }

    #[test]
    fn test_mov() {
        let mut cpu = Cpu::default();
        cpu.r[0] = 0;
        cpu.r[13] = 42;

        cpu.mov(0, 13);
        assert_eq!(cpu.r[0], 42);
    }

    #[test]
    fn test_ldm_hhll() {
        let mut cpu = Cpu::default();
        let mut mem = Memory::default();

        mem[0xfffa] = 0xad;
        mem[0xfffb] = 0xde;
        cpu.ldm_hhll(0, 0xfa, 0xff, &mut mem);
        assert_eq!(cpu.r[0], -8531);
    }

    #[test]
    fn test_tsti() {
        let mut cpu = Cpu::default();

        cpu.r[0] = 42;
        cpu.tsti(0, 0x00, 0x00);
        assert_eq!(cpu.r[0], 42);
        assert!(cpu.flags.z());
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
        assert!(cpu.flags.z());
        assert!(!cpu.flags.n());

        cpu.r[0] = -8531;
        cpu.andi(0, 0x00, 0xde);
        assert!(!cpu.flags.z());
        assert!(cpu.flags.n());

    }

    #[test]
    fn test_subi() {
        let mut cpu = Cpu::default();

        cpu.r[0] = 42;
        cpu.subi(0, 0x01, 0x00);
        assert_eq!(cpu.r[0], 41);
        assert!(!cpu.flags.z());
        assert!(!cpu.flags.n());
        assert!(!cpu.flags.c());
        assert!(!cpu.flags.o());

        cpu.subi(0, 0x29, 0x00);
        assert_eq!(cpu.r[0], 0);
        assert!(cpu.flags.z());
        assert!(!cpu.flags.n());
        assert!(!cpu.flags.c());
        assert!(!cpu.flags.o());

        cpu.subi(0, 0x01, 0x00);
        assert_eq!(cpu.r[0], -1);
        assert!(!cpu.flags.z());
        assert!(cpu.flags.n());
        assert!(cpu.flags.c());
        assert!(!cpu.flags.o());

        cpu.r[0] = -42;
        cpu.subi(0, 0xff, 0x7f);
        assert!(!cpu.flags.z());
        assert!(!cpu.flags.n());
        assert!(!cpu.flags.c());
        assert!(cpu.flags.o());

        cpu.r[0] = 10000;
        cpu.subi(0, 0xff, 0x8f);
        assert!(!cpu.flags.z());
        assert!(cpu.flags.n());
        assert!(cpu.flags.c());
        assert!(cpu.flags.o());
    }

    #[test]
    fn test_addi() {
        let mut cpu = Cpu::default();

        cpu.r[0] = 42;
        cpu.addi(0, 0x01, 0x00);
        assert_eq!(cpu.r[0], 43);
        assert!(!cpu.flags.z());
        assert!(!cpu.flags.n());
        assert!(!cpu.flags.c());
        assert!(!cpu.flags.o());

        cpu.r[0] = -41;
        cpu.addi(0, 0x29, 0x00);
        assert_eq!(cpu.r[0], 0);
        assert!(cpu.flags.z());
        assert!(!cpu.flags.n());
        assert!(cpu.flags.c());
        assert!(!cpu.flags.o());

        cpu.r[0] = -41;
        cpu.addi(0, 0x01, 0x00);
        assert_eq!(cpu.r[0], -40);
        assert!(!cpu.flags.z());
        assert!(cpu.flags.n());
        assert!(!cpu.flags.c());
        assert!(!cpu.flags.o());

        cpu.r[0] = 32767;
        cpu.addi(0, 0xff, 0x00);
        assert!(!cpu.flags.z());
        assert!(cpu.flags.n());
        assert!(!cpu.flags.c());
        assert!(cpu.flags.o());
    }

    #[test]
    fn test_muli() {
        let mut cpu = Cpu::default();

        cpu.r[0] = 2;
        cpu.muli(0, 0x02, 0x00);
        assert_eq!(cpu.r[0], 4);
        assert!(!cpu.flags.z());
        assert!(!cpu.flags.n());
        assert!(!cpu.flags.c());

        cpu.muli(0, 0xfe, 0xff);
        assert_eq!(cpu.r[0], -8);
        assert!(!cpu.flags.z());
        assert!(cpu.flags.n());
        assert!(cpu.flags.c());

        cpu.muli(0, 0x00, 0x00);
        assert_eq!(cpu.r[0], 0);
        assert!(cpu.flags.z());
        assert!(!cpu.flags.n());
        assert!(!cpu.flags.c());
    }

    #[test]
    fn test_mul_xyz() {
        let mut cpu = Cpu::default();

        cpu.r[0] = 2;
        cpu.r[1] = 2;
        cpu.mul_xyz(0, 1, 2);
        assert_eq!(cpu.r[2], 4);
    }

    #[test]
    fn test_div_xy() {
        let mut cpu = Cpu::default();
        cpu.r[0] = 6;
        cpu.r[1] = 2;

        cpu.div_xy(0, 1);
        assert_eq!(cpu.r[0], 3);
        assert!(!cpu.flags.c());
        assert!(!cpu.flags.z());
        assert!(!cpu.flags.n());

        cpu.r[0] = 5;
        cpu.r[1] = 3;
        cpu.div_xy(0, 1);
        assert_eq!(cpu.r[0], 1);
        assert!(cpu.flags.c());
        assert!(!cpu.flags.z());
        assert!(!cpu.flags.n());

        cpu.r[0] = -5;
        cpu.r[1] = 3;
        cpu.div_xy(0, 1);
        assert_eq!(cpu.r[0], -1);
        assert!(cpu.flags.c());
        assert!(!cpu.flags.z());
        assert!(cpu.flags.n());
    }

    #[test]
    fn test_xor_xy() {
        let mut cpu = Cpu::default();
        cpu.r[0] = 0xa;
        cpu.r[1] = 0xf;

        cpu.xor_xy(0, 1);
        assert_eq!(cpu.r[0], 0x5);
        assert!(!cpu.flags.z());
        assert!(!cpu.flags.n());

        cpu.r[0] = 0xa;
        cpu.r[1] = 0xa;

        cpu.xor_xy(0, 1);
        assert_eq!(cpu.r[0], 0);
        assert!(cpu.flags.z());
        assert!(!cpu.flags.n());

        cpu.r[0] = 0xa;
        cpu.r[1] = -10;

        cpu.xor_xy(0, 1);
        assert_eq!(cpu.r[0], -4);
        assert!(!cpu.flags.z());
        assert!(cpu.flags.n());
    }

    #[test]
    fn test_pushf() {
        let mut cpu = Cpu::default();
        let mut mem = Memory::default();

        cpu.flags.set_c();
        cpu.flags.set_o();
        cpu.flags.set_z();
        cpu.flags.set_n();

        let old_sp = cpu.sp;
        cpu.pushf(&mut mem);

        assert_eq!(cpu.sp, old_sp + STACK_ENTRY_SIZE as u16);
        assert_eq!(mem[old_sp as usize], 0b11000110);
    }

    #[test]
    fn test_pop() {
        let mut cpu = Cpu::default();
        let mut mem = Memory::default();
        mem[cpu.sp as usize] = 0xad;
        mem[cpu.sp as usize + 1] = 0xde;
        cpu.sp += STACK_ENTRY_SIZE as u16;

        cpu.pop(0, &mut mem);
        assert_eq!(cpu.r[0], -8531);
    }

    #[test]
    fn test_shr() {
        let mut cpu = Cpu::default();
        cpu.r[0] = 4;

        cpu.shr(0, 2);
        assert_eq!(cpu.r[0], 1);
        assert!(!cpu.flags.n());
        assert!(!cpu.flags.z());

        cpu.shr(0, 1);
        assert_eq!(cpu.r[0], 0);
        assert!(!cpu.flags.n());
        assert!(cpu.flags.z());
    }

    #[test]
    fn test_shl() {
        let mut cpu = Cpu::default();
        cpu.r[0] = 1;

        cpu.shl(0, 2);
        assert_eq!(cpu.r[0], 4);
        assert!(!cpu.flags.n());
        assert!(!cpu.flags.z());

        cpu.shl(0, 13);
        assert_eq!(cpu.r[0], -32768);
        assert!(cpu.flags.n());
        assert!(!cpu.flags.z());

        cpu.shl(0, 1);
        assert_eq!(cpu.r[0], 0);
        assert!(!cpu.flags.n());
        assert!(cpu.flags.z());
    }

    #[test]
    fn test_or_xy() {
        let mut cpu = Cpu::default();
        cpu.r[0] = 0xa;
        cpu.r[1] = 0xf;

        cpu.or_xy(0, 1);
        assert_eq!(cpu.r[0], 0xff);
        assert!(!cpu.flags.z());
        assert!(!cpu.flags.n());

        cpu.r[0] = 0xa;
        cpu.r[1] = 0xa;

        cpu.or_xy(0, 1);
        assert_eq!(cpu.r[0], 0xa);
        assert!(!cpu.flags.z());
        assert!(!cpu.flags.n());
    }
}
