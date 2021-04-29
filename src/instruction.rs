use crate::enum_primitive::FromPrimitive;
use crate::opcode::Opcode;
use std::fmt;

#[derive(Debug)]
pub struct Instruction<'a>(pub &'a [u8]);

impl<'a> Instruction<'a> {
    #[inline(always)]
    pub fn opcode(&self) -> Option<Opcode> {
        Opcode::from_u8(self.0[0])
    }

    #[inline(always)]
    pub fn x(&self) -> u8 {
        self.0[1] & 0x0f
    }

    #[inline(always)]
    pub fn y(&self) -> u8 {
        (self.0[1] & 0xf0) >> 4
    }

    #[inline(always)]
    pub fn z(&self) -> u8 {
        self.0[2] & 0x0f
    }

    #[inline(always)]
    pub fn ll(&self) -> u8 {
        self.0[2]
    }

    #[inline(always)]
    pub fn hh(&self) -> u8 {
        self.0[3]
    }

    pub fn to_asm_str(&self) -> String {
        match self.opcode() {
            Some(Opcode::NOP) => String::from("NOP"),
            Some(Opcode::CLS) => String::from("CLS"),
            Some(Opcode::VBLNK) => String::from("VBLNK"),
            Some(Opcode::BGC) => format!("BGC {:02X}", self.z()),
            Some(Opcode::SPR) => format!("SPR {:02X}{:02X}", self.hh(), self.ll()),
            Some(Opcode::DRW_XY_HHLL) => format!("DRW R{:01X}, R{:01X}, {:02X}{:02X}", self.x(), self.y(), self.hh(), self.ll()),
            Some(Opcode::DRW_XYZ) => format!("DRW R{:01X}, R{:01X}, R{:01X}", self.x(), self.y(), self.z()),
            Some(Opcode::SND2) => format!("SND2 {:02X}{:02X}", self.hh(), self.ll()),
            Some(Opcode::JMP) => format!("JMP {:02X}{:02X}", self.hh(), self.ll()),
            Some(Opcode::JX) => {
                let opcode = match self.x() {
                    0x0 => "JZ",
                    0x1 => "JNZ",
                    0x2 => "JN",
                    0x3 => "JNN",
                    0x4 => "JP",
                    0x5 => "JO",
                    0x6 => "JNO",
                    0x7 => "JA",
                    0x8 => "JAE",
                    0x9 => "JB",
                    0xa => "JBE",
                    0xb => "JG",
                    0xc => "JGE",
                    0xd => "JL",
                    0xe => "JLE",
                    0xf => "RES",
                    _ => "J??"
                };

                format!("{} {:02X}{:02X}", opcode, self.hh(), self.ll())
            },
            Some(Opcode::CALL_HHLL) => format!("CALL {:02X}{:02X}", self.hh(), self.ll()),
            Some(Opcode::RET) => format!("RET"),
            Some(Opcode::LDI) => format!("LDI R{:01X}, {:02X}{:02X}", self.x(), self.hh(), self.ll()),
            Some(Opcode::LDM_HHLL) => format!("LDM SP, {:02X}{:02X}", self.hh(), self.ll()),
            Some(Opcode::LDM_R) => format!("LDM R{:01X}, R{:01X}", self.x(), self.y()),
            Some(Opcode::MOV) => format!("MOV R{:01X}, R{:01X}", self.x(), self.y()),
            Some(Opcode::STM) => format!("STM R{:01X}, {:02X}{:02X}", self.x(), self.hh(), self.ll()),
            Some(Opcode::STM_XY) => format!("STM R{:01X}, R{:01X}", self.x(), self.y()),
            Some(Opcode::ADDI) => format!("ADDI R{:01X}, {:02X}{:02X}", self.x(), self.hh(), self.ll()),
            Some(Opcode::ADD_XY) => format!("ADD R{:01X}, R{:01X}", self.x(), self.y()),
            Some(Opcode::SUBI) => format!("SUBI R{:01X}, {:02X}{:02X}", self.x(), self.hh(), self.ll()),
            Some(Opcode::SUB_XY) => format!("SUB R{:01X}, R{:01X}", self.x(), self.y()),
            Some(Opcode::CMPI) => format!("CMPI R{:01X}, {:02X}{:02X}", self.x(), self.hh(), self.ll()),
            Some(Opcode::ANDI) => format!("ANDI R{:01X}, {:02X}{:02X}", self.x(), self.hh(), self.ll()),
            Some(Opcode::AND_XY) => format!("AND R{:01X}, R{:01X}", self.x(), self.y()),
            Some(Opcode::TSTI) => format!("TSTI R{:01X}, {:02X}{:02X}", self.x(), self.hh(), self.ll()),
            Some(Opcode::OR_XY) => format!("OR R{:01X}, R{:01X}", self.x(), self.y()),
            Some(Opcode::XOR_XY) => format!("XOR R{:01X}, R{:01X}", self.x(), self.y()),
            Some(Opcode::MULI) => format!("MULI R{:01X}, {:02X}{:02X}", self.x(), self.hh(), self.ll()),
            Some(Opcode::MUL_XY) => format!("MUL R{:01X}, R{:01X}", self.x(), self.y()),
            Some(Opcode::MUL_XYZ) => format!("MUL R{:01X}, R{:01X}, R{:01X}", self.x(), self.y(), self.z()),
            Some(Opcode::DIV_XY) => format!("DIV R{:01X}, R{:01X}", self.x(), self.y()),
            Some(Opcode::SHL) => format!("SHL R{:01X}, R{:01X}", self.x(), self.z()),
            Some(Opcode::SHR) => format!("SHR R{:01X}, R{:01X}", self.x(), self.z()),
            Some(Opcode::POP) => format!("POP R{:01X}", self.x()),
            Some(Opcode::PUSHF) => format!("PUSHF"),
            _ => String::from("??")
        }
    }
}

