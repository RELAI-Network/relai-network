use crate::common::CommonMeta;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize,  Serialize};
use sp_core::RuntimeDebug;
use sp_std::prelude::Vec as SpVec;

use scale_info::prelude::string::String;
pub type AssetId = u32;

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

//pub type Reviews = SpVec<SpVec<u8>>;
pub type Reviews = SpVec<String>;


#[derive(Serialize, Deserialize, RuntimeDebug, Encode, Decode)]
pub struct ReviewsResponse {
	pub reviews: Reviews,
}
