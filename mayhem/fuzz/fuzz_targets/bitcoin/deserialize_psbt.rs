// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness for the mining branch: BASE still has `bitcoin::psbt` in-tree.
#![no_main]

use std::collections::BTreeMap;

use bitcoin::key::PrivateKey;
use bitcoin::psbt::Psbt;
use bitcoin::PublicKey;

fn do_test(data: &[u8]) {
    let mut psbt = match Psbt::deserialize(data) {
        Err(_) => return,
        Ok(psbt) => psbt,
    };

    let ser = Psbt::serialize(&psbt);
    let deser = Psbt::deserialize(&ser).unwrap();
    // Fuzz data could order psbt fields differently, so compare our own ser/deser/ser.
    assert_eq!(ser, Psbt::serialize(&deser));

    // Signing resolves the spend UTXO of every input, even with no keys to sign with.
    let keys: BTreeMap<PublicKey, PrivateKey> = BTreeMap::new();
    let _ = psbt.sign(&keys);
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
