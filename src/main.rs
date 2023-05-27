mod bitstream;
mod to_monochrome_png;
use bitstream::BitArray;

fn main() {
    // A bunch of constants
    // L variables in UPC and EAN
    let l_digits = [
        0b0001101, 0b0011001, 0b0010011, 0b0111101, 0b0100011, 0b0110001, 0b0101111, 0b0111011,
        0b0110111, 0b0001011,
    ];

    // // G variables in EAN-13
    // let g_digits = [
    //     0b0100111, 0b0110011, 0b0011011, 0b0100001, 0b0011101, 0b0111001, 0b0000101, 0b0010001,
    //     0b0001001, 0b0010111,
    // ];

    // R variables in UPC and EAN
    let r_digits = [
        0b1110010, 0b1100110, 0b1101100, 0b1000010, 0b1011100, 0b1001110, 0b1010000, 0b1000100,
        0b1001000, 0b1110100,
    ];
    const BARCODE_HEIGHT: u32 = 78;
    let barcode_width: u32 = 113;

    let mut bit_array = BitArray::new(barcode_width as usize);

    bit_array.seek(8);
    bit_array.add_bits(0b0101, 4);

    for i in 1..=6 {
        bit_array.add_bits(l_digits[i % 10], 7);
    }

    bit_array.add_bits(0b01010, 5);

    for i in 7..=12 {
        bit_array.add_bits(r_digits[i % 10], 7);
    }

    bit_array.add_bits(0b1010, 4);
    bit_array.seek(8);

    let byte_array = bit_array.get_bytes();

    to_monochrome_png::write_png(123456789012, BARCODE_HEIGHT, barcode_width, byte_array);
}
