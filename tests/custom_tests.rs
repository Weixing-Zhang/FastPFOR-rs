#![cfg(feature = "rust")]

use std::io::Cursor;
use fastpfor::rust::{Codec, FastPFOR, VariableByte, JustCopy, Integer, FastPForResult, DEFAULT_PAGE_SIZE, BLOCK_SIZE_128};


/// Dynamically create a codec based on user input
fn create_codec(name: &str) -> Codec {
    match name {
        "fastpfor" => FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128).into(),
        "variablebyte" => VariableByte::new().into(),
        "justcopy" => JustCopy::new().into(),
        _ => panic!("Unknown codec type: {}", name),
    }
}

/// Compress and decompress using a codec selected at runtime
fn compress_decompress_with(codec_name: &str, input: &[u32]) -> FastPForResult<Vec<u32>> {
    let mut codec = create_codec(codec_name);

    let mut compressed = vec![0u32; input.len() * 2]; // over-allocate
    let mut compressed_offset = Cursor::new(0u32);
    let mut input_offset = Cursor::new(0u32);

    codec.compress(
        input,
        input.len() as u32,
        &mut input_offset,
        &mut compressed,
        &mut compressed_offset,
    )?;

    let compressed_len = compressed_offset.position() as usize;

    let mut decompressed = vec![0u32; input.len()]; // assume exact output
    let mut decompress_offset = Cursor::new(0u32);
    let mut compressed_read_offset = Cursor::new(0u32);

    codec.uncompress(
        &compressed[..compressed_len],
        compressed_len as u32,
        &mut compressed_read_offset,
        &mut decompressed,
        &mut decompress_offset,
    )?;

    Ok(decompressed)
}


#[test]
fn test_dynamic_codec_switching() {
    let input: Vec<u32> = (0..BLOCK_SIZE_128).collect(); 

    for codec in &["fastpfor", "variablebyte", "justcopy"] {
        let result = compress_decompress_with(codec, &input)
            .expect(&format!("Codec '{}' failed", codec));

        assert_eq!(result[..input.len()], input, "Mismatch using codec '{}'", codec);
        println!("✅ {} passed! Output: {:?}", codec, result);
    }
}


#[test]
fn simple_fastpfor_test() {
    let input: Vec<u32> = (0..BLOCK_SIZE_128).collect();
    let mut output: Vec<u32> = vec![0; input.len()*2]; 
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
    let mut output: Vec<u32> = vec![0; input.len()*2]; 
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