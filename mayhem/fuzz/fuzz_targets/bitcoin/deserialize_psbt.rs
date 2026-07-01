// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — PSBT moved out of bitcoin 0.33; pin 0.32 like upstream fuzz/.
#![no_main]

use bitcoin_fuzz::fuzz_utils::consume_random_bytes;

fn do_test(data: &[u8]) {
    let mut new_data = data;
    let bytes = consume_random_bytes(&mut new_data);
    let psbt: Result<bitcoin_0_32::psbt::Psbt, _> = bitcoin_0_32::psbt::Psbt::deserialize(bytes);
    match psbt {
        Err(_) => {}
        Ok(mut psbt) => {
            let ser = bitcoin_0_32::psbt::Psbt::serialize(&psbt);
            let deser = bitcoin_0_32::psbt::Psbt::deserialize(&ser).unwrap();
            // Since the fuzz data could order psbt fields differently, we compare to our deser/ser instead of data
            assert_eq!(ser, bitcoin_0_32::psbt::Psbt::serialize(&deser));

            let new_bytes = consume_random_bytes(&mut new_data);
            let psbt_b: Result<bitcoin_0_32::psbt::Psbt, _> =
                bitcoin_0_32::psbt::Psbt::deserialize(new_bytes);
            match psbt_b {
                Err(_) => {}
                Ok(mut psbt_b) => {
                    assert_eq!(psbt_b.combine(psbt.clone()).is_ok(), psbt.combine(psbt_b).is_ok());
                }
            }
        }
    }
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
