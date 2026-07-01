// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — port of archive deser_net_msg (libfuzzer-sys).
#![no_main]

use bitcoin_consensus_encoding::decode_from_slice_unbounded;
use p2p::message::V1NetworkMessage;

fn do_test(data: &[u8]) {
    let mut slice = data;
    let _: Result<V1NetworkMessage, _> = decode_from_slice_unbounded(&mut slice);
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
