// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — generated from ../../../fuzz/fuzz_targets/hashes/json.rs (libfuzzer-sys port).
#![no_main]

use bitcoin::hashes::{ripemd160, sha1, sha256d, sha512, Hmac};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Hmacs {
    sha1: Hmac<sha1::Hash>,
    sha512: Hmac<sha512::Hash>,
}

#[derive(Deserialize, Serialize)]
struct Main {
    hmacs: Hmacs,
    ripemd: ripemd160::Hash,
    sha2d: sha256d::Hash,
}

fn do_test(data: &[u8]) {
    if let Ok(m) = serde_json::from_slice::<Main>(data) {
        let vec = serde_json::to_vec(&m).unwrap();
        assert_eq!(data, &vec[..]);
    }
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
