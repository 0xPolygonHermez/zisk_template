//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM
#![no_main]
ziskos::entrypoint!(main);

use std::convert::TryInto;
use ziskos::{read_input, write_output};

fn main() {
    // Get the input slice from ziskos
    let input: Vec<u8> = read_input();

    // Convert the byte array to a u64 number
    let n: u64 = u64::from_le_bytes(input.try_into().unwrap());

    // Initialize the first two Fibonacci numbers
    let mut a: u32 = 0;
    let mut b: u32 = 1;

    // Calculate the nth Fibonacci number
    for _ in 0..n {
        let c: u32 = a + b;
        a = b;
        b = c;
    }

    // Write the output using ziskos
    let output = format!("n:{} a:{} b:{}\n", n, a, b);
    write_output(output.as_bytes(), output.len());
}
