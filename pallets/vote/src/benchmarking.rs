//! Benchmarking setup for pallet-citizenship
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn new_passport() {
		let id: u32 = 1u32.into();
		let caller: T::AccountId = whitelisted_caller();
		let mut passport = Vec::<u8>::new();
		passport.push(b'A');
		passport.push(b'B');
		#[extrinsic_call]
		new_passport(RawOrigin::Signed(caller.clone()), caller.clone(), id, passport);

		//assert_eq!(Something::<T>::get(), Some(value));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
