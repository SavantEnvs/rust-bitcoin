// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — generated from ../../../fuzz/fuzz_targets/bitcoin/deserialize_block.rs (libfuzzer-sys port).
#![no_main]

use bitcoin::block::{self, Block, BlockCheckedExt as _};
use bitcoin::encoding::{decode_from_slice, encode_to_vec};

fn do_test(data: &[u8]) {
    let block_result: Result<bitcoin::block::Block, _> =
        decode_from_slice(data);

    match block_result {
        Err(_) => {}
        Ok(block) => {
            let ser = encode_to_vec(&block);
            assert_eq!(&ser[..], data);

            // Exercise merkle/witness helpers on the unchecked block data.
            let (header, transactions) = block.into_parts();
            block::compute_merkle_root(&transactions);
            block::compute_witness_root(&transactions);
            let unchecked = Block::new_unchecked(header.clone(), transactions.clone());
            let _ = unchecked.compute_witness_commitment(&[]);

            if let Ok(block) = Block::new_checked(header, transactions) {
                let _ = block.bip34_block_height();
                block.block_hash();
                block.weight();
            }
        }
    }
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
