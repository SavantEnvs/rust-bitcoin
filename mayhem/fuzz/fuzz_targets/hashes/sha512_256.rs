// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — generated from ../../../fuzz/fuzz_targets/hashes/sha512_256.rs (libfuzzer-sys port).
#![no_main]

use bitcoin::hashes::{sha512_256, HashEngine};

fn do_test(data: &[u8]) {
    let mut engine = sha512_256::Hash::engine();
    engine.input(data);
    let eng_hash = sha512_256::Hash::from_engine(engine);

    let hash = sha512_256::Hash::hash(data);
    assert_eq!(hash.as_byte_array(), eng_hash.as_byte_array());
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
