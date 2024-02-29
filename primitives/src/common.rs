use sp_core::{bounded::BoundedVec, ConstU32};

pub const MAX_META_LENGTH: u32 = 100;
pub const MAX_DESC_META_LENGTH: u32 = 4000;

pub type CommonMeta = BoundedVec<u8, ConstU32<MAX_META_LENGTH>>;
pub type CommonDesc = BoundedVec<u8, ConstU32<MAX_DESC_META_LENGTH>>;
