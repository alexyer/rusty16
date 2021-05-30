macro_rules! little_endian {
    ($ll:expr, $hh:expr) => {
        ((($hh as u16) << 8) | (($ll as u16) & 0x00ff))
    };
}