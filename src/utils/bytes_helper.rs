pub fn bytes_to_f64(bytes: Vec<u8>) -> f64 {
    f64::from_bits(u64::from_le_bytes(bytes.try_into().unwrap()))
}

pub fn bytes_to_i64(bytes: Vec<u8>) -> i64 {
    i64::from_le_bytes(bytes.try_into().unwrap())
}
