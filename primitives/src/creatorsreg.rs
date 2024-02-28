use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;

use crate::common::CommonMeta;

//TODO: Later set email and website limits at Pallet config level
#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct DevInfo {
	pub id: u32,
	pub name: CommonMeta,
	pub email: CommonMeta,
	pub website: CommonMeta,
	//pub wallet_address: AccountId,
}
