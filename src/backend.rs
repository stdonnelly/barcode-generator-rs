use crate::to_monochrome_png;
use crate::bitstream::BitArray;
use std::io::{stdin, stdout, Write};

pub(crate) fn main() {
    const ARR_LEN: usize = 10;

    // L variables in UPC and EAN
    // 1 = white, 0 = black
    let l_digits: [u8; ARR_LEN] = [
        0b111_0010, 0b110_0110, 0b110_1100, 0b100_0010, 0b101_1100, 0b100_1110, 0b101_0000,
        0b100_0100, 0b100_1000, 0b111_0100,
    ];

    // Derive R and G from L
    let mut r_digits: [u8; ARR_LEN] = [0; ARR_LEN];
    let mut g_digits: [u8; ARR_LEN] = [0; ARR_LEN];

    for i in 0..ARR_LEN {
        // R digits are the complement of L digits
        r_digits[i] = l_digits[i] ^ 0b111_1111;

        // G digits are reversed R digits
        // rshift is because these use 7 bits, not 8
        g_digits[i] = r_digits[i].reverse_bits() >> 1;
    }

    const BARCODE_HEIGHT: u32 = 78;
    let barcode_width: u32 = 113;

    let mut bit_array = BitArray::new(barcode_width as usize);

    bit_array.add_bits(0b1111_1111, 8);
    bit_array.add_bits(0b1010, 4);

    // Fill with user-defined info
    let mut buf: String = String::new();
    print!("Enter the barcode number: ");
    stdout().flush().expect("Error while writing to console");
    stdin().read_line(&mut buf).expect("Error reading user input");

    let original_num = ints_from_str(&buf);

    if original_num.len() < 12 {
        panic!("Not enough digits");
    }

    for digit in &original_num[0..6] {
        bit_array.add_bits(l_digits[(digit % 10) as usize], 7);
    }

    bit_array.add_bits(0b10101, 5);

    for digit in &original_num[6..12] {
        bit_array.add_bits(r_digits[(digit % 10) as usize], 7);
    }

    bit_array.add_bits(0b0101, 4);
    bit_array.add_bits(0b1111_1111, 8);

    let byte_array = bit_array.get_bytes();

    to_monochrome_png::write_png(&original_num, BARCODE_HEIGHT, barcode_width, byte_array);
}

fn ints_from_str(in_str: &str) -> Vec<u8> {
    in_str.chars().filter_map(|x| {
        if let Some(digit) = x.to_digit(10) {
            Some(digit as u8)
        } else {
            None
        }
    }).collect()
}

// enum DigitType {
//     L,
//     G,
// }
