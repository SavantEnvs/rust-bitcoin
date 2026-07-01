// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — generated from ../../../fuzz/fuzz_targets/bitcoin/script_bytes_to_asm_fmt.rs (libfuzzer-sys port).
#![no_main]

use std::fmt::{self, Write as _};

// faster than String, we don't need to actually produce the value, just check absence of panics
struct NullWriter;

impl fmt::Write for NullWriter {
    fn write_str(&mut self, _s: &str) -> fmt::Result { Ok(()) }

    fn write_char(&mut self, _c: char) -> fmt::Result { Ok(()) }
}

fn do_test(data: &[u8]) {
    let mut writer = NullWriter;
    let script = bitcoin::WitnessScript::from_bytes(data);
    write!(writer, "{script}").unwrap();
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
