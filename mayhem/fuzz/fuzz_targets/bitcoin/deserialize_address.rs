// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — generated from ../../../fuzz/fuzz_targets/bitcoin/deserialize_address.rs (libfuzzer-sys port).
#![no_main]


fn do_test(data: &[u8]) {
    let data_str = String::from_utf8_lossy(data);
    let addr = match data_str.parse::<bitcoin::address::Address<_>>() {
        Ok(addr) => addr.assume_checked(),
        Err(_) => return,
    };
    assert_eq!(addr.to_string(), data_str);
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
