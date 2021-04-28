use std::fmt;

#[derive(Default, Debug)]
pub struct CpuFlags(pub u8);

macro_rules! flag {
    ($set_flag:ident, $clear_flag:ident, $flag:ident, $i:expr) => {
        pub fn $set_flag (&mut self) { self.0 |= 0x1 << $i; }
        pub fn $clear_flag (&mut self) { self.0 &= !(0x1 << $i); }
        pub fn $flag (&mut self) -> bool { self.0 & (0x1 << $i) > 0 }
    };
}

impl CpuFlags {
    flag!(set_c, clear_c, c, 1);
    flag!(set_z, clear_z, z, 2);
    flag!(set_o, clear_o, o, 6);
    flag!(set_n, clear_n, n, 7);

    pub fn check_n(&mut self, val: i16) {
        if val < 0 {
            self.set_n();
        } else {
            self.clear_n();
        }
    }

    pub fn check_z(&mut self, val: i16) {
        if val == 0 {
            self.set_z();
        } else {
            self.clear_z();
        }
    }
}

impl fmt::Display for CpuFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(width) = f.width() {
            write!(f, "{:width$}", format!("{:<08b}", self.0), width = width)
        } else {
            write!(f, "{:<08b}", self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::flags::CpuFlags;

    #[test]
    fn test_check_n() {
        let mut flags = CpuFlags::default();

        flags.check_n(-2);
        assert!(flags.n());

        flags.check_n(0);
        assert!(!flags.n());
    }

    #[test]
    fn test_check_z() {
        let mut flags = CpuFlags::default();

        flags.check_z(2);
        assert!(!flags.z());

        flags.check_z(0);
        assert!(flags.z());
    }
}