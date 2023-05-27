#![allow(non_snake_case)]

use crc32fast as crc;
use miniz_oxide::deflate::compress_to_vec_zlib;
use std::fs::File;
use std::io::prelude::Write;
use std::path::Path;

pub fn write_png(original_num: u64, height: u32, width: u32, byte_array: &[u8]) {
    // Don't know if these are UPC or most barcodes
    const PNG_MAGIC_NUMBER: [u8; 8] = [0x89u8, b'P', b'N', b'G', 0x0du8, 0x0au8, 0x1au8, 0x0au8];

    let filename = format!("{:012}.png", original_num);

    let path = Path::new(&filename);

    let mut file = File::create(path).expect("Could not open file.");

    // Headers:
    // Magic number
    file.write_all(&PNG_MAGIC_NUMBER)
        .expect("Could not write to file");

    // IHDR Chunk - Image header
    let mut IHDR_data: Vec<u8> = Vec::with_capacity(13);

    IHDR_data.extend_from_slice(&(width.to_be_bytes()));
    IHDR_data.extend_from_slice(&(height.to_be_bytes()));
    IHDR_data.extend_from_slice(&[
        1, // Bit depth: 1 (monochrome)
        0, // Color type: Grayscale (Was using 3 with a monochrome palette before, this might mean I need to flip every bit)
        0, // Compression method 0 (only accepted value)
        0, // Filter method 0 (only accepted value)
        0  // Interlace method: "no interlace"
    ]);
    write_chunk(&mut file, b"IHDR", &IHDR_data);

    // hasher = Hasher::new();
    // file.write_all(&(13u32.to_be_bytes()))
    //     .expect("Could not write to file");
    // write_with_crc(b"IHDR", &mut hasher, &mut file);
    // write_with_crc(&(width.to_be_bytes()), &mut hasher, &mut file);
    // write_with_crc(&(height.to_be_bytes()), &mut hasher, &mut file);
    // write_with_crc(
    //     &[
    //         1, // Bit depth
    //         3, // Color type (indexed)
    //         0, // Compression method
    //         0, // Filter method
    //         0, // Interlace method = "no"
    //     ],
    //     &mut hasher,
    //     &mut file,
    // );

    // let ihdr_crc:[u8; 4] = hasher.finalize().to_be_bytes();
    // file.write_all(&ihdr_crc).expect("Could not write to file");

    // Don't worry about PLTE because we are using grayscale
    // // PLTE
    // hasher = Hasher::new();
    // file.write_all(&(6u32.to_be_bytes()))
    //     .expect("Could not write to file");
    // write_with_crc(b"PLTE", &mut hasher, &mut file);
    // write_with_crc(&[0xffu8; 3], &mut hasher, &mut file); // Color 0 is white (0xffffff)
    // write_with_crc(&[0u8; 3], &mut hasher, &mut file); // Color 1 is white (0x000000)
    // let plte_crc:[u8; 4] = hasher.finalize().to_be_bytes();

    // // pHYs - Physical pixel dimensions
    // const PIXELS_PER_UNIT: [u8; 4] = 3000u32.to_be_bytes();
    // let mut pHYs_data: Vec<u8> = Vec::with_capacity(9);
    // pHYs_data.extend_from_slice(&PIXELS_PER_UNIT); // X axis
    // pHYs_data.extend_from_slice(&PIXELS_PER_UNIT); // Y axis
    // pHYs_data.push(1u8);
    // write_chunk(&mut file, b"pHYs", &pHYs_data);

    // IDAT - Image data
    // First, expand the byte array to the required height
    let IDAT_size = (byte_array.len() + 1) * (height as usize);
    let mut IDAT_uncompressed: Vec<u8> = Vec::with_capacity(IDAT_size);
    for _ in 0..height {
        IDAT_uncompressed.push(0u8);
        IDAT_uncompressed.extend_from_slice(byte_array);
    }

    // Then, compress that data (level 3 because these are very easy to compress with a low level)
    let IDAT_data = compress_to_vec_zlib(&IDAT_uncompressed, 3);
    write_chunk(&mut file, b"IDAT", &IDAT_data);

    // IEND - Image trailer - always empty
    write_chunk(&mut file, b"IEND", &[]);

}

fn write_chunk(file: &mut File, name: &[u8], data: &[u8]) {
    let length: [u8; 4] = (data.len() as u32).to_be_bytes();
    file.write_all(&length).expect("Could not write to file");

    file.write_all(name).expect("Could not write to file");

    file.write_all(data).expect("Could not write to file");

    let crc_bytes:[u8; 4] = crc::hash([name, data].concat().as_slice()).to_be_bytes();
    file.write_all(&crc_bytes).expect("Could not write to file");
}
