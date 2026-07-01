// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — generated from ../../../fuzz/fuzz_targets/hashes/sha1.rs (libfuzzer-sys port).
#![no_main]

use bitcoin::hashes::{sha1, HashEngine};

fn do_test(data: &[u8]) {
    let mut engine = sha1::Hash::engine();
    engine.input(data);
    let eng_hash = sha1::Hash::from_engine(engine);

    let hash = sha1::Hash::hash(data);
    assert_eq!(hash.as_byte_array(), eng_hash.as_byte_array());
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
