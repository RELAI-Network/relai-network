use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{bounded::BoundedVec, RuntimeDebug, ConstU32};

//TODO: Later set email and website limits at Pallet config level
#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct DevInfo {
	pub id: u32,
	pub name: BoundedVec<u8, ConstU32<100>>,
	pub email: BoundedVec<u8, ConstU32<100>>,
	pub website: BoundedVec<u8, ConstU32<100>>,
	//pub wallet_address: AccountId,
}
