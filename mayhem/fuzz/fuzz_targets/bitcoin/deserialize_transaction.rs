// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — generated from ../../../fuzz/fuzz_targets/bitcoin/deserialize_transaction.rs (libfuzzer-sys port).
#![no_main]

use bitcoin::encoding::{decode_from_slice, encode_to_vec};
use bitcoin::transaction::TransactionExt as _;

fn do_test(data: &[u8]) {
    let tx_result: Result<bitcoin::transaction::Transaction, _> =
        decode_from_slice(data);
    match tx_result {
        Err(_) => {}
        Ok(mut tx) => {
            let ser = encode_to_vec(&tx);
            assert_eq!(&ser[..], data);
            let len = ser.len();
            let calculated_weight = tx.weight().to_wu() as usize;
            for input in &mut tx.inputs {
                input.witness = bitcoin::witness::Witness::default();
            }
            let no_witness_len = encode_to_vec(&tx).len();
            // For 0-input transactions, `no_witness_len` will be incorrect because
            // we serialize as SegWit even after "stripping the witnesses". We need
            // to drop two bytes (i.e. eight weight). Similarly, calculated_weight is
            // incorrect and needs 2 wu removing for the marker/flag bytes.
            if tx.inputs.is_empty() {
                assert_eq!(no_witness_len * 3 + len - 8, calculated_weight - 2);
            } else {
                assert_eq!(no_witness_len * 3 + len, calculated_weight);
            }
        }
    }
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
