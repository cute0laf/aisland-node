
//! Autogenerated weights for `pallet_docsig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `unknown.local`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ../target/release/aisland-node
// benchmark
// pallet
// --chain
// dev
// --wasm-execution=compiled
// --pallet
// pallet_docsig
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// docsig/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

pub trait WeightInfo {
	fn new_document() -> Weight;
	fn destroy_document() -> Weight;
	fn sign_document() -> Weight;
	fn store_publickey() -> Weight;
	fn new_blob() -> Weight;
	fn destroy_blob() -> Weight;
}
/// Weight functions for `pallet_docsig`.
//pub struct WeightInfo<T>(PhantomData<T>);
//impl<T: frame_system::Config> pallet_docsig::WeightInfo for WeightInfo<T> {
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {

	/// Storage: DocSig Documents (r:1 w:1)
	/// Proof Skipped: DocSig Documents (max_values: None, max_size: None, mode: Measured)
	fn new_document() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3471`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3471))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: DocSig Documents (r:1 w:1)
	/// Proof Skipped: DocSig Documents (max_values: None, max_size: None, mode: Measured)
	fn destroy_document() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `166`
		//  Estimated: `3631`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3631))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: DocSig Signatures (r:1 w:1)
	/// Proof Skipped: DocSig Signatures (max_values: None, max_size: None, mode: Measured)
	fn sign_document() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3471`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3471))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: DocSig Signatures (r:1 w:1)
	/// Proof Skipped: DocSig Signatures (max_values: None, max_size: None, mode: Measured)
	fn store_publickey() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3471`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3471))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: DocSig Blobs (r:1 w:1)
	/// Proof Skipped: DocSig Blobs (max_values: None, max_size: None, mode: Measured)
	fn new_blob() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3471`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3471))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: DocSig Signatures (r:1 w:0)
	/// Proof Skipped: DocSig Signatures (max_values: None, max_size: None, mode: Measured)
	/// Storage: DocSig Blobs (r:1 w:1)
	/// Proof Skipped: DocSig Blobs (max_values: None, max_size: None, mode: Measured)
	fn destroy_blob() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10123`
		//  Estimated: `13588`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(Weight::from_parts(0, 13588))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: DocSig Documents (r:1 w:1)
	/// Proof Skipped: DocSig Documents (max_values: None, max_size: None, mode: Measured)
	fn new_document() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3471`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3471))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: DocSig Documents (r:1 w:1)
	/// Proof Skipped: DocSig Documents (max_values: None, max_size: None, mode: Measured)
	fn destroy_document() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `166`
		//  Estimated: `3631`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3631))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: DocSig Signatures (r:1 w:1)
	/// Proof Skipped: DocSig Signatures (max_values: None, max_size: None, mode: Measured)
	fn sign_document() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3471`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3471))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: DocSig Signatures (r:1 w:1)
	/// Proof Skipped: DocSig Signatures (max_values: None, max_size: None, mode: Measured)
	fn store_publickey() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3471`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3471))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: DocSig Blobs (r:1 w:1)
	/// Proof Skipped: DocSig Blobs (max_values: None, max_size: None, mode: Measured)
	fn new_blob() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3471`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3471))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: DocSig Signatures (r:1 w:0)
	/// Proof Skipped: DocSig Signatures (max_values: None, max_size: None, mode: Measured)
	/// Storage: DocSig Blobs (r:1 w:1)
	/// Proof Skipped: DocSig Blobs (max_values: None, max_size: None, mode: Measured)
	fn destroy_blob() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10123`
		//  Estimated: `13588`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(Weight::from_parts(0, 13588))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}