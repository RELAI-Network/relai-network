
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;



#[derive(Encode, Decode, Default, RuntimeDebug, Clone, Eq, PartialEq, TypeInfo)]
pub struct DevInfo<AccountId> {
    pub id: u32,
    pub email: Vec<u8>,
    pub website: Vec<u8>,
    pub wallet_address: AccountId
}