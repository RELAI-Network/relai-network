
//! Autogenerated weights for `futur_creators_reg`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-14, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `morgueye`, CPU: `<UNKNOWN>`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/relai-network
// benchmark
// pallet
// --chain
// dev
// --pallet
// futur_creators_reg
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// pallets/futur-creators-reg/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

pub trait WeightInfo {
	fn set_registration_fee() -> Weight;
	fn register_developer() -> Weight;
	fn unregister_developer() -> Weight;
}

/// Weight functions for `futur_creators_reg`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `FuturCreatorsReg::RegFee` (r:0 w:1)
	/// Proof: `FuturCreatorsReg::RegFee` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn set_registration_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 27_000_000 picoseconds.
		Weight::from_parts(27_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FuturCreatorsReg::DevRegistry` (r:1 w:1)
	/// Proof: `FuturCreatorsReg::DevRegistry` (`max_values`: None, `max_size`: Some(358), added: 2833, mode: `MaxEncodedLen`)
	/// Storage: `FuturCreatorsReg::NextDevId` (r:1 w:1)
	/// Proof: `FuturCreatorsReg::NextDevId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn register_developer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `43`
		//  Estimated: `3823`
		// Minimum execution time: 46_000_000 picoseconds.
		Weight::from_parts(47_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3823))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FuturCreatorsReg::DevRegistry` (r:1 w:1)
	/// Proof: `FuturCreatorsReg::DevRegistry` (`max_values`: None, `max_size`: Some(358), added: 2833, mode: `MaxEncodedLen`)
	fn unregister_developer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `167`
		//  Estimated: `3823`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(42_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3823))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}