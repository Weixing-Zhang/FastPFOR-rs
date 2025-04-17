#![cfg(feature = "rust")]

use std::io::Cursor;
use fastpfor::rust::{FastPFOR, Integer, DEFAULT_PAGE_SIZE, BLOCK_SIZE_128};

#[test]
fn simple_fastpfor_test() {
    let input: Vec<u32> = (0..BLOCK_SIZE_128).collect();
    let mut output: Vec<u32> = vec![0; input.len()]; 
    let mut decoded: Vec<u32> = vec![0; input.len()];
    let mut input_offset = Cursor::new(0u32);
    let mut output_offset = Cursor::new(0u32);
    
    let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);

    codec
        .compress(
            &input,
            input.len() as u32,
            &mut input_offset,
            &mut output,
            &mut output_offset,
        )
        .expect("compression failed");

    let compressed_len = output_offset.position() as usize;
    input_offset.set_position(0);

    codec
        .uncompress(
            &output,
            compressed_len as u32,
            &mut input_offset,
            &mut decoded,
            &mut Cursor::new(0u32),
        )
        .expect("decompression failed");
    
    println!("Input: {:?} with bytes", &input); 
    println!("Encoded: {:?}", &output);
    println!("Decoded: {:?}", &decoded);

    let original_size = input.len() * size_of::<u32>();
    let encoded_size = compressed_len * size_of::<u32>();
    let decoded_size = decoded.len() * size_of::<u32>();

    println!("Original size: {} bytes", original_size);
    println!("Encoded size: {} bytes", encoded_size);
    println!("Decoded size: {} bytes", decoded_size);

    assert_eq!(input, decoded, "Input and decompressed data do not match");
}


#[test]
fn large_data_fastpfor_test() {
    let input: Vec<u32> = (0..(DEFAULT_PAGE_SIZE*1024)).collect();
    let mut output: Vec<u32> = vec![0; input.len()]; 
    let mut decoded: Vec<u32> = vec![0; input.len()];
    let mut input_offset = Cursor::new(0u32);
    let mut output_offset = Cursor::new(0u32);
    
    let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE*2, 512);

    codec
        .compress(
            &input,
            input.len() as u32,
            &mut input_offset,
            &mut output,
            &mut output_offset,
        )
        .expect("compression failed");

    let compressed_len = output_offset.position() as usize;
    input_offset.set_position(0);

    codec
        .uncompress(
            &output,
            compressed_len as u32,
            &mut input_offset,
            &mut decoded,
            &mut Cursor::new(0u32),
        )
        .expect("decompression failed");
    
    // println!("Input: {:?} with bytes", &input); 
    // println!("Encoded: {:?}", &output);
    // println!("Decoded: {:?}", &decoded);

    let original_size = input.len() * size_of::<u32>();
    let encoded_size = compressed_len * size_of::<u32>();
    let decoded_size = decoded.len() * size_of::<u32>();

    println!("Original size: {} bytes", original_size);
    println!("Encoded size: {} bytes", encoded_size);
    println!("Decoded size: {} bytes", decoded_size);

    assert_eq!(input, decoded, "Input and decompressed data do not match");
}