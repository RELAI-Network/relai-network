
//! Autogenerated weights for `futur_assets_reg`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-17, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// futur_assets_reg
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// pallets/futur-assets-reg/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions needed for pallet_template.
pub trait WeightInfo {
	fn submit_asset() -> Weight;
	fn publish_asset() -> Weight;
	fn un_publish_asset() -> Weight;
	fn delete_asset() -> Weight;
	fn buy_asset() -> Weight;
}

/// Weight functions for `futur_assets_reg`.
pub struct SubstrateWeight<T> { field1: PhantomData<T> }
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `FuturAssetsReg::NextAssetId` (r:1 w:1)
	/// Proof: `FuturAssetsReg::NextAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `FuturAssetsReg::AssetRegistry` (r:0 w:1)
	/// Proof: `FuturAssetsReg::AssetRegistry` (`max_values`: None, `max_size`: Some(204), added: 2679, mode: `MaxEncodedLen`)
	/// Storage: `FuturAssetsReg::AssetPublicationStatus` (r:0 w:1)
	/// Proof: `FuturAssetsReg::AssetPublicationStatus` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	fn submit_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `1489`
		// Minimum execution time: 43_000_000 picoseconds.
		Weight::from_parts(45_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `FuturAssetsReg::AssetRegistry` (r:1 w:0)
	/// Proof: `FuturAssetsReg::AssetRegistry` (`max_values`: None, `max_size`: Some(204), added: 2679, mode: `MaxEncodedLen`)
	/// Storage: `FuturAssetsReg::AssetPublicationStatus` (r:0 w:1)
	/// Proof: `FuturAssetsReg::AssetPublicationStatus` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	fn publish_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3669`
		// Minimum execution time: 42_000_000 picoseconds.
		Weight::from_parts(42_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3669))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FuturAssetsReg::AssetRegistry` (r:1 w:0)
	/// Proof: `FuturAssetsReg::AssetRegistry` (`max_values`: None, `max_size`: Some(204), added: 2679, mode: `MaxEncodedLen`)
	/// Storage: `FuturAssetsReg::AssetPublicationStatus` (r:0 w:1)
	/// Proof: `FuturAssetsReg::AssetPublicationStatus` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	fn un_publish_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3669`
		// Minimum execution time: 42_000_000 picoseconds.
		Weight::from_parts(42_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3669))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FuturAssetsReg::AssetRegistry` (r:1 w:1)
	/// Proof: `FuturAssetsReg::AssetRegistry` (`max_values`: None, `max_size`: Some(204), added: 2679, mode: `MaxEncodedLen`)
	/// Storage: `FuturAssetsReg::AssetPublicationStatus` (r:1 w:1)
	/// Proof: `FuturAssetsReg::AssetPublicationStatus` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	fn delete_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `150`
		//  Estimated: `3669`
		// Minimum execution time: 56_000_000 picoseconds.
		Weight::from_parts(58_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3669))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FuturAssetsReg::AssetRegistry` (r:1 w:0)
	/// Proof: `FuturAssetsReg::AssetRegistry` (`max_values`: None, `max_size`: Some(204), added: 2679, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `FuturAssetsReg::AssetPurchases` (r:0 w:1)
	/// Proof: `FuturAssetsReg::AssetPurchases` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn buy_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `299`
		//  Estimated: `6196`
		// Minimum execution time: 179_000_000 picoseconds.
		Weight::from_parts(183_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
