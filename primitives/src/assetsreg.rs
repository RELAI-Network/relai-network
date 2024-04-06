use crate::common::CommonMeta;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use sp_std::prelude::Vec as SVec;

pub type AssetId = u32;
pub type Reviews = SVec<SVec<u8>>;

#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub enum AssetType {
	App,
	Game,
	Book,
}

#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct Asset<AccountId, Balance> {
	pub creator: AccountId,
	pub asset_type: AssetType,
	pub name: CommonMeta,
	pub price: Balance,
	pub hash: [u8; 32],
	pub published: bool,
}

#[derive(Serialize, Deserialize, RuntimeDebug, Encode, Decode)]
pub struct ReviewsResponse {
	pub reviews: Reviews,
}
