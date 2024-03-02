use crate::common::CommonMeta;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;


pub type AssetId = u32;

#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub enum AssetType {
	App,
	Game,
	Book
}

#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct Asset<AccountId, Balance> {
	pub creator: AccountId,
	pub asset_type: AssetType,
	pub name: CommonMeta,
	pub price: Option<Balance>,
	pub meta: CommonMeta
}
