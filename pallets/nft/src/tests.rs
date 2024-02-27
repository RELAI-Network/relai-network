use crate::{mock::*, Event};
use frame_support::assert_ok;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(NFT::create_class(RuntimeOrigin::signed(1)));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::NftClassCreated { owner: 1 }.into());
	});
}
