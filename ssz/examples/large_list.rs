//! Encode and decode a list many times.
//!
//! Useful for `cargo flamegraph`.

use ssz::{Decode, Encode};
use ssz_types::{typenum::U8192, VariableList};

fn main() {
    let vec: VariableList<u64, U8192> = vec![4242; 8192].try_into().unwrap();

    let output = (0..40_000)
        .map(|_| <_>::from_ssz_bytes(&vec.as_ssz_bytes()).unwrap())
        .collect::<Vec<VariableList<u64, U8192>>>();

    println!("{}", output.len());
}
