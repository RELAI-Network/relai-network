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
		price: 1000,
		hash: [0u8; 32],
		published: true,
	};

  }: _(RawOrigin::Signed(caller.clone()), asset.clone())
  verify {
	  assert!(AssetRegistry::<T>::contains_key(&1));
	  let stored_asset = AssetRegistry::<T>::get(1).unwrap();
	  assert_eq!(stored_asset,asset);
  }

  pub_unpub_asset {
	let caller: T::AccountId = whitelisted_caller();

	let asset = Asset {
		creator: caller.clone(),
		asset_type: AssetType::App,
		name: CommonMeta::default(),
		price: 1000,
		hash: [0u8; 32],
		published: true,
	};

	let _ = Pallet::<T>::submit_asset(RawOrigin::Signed(caller.clone()).into(), asset.clone());

  }: _(RawOrigin::Signed(caller), 1, false)
  verify {
	let stored_asset = AssetRegistry::<T>:: get(&1).unwrap();
	 assert_eq!(stored_asset.published, false);
  }

  delete_asset {
	let caller: T::AccountId = whitelisted_caller();

	let asset = Asset {
		creator: caller.clone(),
		asset_type: AssetType::App,
		name: CommonMeta::default(),
		price: 1000,
		hash: [0u8; 32],
		published: false,
	};

	let _ = Pallet::<T>::submit_asset(RawOrigin::Signed(caller.clone()).into(), asset.clone());

  }: _(RawOrigin::Signed(caller),1)
  verify {
	 assert!(!AssetRegistry::<T>::contains_key(&1));
  }

  buy_asset {

	let creator: T::AccountId = get_account::<T>("CREATOR");

	let buyer: T::AccountId = get_account::<T>("BUYER");

	let asset = Asset {
		creator: creator.clone(),
		asset_type: AssetType::App,
		name: CommonMeta::default(),
		price: 1000,
		hash: [0u8; 32],
		published: true,
	};

	let _ = Pallet::<T>::submit_asset(RawOrigin::Signed(creator.clone()).into(), asset.clone());

	T::Currency::make_free_balance_be(&buyer, 3000);


  }: _(RawOrigin::Signed(buyer.clone()),1)
  verify {
	 assert!(AssetPurchases::<T>::contains_key(&buyer, &1));
  }

  update_asset {
	let caller: T::AccountId = whitelisted_caller();

	let asset = Asset {
		creator: caller.clone(),
		asset_type: AssetType::App,
		name: CommonMeta::default(),
		price: 1000,
		hash: [0u8; 32],
		published: true,
	};

	let _ = Pallet::<T>::submit_asset(RawOrigin::Signed(caller.clone()).into(), asset.clone());

	let asset = Asset {
		creator: caller.clone(),
		asset_type: AssetType::App,
		name: CommonMeta::default(),
		price: 3000,
		hash: [0u8; 32],
		published: false,
	};

  }: _(RawOrigin::Signed(caller), 1, asset)
  verify {
	let stored_asset = AssetRegistry::<T>:: get(&1).unwrap();
	 assert_eq!(stored_asset.published, false);
	 assert_eq!(stored_asset.price, 3000);

  }

}

impl_benchmark_test_suite!(FuturAssetsReg, crate::mock::new_test_ext(), crate::mock::Test);
