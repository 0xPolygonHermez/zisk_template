//! A simple program that takes a number `n` as input, and computes the SHA256 hash `n` times.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM
#![no_main]
ziskos::entrypoint!(main);

use sha2::{Digest, Sha256};
use std::convert::TryInto;
use ziskos::{read_input, set_output};
use byteorder::ByteOrder;

fn main() {
    // Get the input slice from ziskos
    let input: Vec<u8> = read_input();

    // Convert the byte array to a u64 number
    let n: u64 = u64::from_le_bytes(input.try_into().unwrap());

    let mut out = [0u8; 32];

    for _ in 0..n {
        let mut hasher = Sha256::new();
        hasher.update(out);
        let digest = &hasher.finalize();
        out = Into::<[u8; 32]>::into(*digest);
    }

    // Write the output using ziskos
    for i in 0..8 {
        let val = byteorder::BigEndian::read_u32(&mut out[i*4..i*4+4]);
        set_output(i, val);
    }
}
