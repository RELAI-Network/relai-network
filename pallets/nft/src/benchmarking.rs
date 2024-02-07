//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as NFT;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_class() {
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		create_class(RawOrigin::Signed(caller));

	}

	impl_benchmark_test_suite!(NFT, crate::mock::new_test_ext(), crate::mock::Test);
}
