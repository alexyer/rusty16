enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        // 0x - Misc/Video/Audio
        NOP = 0x00,
        CLS = 0x01,
        SPR = 0x04,

        // 1x - Jumps
        JMP = 0x10,
        JX = 0x12,
        CALL_HHLL = 0x14,
        RET = 0x15,

        // 2x - Loads
        LDI = 0x20,
        LDM_R = 0x23,

        // 6x - Bitwise AND (&)
        ANDI = 0x60,
    }
}