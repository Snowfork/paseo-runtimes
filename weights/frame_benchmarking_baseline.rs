
//! Autogenerated weights for `frame_benchmarking::baseline`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `p-ch-hmb-vm-vapas-002`, CPU: `Intel(R) Xeon(R) Silver 4314 CPU @ 2.40GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./runtimes/chain_specs/paseo-raw.json")`, DB CACHE: 1024

// Executed Command:
// ./bin/polkadot
// benchmark
// pallet
// --chain=./runtimes/chain_specs/paseo-raw.json
// --steps=50
// --repeat=20
// --pallet=frame_benchmarking::baseline
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `frame_benchmarking::baseline`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_benchmarking::baseline::WeightInfo for WeightInfo<T> {
	/// The range of component `i` is `[0, 1000000]`.
	fn addition(_i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 220_000 picoseconds.
		Weight::from_parts(281_565, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn subtraction(_i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 223_000 picoseconds.
		Weight::from_parts(264_970, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn multiplication(_i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 223_000 picoseconds.
		Weight::from_parts(274_583, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn division(_i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 221_000 picoseconds.
		Weight::from_parts(236_997, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn hashing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 30_155_375_000 picoseconds.
		Weight::from_parts(32_739_997_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// The range of component `i` is `[0, 100]`.
	fn sr25519_verification(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 362_000 picoseconds.
		Weight::from_parts(409_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 94_193
			.saturating_add(Weight::from_parts(66_436_702, 0).saturating_mul(i.into()))
	}
}
