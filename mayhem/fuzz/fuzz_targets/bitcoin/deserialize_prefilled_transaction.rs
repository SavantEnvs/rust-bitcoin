// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — port of archive deserialize_prefilled_transaction (libfuzzer-sys).
#![no_main]

use bitcoin_consensus_encoding::{decode_from_slice_unbounded, encode_to_vec};
use p2p::bip152::PrefilledTransaction;

fn do_test(data: &[u8]) {
    // We already fuzz Transactions in `./deserialize_transaction.rs`.
    let mut slice = data;
    let tx_result: Result<PrefilledTransaction, _> = decode_from_slice_unbounded(&mut slice);

    match tx_result {
        Err(_) => {}
        Ok(tx) => {
            let ser = encode_to_vec(&tx);
            assert_eq!(&ser[..], data);
        }
    }
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
