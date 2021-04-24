pub fn get_hex_digits(opcode: u16) -> [u8; 4] {
    let mut digits_array: [u8; 4] = [0; 4];

    for i in 0..4 {
        // Shifts by 4 and takes only last 4
        let hex_digit = (opcode >> i * 4) & 0x0F;
        digits_array[i] = hex_digit as u8;
    }
    digits_array.reverse();

    digits_array
}
