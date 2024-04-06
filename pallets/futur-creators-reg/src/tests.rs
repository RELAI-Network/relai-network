#![cfg(test)]

use frame_support::assert_ok;
use relai_primitives::creatorsreg::DevInfo;
use sp_core::{bounded::BoundedVec, ConstU32};

use crate::mock::*;

#[test]
fn set_registration_fees_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(FuturCreatorsReg::set_registration_fee(RuntimeOrigin::root(), 1000));
		// Read pallet storage and assert an expected result.
		assert_eq!(FuturCreatorsReg::reg_fee(), 1000);
	});
}

#[test]
fn register_developer_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(FuturCreatorsReg::set_registration_fee(RuntimeOrigin::root(), 1000));

		let test_vec: Vec<u8> = "test".as_bytes().to_vec();
		let test = BoundedVec::<u8, ConstU32<100>>::truncate_from(test_vec);

		assert_ok!(FuturCreatorsReg::register_developer(
			RuntimeOrigin::signed(1),
			test.clone(),
			test.clone(),
			test.clone()
		));

		let dev_info =
			DevInfo { id: 1, name: test.clone(), email: test.clone(), website: test.clone() };

		assert_eq!(FuturCreatorsReg::dev_regisitry(1), Some(dev_info));
	});
}

#[test]
fn unregister_developer_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(FuturCreatorsReg::set_registration_fee(RuntimeOrigin::root(), 1000));

		let test_vec: Vec<u8> = "test".as_bytes().to_vec();
		let test = BoundedVec::<u8, ConstU32<100>>::truncate_from(test_vec);

		assert_ok!(FuturCreatorsReg::register_developer(
			RuntimeOrigin::signed(1),
			test.clone(),
			test.clone(),
			test.clone()
		));

		let dev_info =
			DevInfo { id: 1, name: test.clone(), email: test.clone(), website: test.clone() };

		assert_eq!(FuturCreatorsReg::dev_regisitry(1), Some(dev_info));

		assert_ok!(FuturCreatorsReg::unregister_developer(RuntimeOrigin::signed(1)));

		assert_eq!(FuturCreatorsReg::dev_regisitry(1), None);
	});
}
