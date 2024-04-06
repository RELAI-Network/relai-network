#![cfg(test)]

use frame_support::assert_ok;
use relai_primitives::assetsreg::{Asset, AssetType};
use sp_core::{bounded::BoundedVec, ConstU32};

use crate::mock::*;

const ALICE: u64 = 1;
const BOB: u64 = 2;

#[test]
fn submit_asset_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let test_vec: Vec<u8> = "test".as_bytes().to_vec();
		let test = BoundedVec::<u8, ConstU32<100>>::truncate_from(test_vec);

		let asset = Asset {
			creator: ALICE,
			asset_type: AssetType::App,
			name: test.clone(),
			price: 100,
			hash: Default::default(),
			published: true,
		};

		// Dispatch a signed extrinsic.
		assert_ok!(FuturAssetsReg::submit_asset(RuntimeOrigin::signed(ALICE), asset.clone()));

		//Asset Id should be 1
		assert_eq!(FuturAssetsReg::asset_regisitry(1), Some(asset));
	});
}

#[test]
fn pub_unpub_asset_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let test_vec: Vec<u8> = "test".as_bytes().to_vec();
		let test = BoundedVec::<u8, ConstU32<100>>::truncate_from(test_vec);

		let asset = Asset {
			creator: ALICE,
			asset_type: AssetType::App,
			name: test.clone(),
			price: 100,
			hash: Default::default(),
			published: true,
		};

		// Dispatch a signed extrinsic.
		assert_ok!(FuturAssetsReg::submit_asset(RuntimeOrigin::signed(ALICE), asset.clone()));

		//Asset Id should be 1
		let stored_asset = FuturAssetsReg::asset_regisitry(1).unwrap();

		assert_eq!(stored_asset, asset);

		// Let's unpublish the asset by setting published to false

		assert_ok!(FuturAssetsReg::pub_unpub_asset(RuntimeOrigin::signed(ALICE), 1, false));

		let stored_asset = FuturAssetsReg::asset_regisitry(1).unwrap();

		assert_eq!(stored_asset.published, false);
	});
}

#[test]
fn buy_asset_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let test_vec: Vec<u8> = "test".as_bytes().to_vec();
		let test = BoundedVec::<u8, ConstU32<100>>::truncate_from(test_vec);

		let asset = Asset {
			creator: ALICE,
			asset_type: AssetType::App,
			name: test.clone(),
			price: 100,
			hash: Default::default(),
			published: true,
		};

		// Dispatch a signed extrinsic.
		assert_ok!(FuturAssetsReg::submit_asset(RuntimeOrigin::signed(ALICE), asset.clone()));

		//Asset Id should be 1
		assert_ok!(FuturAssetsReg::buy_asset(RuntimeOrigin::signed(BOB), 1));

		assert_eq!(FuturAssetsReg::asset_purchases(BOB, 1), Some(true));
	});
}

#[test]
fn update_asset_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let test_vec: Vec<u8> = "test".as_bytes().to_vec();
		let test = BoundedVec::<u8, ConstU32<100>>::truncate_from(test_vec);

		let asset = Asset {
			creator: ALICE,
			asset_type: AssetType::App,
			name: test.clone(),
			price: 100,
			hash: Default::default(),
			published: true,
		};

		// Dispatch a signed extrinsic.
		assert_ok!(FuturAssetsReg::submit_asset(RuntimeOrigin::signed(ALICE), asset.clone()));

		let updated_asset = Asset {
			creator: ALICE,
			asset_type: AssetType::Game,
			name: test.clone(),
			price: 300,
			hash: Default::default(),
			published: true,
		};

		//Asset Id should be 1
		assert_ok!(FuturAssetsReg::update_asset(RuntimeOrigin::signed(ALICE), 1, updated_asset));

		let stored_asset = FuturAssetsReg::asset_regisitry(1).unwrap();

		assert_eq!(stored_asset.asset_type, AssetType::Game);

		assert_eq!(stored_asset.price, 300);
	});
}
