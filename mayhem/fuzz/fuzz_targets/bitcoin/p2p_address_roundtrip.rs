// SPDX-License-Identifier: CC0-1.0
// Additive Mayhem harness — port of archive p2p_address_roundtrip (libfuzzer-sys).
#![no_main]

use std::convert::TryFrom;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use bitcoin_consensus_encoding::decode_from_slice;
use p2p::address::AddrV2;

fn do_test(data: &[u8]) {
    if data.len() < 2 {
        return;
    }

    let addr_v2 = if let Ok(addr) = decode_from_slice::<AddrV2>(data) {
        addr
    } else {
        return;
    };

    if let Ok(ip_addr) = IpAddr::try_from(addr_v2.clone()) {
        let round_trip: AddrV2 = AddrV2::from(ip_addr);
        assert_eq!(addr_v2, round_trip, "AddrV2 -> IpAddr -> AddrV2 should round-trip correctly");
    }

    if let Ok(ip_addr) = Ipv4Addr::try_from(addr_v2.clone()) {
        let round_trip: AddrV2 = AddrV2::from(ip_addr);
        assert_eq!(addr_v2, round_trip, "AddrV2 -> Ipv4Addr -> AddrV2 should round-trip correctly");
    }

    if let Ok(ip_addr) = Ipv6Addr::try_from(addr_v2.clone()) {
        let round_trip: AddrV2 = AddrV2::from(ip_addr);
        assert_eq!(addr_v2, round_trip, "AddrV2 -> Ipv6Addr -> AddrV2 should round-trip correctly");
    }
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    do_test(data);
});
