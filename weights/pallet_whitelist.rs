
//! Autogenerated weights for `pallet_whitelist`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_whitelist
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

/// Weight functions for `pallet_whitelist`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_whitelist::WeightInfo for WeightInfo<T> {
	/// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
	/// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:1)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn whitelist_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `84`
		//  Estimated: `3556`
		// Minimum execution time: 23_229_000 picoseconds.
		Weight::from_parts(24_083_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
	/// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:1)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn remove_whitelisted_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `213`
		//  Estimated: `3556`
		// Minimum execution time: 20_356_000 picoseconds.
		Weight::from_parts(21_527_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
	/// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:1 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `Measured`)
	/// Storage: `Preimage::StatusFor` (r:1 w:1)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 4194294]`.
	fn dispatch_whitelisted_call(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `289 + n * (1 ±0)`
		//  Estimated: `3753 + n * (1 ±0)`
		// Minimum execution time: 34_977_000 picoseconds.
		Weight::from_parts(35_485_000, 0)
			.saturating_add(Weight::from_parts(0, 3753))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1_550, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
	}
	/// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
	/// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:1)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 10000]`.
	fn dispatch_whitelisted_call_with_preimage(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `213`
		//  Estimated: `3556`
		// Minimum execution time: 24_997_000 picoseconds.
		Weight::from_parts(25_632_688, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			// Standard Error: 15
			.saturating_add(Weight::from_parts(1_870, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
