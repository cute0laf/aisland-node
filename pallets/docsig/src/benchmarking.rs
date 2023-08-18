

//! Benchmarking setup for pallet_docsig
#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
#[allow(unused)]
use crate::Pallet as Docsig;

benchmarks! {
  // benchmarks here
	new_document {
			//set the initial state 
			let id:u32 = 1u32.into();
			let caller: T::AccountId = whitelisted_caller();
			let mut document = Vec::<u8>::new();
			// worst scenario = 128 bytes hash length
			for _n in 1..128 {
				document.push(b'x');
			}
	}:_(RawOrigin::Signed(caller.clone()), id.clone(),document.clone())
	verify {
		// verification of the content
		assert_eq!(Documents::<T>::get(caller,1u32),document);
	}
	// destroy document benchmark
	destroy_document {
		//set the initial state creating a document
		let id:u32 = 1u32.into();
		let caller: T::AccountId = whitelisted_caller();
		let mut document = Vec::<u8>::new();
		for _n in 1..64 {
		document.push(b'x');
		}
		let _ = Pallet::<T>::new_document(RawOrigin::Signed(caller.clone()).into(), id.clone(),document.clone());
		//execute the extrinsic
	}:_(RawOrigin::Signed(caller.clone()), id.clone())
	verify {
		// verification that the document has been removed
		assert_eq!(Documents::<T>::get(caller,1u32).len(),0);
	}
	// sign document benchmark
	sign_document {
		//set the initial state creating a document
		let id:u32 = 1u32.into();
		let caller: T::AccountId = whitelisted_caller();
		let mut hash = Vec::<u8>::new();
		for _n in 1..64 {
			hash.push(b'x');
		}
		//let _ = Pallet::<T>::new_document(RawOrigin::Signed(caller.clone()).into(), id.clone(),document.clone());
		//execute the extrinsic
	}:_(RawOrigin::Signed(caller.clone()), id.clone(),hash.clone())
	verify {
		// verification that the signature has been stored
		assert_eq!(Signatures::<T>::get(caller,1u32),hash);
	}
	// store public key benchmark
	store_publickey {
		//set the iitial state storing a public key
		let caller: T::AccountId = whitelisted_caller();
		let mut publickey = Vec::<u8>::new();
		for _n in 1..64 {
			publickey.push(b'0');
		}
	}:_(RawOrigin::Signed(caller.clone()), publickey.clone())
	verify {
		// verification that the signature has been stored
		assert_eq!(EncryptionPublicKeys::<T>::get(caller),publickey);
	}
	// new blob benchmark
	new_blob {
		//set the initial state creating a document
		let id:u32 = 1u32.into();
		let chunkid:u32 = 0u32.into();
		let caller: T::AccountId = whitelisted_caller();
		let mut blob = Vec::<u8>::new();
		//worst scenarion 100,000 bytes blob
		for _n in 1..100000 {
			blob.push(b'x');
		}
		//execute the extrinsic
	}:_(RawOrigin::Signed(caller.clone()),id.clone(),chunkid.clone(),blob.clone())
	verify {
		// verification that the blob has been stored
		let keyarg=(caller,1u32,0u32);
		assert_eq!(Blobs::<T>::get(keyarg),blob);
	}
	
	// new blob benchmark
	destroy_blob {
		//set the initial state creating a document
		let id:u32 = 1u32.into();
		let chunkid:u32 = 0u32.into();
		let caller: T::AccountId = whitelisted_caller();
		let mut blob = Vec::<u8>::new();
		for _n in 1..1000 {
			blob.push(b'x');
		}
		let _ = Pallet::<T>::new_blob(RawOrigin::Signed(caller.clone()).into(), id.clone(),chunkid.clone(),blob.clone());
		//execute the extrinsic
	}:_(RawOrigin::Signed(caller.clone()),id.clone(),chunkid.clone())
	verify {
		// verification that the blob has been destroyed
		let keyarg=(caller,1u32,0u32);
		assert_eq!(Blobs::<T>::get(keyarg).len(),0);
	}
	
  	impl_benchmark_test_suite!(Docsig, crate::mock::new_test_ext(), crate::mock::Test);
}
// notes for testing and executing benchmarks
// build node: cargo build --release --features runtime-benchmarks
// run test: cargo test -p pallet-docsig --features runtime-benchmarks
// list: ../target/release/aisland-node benchmark pallet --chain dev --pallet "*" --extrinsic "*" --repeat 0
// execute benchmarks and save weigthinfo
// ../target/release/aisland-node benchmark pallet --chain dev --wasm-execution=compiled --pallet "pallet_docsig" --extrinsic "*" --repeat 20  --steps 50 --output docsig/src/weights.rs


