use std::fmt;

pub struct BitArray {
    byte_array: Vec<u8>,
    cursor: usize,
}

impl BitArray {
    pub fn new(length_bits: usize) -> Self {
        // Always round up
        let mut length_bytes = length_bits / 8;

        if length_bits % 8 != 0 {
            length_bytes += 1;
        }

        Self {
            byte_array: vec![0; length_bytes],
            cursor: 0,
        }
    }

    pub fn add_bits(&mut self, bits: u8, length: usize) {
        // The cursor currently points to where we want the new data to *start* (addressed by bits)
        // First, we need to change the address to be by bytes and offset
        // This can be optimized using some kind of overflowing bitshift, probably
        let byte_cursor = self.cursor / 8;
        let bit_offset = (self.cursor % 8) as u8;

        // shift the bits left depending on the cursor's bit offset, and the length of the input
        // We want the beginning of the new data to be 16 - bit_offset
        // We subtract length from this, because we want to shift relative to the right side of the bits
        let shifted_bits = (bits as u16) << 16 - bit_offset - (length as u8);
        let [lhs_bits, rhs_bits]: [u8; 2] = shifted_bits.to_be_bytes();

        self.byte_array[byte_cursor] |= lhs_bits;
        self.byte_array[byte_cursor + 1] |= rhs_bits;

        self.cursor += length;
    }

    // pub fn seek(&mut self, n: usize) {
    //     self.cursor += n;
    // }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.byte_array
    }
}

impl fmt::Display for BitArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", print_bits(&self.byte_array))
    }
}

fn print_bits(my_bits: &[u8]) -> String {
    let mut out_string = String::new();

    for &my_byte in my_bits {
        let mut bitmask: u8 = 0b10000000;
        while bitmask > 0 {
            out_string.push(if (bitmask & my_byte) != 0u8 {
                '\u{2588}'
            } else {
                ' '
            });
            bitmask >>= 1;
        }
    }

    return out_string;
}
