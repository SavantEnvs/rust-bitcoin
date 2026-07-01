// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — generated from ../../../fuzz/fuzz_targets/bitcoin/deserialize_script.rs (libfuzzer-sys port).
#![no_main]

use bitcoin::address::{Address, AddressExt as _};
use bitcoin::encoding;
use bitcoin::script::{self, BuilderExt as _, PushBytesExt as _, ScriptExt as _, ScriptPubKeyExt as _};
use bitcoin::{FeeRate, Network, ScriptPubKeyBuf};
use bitcoin_fuzz::fuzz_utils::{consume_random_bytes, consume_u32};

fn do_test(data: &[u8]) {
    let mut new_data = data;
    let bytes = consume_random_bytes(&mut new_data);
    let s: Result<ScriptPubKeyBuf, _> = encoding::decode_from_slice(bytes);
    if let Ok(script) = s {
        let _: Result<Vec<script::Instruction>, script::Error> = script.instructions().collect();

        let _ = script.to_string();
        let _ = script.count_sigops();
        let _ = script.count_sigops_legacy();
        let _ = script.minimal_non_dust();

        let fee_rate = FeeRate::from_sat_per_kwu(consume_u32(&mut new_data));
        let _ = script.minimal_non_dust_custom(fee_rate);

        let mut b = script::Builder::new();
        for ins in script.instructions_minimal() {
            if ins.is_err() {
                return;
            }
            match ins.ok().unwrap() {
                script::Instruction::Op(op) => {
                    b = b.push_opcode(op);
                }
                script::Instruction::PushBytes(bytes) => {
                    // Any one-byte pushes, except -0, which can be interpreted as numbers, should be
                    // reserialized as numbers. (For -1 through 16, this will use special ops; for
                    // others it'll just reserialize them as pushes.)
                    if bytes.len() == 1 && bytes[0] != 0x80 && bytes[0] != 0x00 {
                        if let Ok(num) = bytes.read_scriptint() {
                            b = b.push_int_unchecked(i64::from(num));
                        } else {
                            b = b.push_slice(bytes);
                        }
                    } else {
                        b = b.push_slice(bytes);
                    }
                }
            }
        }
        assert_eq!(b.into_script(), script);
        assert_eq!(bytes, &encoding::encode_to_vec(script.as_script())[..]);

        // Check if valid address and if that address roundtrips.
        if let Ok(addr) = Address::from_script(&script, Network::Bitcoin) {
            assert_eq!(addr.script_pubkey(), script);
        }
    }
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
