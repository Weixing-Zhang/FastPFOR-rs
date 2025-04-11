#![cfg(feature = "rust")]

use std::io::Cursor;
use fastpfor::rust::{FastPFOR, Integer, DEFAULT_PAGE_SIZE, BLOCK_SIZE_128};

#[test]
fn simple_fastpfor_test() {
    let input: Vec<u32> = (0..BLOCK_SIZE_128).collect(); // works best with 128 values
    let mut output: Vec<u32> = vec![0; input.len() * 4]; // make sure it's big enough
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

    let len = output_offset.position() as u32;
    output_offset.set_position(0);

    codec
        .uncompress(
            &output,
            len,
            &mut output_offset,
            &mut decoded,
            &mut Cursor::new(0u32),
        )
        .expect("decompression failed");

    assert_eq!(input, decoded, "Input and decompressed data do not match");
}
