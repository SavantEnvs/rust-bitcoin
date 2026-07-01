// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — generated from ../../../fuzz/fuzz_targets/bitcoin/deserialize_witness.rs (libfuzzer-sys port).
#![no_main]

use bitcoin::encoding::{decode_from_slice, encode_to_vec};
use bitcoin::witness::Witness;

fn do_test(data: &[u8]) {
    let w: Result<Witness, _> = decode_from_slice(data);
    if let Ok(witness) = w {
        let serialized = encode_to_vec(&witness);
        assert_eq!(data, &serialized[..]);
    }
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
