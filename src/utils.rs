pub fn get_hex_digits(opcode: u16) -> [u8; 4] {
    let mut digits_array: [u8; 4] = [0; 4];

    for i in 0..4 {
        let hex: u16 = 0x10;
        digits_array[i] = ((opcode / hex.pow(i as u32)) % hex) as u8;
    }
    digits_array.reverse();

    digits_array
}
