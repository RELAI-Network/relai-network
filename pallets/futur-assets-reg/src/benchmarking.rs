#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::Pallet as FuturAssetsReg;

use frame_benchmarking::v1::{
	account as benchmark_account, benchmarks, impl_benchmark_test_suite, whitelisted_caller,
};

use frame_support::traits::Currency;

use frame_system::RawOrigin;

use relai_primitives::{
	assetsreg::{Asset, AssetType},
	common::CommonMeta,
};

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}

benchmarks! {

  submit_asset {
	let caller: T::AccountId = whitelisted_caller();

	let asset = Asset {
		creator: caller.clone(),
		asset_type: AssetType::App,
		name: CommonMeta::default(),
		price: Some(1000),
		meta: [0u8; 32],
	};

	let to_publish = true;

  }: _(RawOrigin::Signed(caller.clone()), asset.clone(), to_publish)
  verify {
	  assert!(AssetRegistry::<T>::contains_key(&1));
  }

  publish_asset {
	let caller: T::AccountId = whitelisted_caller();

	let asset = Asset {
		creator: caller.clone(),
		asset_type: AssetType::App,
		name: CommonMeta::default(),
		price: Some(1000),
		meta: [0u8; 32],
	};

	let _ = Pallet::<T>::submit_asset(RawOrigin::Signed(caller.clone()).into(), asset.clone(), false);

	assert_eq!(AssetPublicationStatus::<T>:: get(&1).unwrap(),false);

  }: _(RawOrigin::Signed(caller),1)
  verify {
	 assert_eq!(AssetPublicationStatus::<T>:: get(&1).unwrap(),true);
  }

  un_publish_asset {
	let caller: T::AccountId = whitelisted_caller();

	let asset = Asset {
		creator: caller.clone(),
		asset_type: AssetType::App,
		name: CommonMeta::default(),
		price: Some(1000),
		meta: [0u8; 32],
	};

	let _ = Pallet::<T>::submit_asset(RawOrigin::Signed(caller.clone()).into(), asset.clone(), true);

	assert_eq!(AssetPublicationStatus::<T>:: get(&1).unwrap(),true);

  }: _(RawOrigin::Signed(caller),1)
  verify {
	 assert_eq!(AssetPublicationStatus::<T>:: get(&1).unwrap(),false);
  }

  delete_asset {
	let caller: T::AccountId = whitelisted_caller();

	let asset = Asset {
		creator: caller.clone(),
		asset_type: AssetType::App,
		name: CommonMeta::default(),
		price: Some(1000),
		meta: [0u8; 32],
	};

	let _ = Pallet::<T>::submit_asset(RawOrigin::Signed(caller.clone()).into(), asset.clone(), false);

  }: _(RawOrigin::Signed(caller),1)
  verify {
	 assert!(!AssetRegistry::<T>::contains_key(&1));
	 assert!(!AssetPublicationStatus::<T>::contains_key(&1));
  }


  buy_asset {

	let creator: T::AccountId = get_account::<T>("CREATOR");

	let buyer: T::AccountId = get_account::<T>("BUYER");

	let asset = Asset {
		creator: creator.clone(),
		asset_type: AssetType::App,
		name: CommonMeta::default(),
		price: Some(1000),
		meta: [0u8; 32],
	};

	let _ = Pallet::<T>::submit_asset(RawOrigin::Signed(creator.clone()).into(), asset.clone(), true);

	T::Currency::make_free_balance_be(&buyer, 3000);


  }: _(RawOrigin::Signed(buyer.clone()),1)
  verify {
	 assert!(AssetPurchases::<T>::contains_key(&buyer, &1));
  }

}

impl_benchmark_test_suite!(FuturAssetsReg, crate::mock::new_test_ext(), crate::mock::Test);
