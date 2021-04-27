macro_rules! little_endian {
    ($ll:tt, $hh:tt) => {
        ((($hh as u16) << 8) | (($ll as u16) & 0x00ff))
    };
}