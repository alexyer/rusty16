use crate::enum_primitive::FromPrimitive;
use crate::opcode::Opcode;
use std::fmt;

#[derive(Debug)]
pub struct Instruction<'a>(pub &'a [u8]);

impl<'a> Instruction<'a> {
    #[inline(always)]
    pub fn opcode(&self) -> Opcode {
        Opcode::from_u8(self.0[0]).unwrap_or_else(|| {
           panic!("Unrecognized opcode: {:#04x}. Instruction: {:X?}", self.0[0], self.0)
        })
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