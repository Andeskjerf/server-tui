pub fn bytes_to_f64(bytes: Vec<u8>) -> f64 {
    f64::from_bits(u64::from_le_bytes(bytes.try_into().unwrap()))
}
