#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::Pallet as FuturCreatorsReg;

use frame_benchmarking::v1::{
	account as benchmark_account, benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller,
};

use frame_system::RawOrigin;

use sp_core::{bounded::BoundedVec, ConstU32};

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}

benchmarks! {
  set_registration_fee {

  }: _(RawOrigin::Root, 1000000)
  verify {
	  assert_eq!(Pallet::<T>::reg_fee(), 1000000)
  }

  register_developer {

	let caller: T::AccountId = whitelisted_caller();
	let tester_data: BoundedVec::<u8, ConstU32<100>> = vec![0u8; 100].try_into().unwrap();

  }: _(RawOrigin::Signed(caller.clone()), tester_data.clone(), tester_data.clone(), tester_data.clone())
  verify {
	  assert!(DevRegistry::<T>::contains_key(&caller));
  }

  unregister_developer {

	let caller: T::AccountId = get_account::<T>("ALICE");// whitelisted_caller();

	let tester_data: BoundedVec::<u8, ConstU32<100>> = vec![0u8; 100].try_into().unwrap();

	let _ = Pallet::<T>::register_developer(RawOrigin::Signed(caller.clone()).into(), tester_data.clone(), tester_data.clone(), tester_data.clone());

  }: _(RawOrigin::Signed(caller.clone()))
  verify {
	  assert!(!DevRegistry::<T>::contains_key(caller));
  }
}

impl_benchmark_test_suite!(FuturCreatorsReg, crate::mock::new_test_ext(), crate::mock::Test);
