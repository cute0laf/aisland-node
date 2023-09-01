//! Benchmarking setup for pallet-marketplace
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Marketplace;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_iso_country() {
		let caller: T::AccountId = whitelisted_caller();
		let mut countrycode = Vec::<u8>::new();
		countrycode.push(b'A');
		countrycode.push(b'I');
		let mut countryname = Vec::<u8>::new();
		countryname.push(b'A');
		countryname.push(b'i');
		countryname.push(b's');
		countryname.push(b'l');
		countryname.push(b'a');
		countryname.push(b'n');
		countryname.push(b'd');
		#[extrinsic_call]
		create_iso_country(RawOrigin::Signed(caller), countrycode.clone(), countryname.clone());
		//assert_eq!(create_iso_country::<T>::get(), Some(countryname));
	}

	impl_benchmark_test_suite!(Marketplace, crate::mock::new_test_ext(), crate::mock::Test);
}