impl<'a> fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(width) = f.width() {
            write!(f, "{:width$}",
                   format!("{: <4X}|{: <4X}|{: <4X}|{: <4X}",
                           self.0[0], self.0[1], self.0[2], self.0[3]), width = width)
        } else {
            write!(f, "{: <4X}|{: <4X}|{: <4X}|{: <4X}",
                self.0[0], self.0[1], self.0[2], self.0[3])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;

    #[test]
    fn test_x() {
        let cases = vec![
            (Instruction(&[0x20, 0x01, 0x02, 0x03]), 0x01),
            (Instruction(&[0x20, 0x0f, 0x02, 0x03]), 0x0f),
            (Instruction(&[0x20, 0xf1, 0x02, 0x03]), 0x01),
            (Instruction(&[0x20, 0xf, 0x02, 0x03]), 0x0f),
        ];

        for case in cases {
            assert_eq!(case.0.x(), case.1);
        }
    }

    #[test]
    fn test_y() {
        let cases = vec![
            (Instruction(&[0x20, 0x10, 0x02, 0x03]), 0x01),
            (Instruction(&[0x20, 0xf0, 0x02, 0x03]), 0x0f),
            (Instruction(&[0x20, 0x1f, 0x02, 0x03]), 0x01),
        ];

        for case in cases {
            assert_eq!(case.0.y(), case.1);
        }
    }

    #[test]
    #[should_panic]
    fn test_unknown_opcode() {
        Instruction(&[0xff, 0x01, 0x02, 0x03]).opcode();
    }

    #[test]
    fn test_ll() {
        let ll = Instruction(&[0x00, 0x11, 0x22, 0x33]).ll();
        assert_eq!(ll, 0x22);
    }

    #[test]
    fn test_hh() {
        let hh = Instruction(&[0x00, 0x11, 0x22, 0x33]).hh();
        assert_eq!(hh, 0x33);
    }
}