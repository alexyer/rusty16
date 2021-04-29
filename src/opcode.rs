use std::fmt;
enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        // 0x - Misc/Video/Audio
        NOP = 0x00,
        CLS = 0x01,
        VBLNK = 0x02,
        BGC = 0x03,
        SPR = 0x04,
        DRW_XY_HHLL = 0x05,
        DRW_XYZ = 0x06,
        SND2 = 0x0b,

        // 1x - Jumps
        JMP = 0x10,
        JX = 0x12,
        CALL_HHLL = 0x14,
        RET = 0x15,

        // 2x - Loads
        LDI = 0x20,
        LDM_HHLL = 0x22,
        LDM_R = 0x23,
        MOV = 0x24,

        // 3x - Stores
        STM = 0x30,
        STM_XY = 0x31,

        // 4x - Addition
        ADDI = 0x40,
        ADD_XY = 0x41,

        // 5x - Subtraction
        SUBI = 0x50,
        SUB_XY = 0x51,
        CMPI = 0x53,

        // 6x - Bitwise AND (&)
        ANDI = 0x60,
        AND_XY = 0x61,
        TSTI = 0x63,

        // 7x - Bitwise OR
        OR_XY = 0x71,

        // 8x - Bitwise XOR (^)
        XOR_XY = 0x81,

        // 9x - Multiplication
        MULI = 0x90,
        MUL_XY = 0x91,
        MUL_XYZ = 0x92,

        // Ax - Division
        DIV_XY = 0xa1,

        // Bx - Logical/Arithmetic Shifts
        SHL = 0xb0,
        SHR = 0xb1,

        // Cx - Push/Pop
        POP = 0xc1,
        PUSHF = 0xc4,
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(width) = f.width() {
            write!(f, "{:width$}", format!("{:?}", self), width = width)
        } else {
            write!(f, "{:?}", self)
        }
    }
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum JMP_TYPE {
        Z = 0x0,
        B = 0x9,
    }
}
