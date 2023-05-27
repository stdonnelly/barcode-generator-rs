use crc32fast::Hasher;
use miniz_oxide::deflate;
use std::fs::File;
use std::io::prelude::Write;
use std::path::Path;

pub fn write_png(original_num: u32, height: u32, width: u32, byte_array: &[u8]) {
    // Don't know if these are UPC or most barcodes
    let BARCODE_HEIGHT = 78;
    let PIXELS_PER_UNIT = 3000;
    let PNG_MAGIC_NUMBER = [0x89u8, b'P', b'N', b'G', 0x0du8, 0x0au8, 0x1au8, 0x0au8];

    let filename = format!("{:012}.png", original_num);

    let path = Path::new(&filename);

    let mut file = File::create(path).expect("Could not open file.");

    // Headers:
    // Magic number
    file.write_all(&PNG_MAGIC_NUMBER)
        .expect("Could not write to file");

    // IHDR Chunk
    let mut hasher = Hasher::new();
    file.write_all(&(13u32.to_be_bytes()))
        .expect("Could not write to file");
    write_with_crc(b"IHDR", &mut hasher, &mut file);
    write_with_crc(&(width.to_be_bytes()), &mut hasher, &mut file);
    write_with_crc(&(height.to_be_bytes()), &mut hasher, &mut file);
    write_with_crc(
        &[
            1, // Bit depth
            3, // Color type (indexed)
            0, // Compression method
            0, // Filter method
            0, // Interlace method = "no"
        ],
        &mut hasher,
        &mut file,
    );

    let ihdr_crc:[u8; 4] = hasher.finalize().to_be_bytes();
    file.write_all(&ihdr_crc).expect("Could not write to file");

    // TODO:
    // PLTE
    // Make a new hasher
    hasher = Hasher::new();

    // pHYs
    // Make a new hasher
    hasher = Hasher::new();

    // IDAT
    // Make a new hasher
    hasher = Hasher::new();

    // IEND
    // Make a new hasher
    hasher = Hasher::new();
}

fn write_with_crc(bytes: &[u8], hasher: &mut Hasher, file: &mut File) {
    hasher.update(bytes);
    file.write_all(bytes).expect("Could not write to file");
}
