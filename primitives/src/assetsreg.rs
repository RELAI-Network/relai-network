
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;
use crate::common::{CommonMeta, CommonDesc};


// Asset trait
pub trait Asset {
    fn get_metadata(&self) -> &AssetMetadata;
}

// AssetType enum
#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub enum AssetType {
    App,
    Game,
    Book,
}

// AssetMetadata struct
#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct AssetMetadata {
    pub asset_type: AssetType,
}

// Implement Asset trait for App
#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct App {
    metadata: AppMetadata,
}

impl Asset for App {
    fn get_metadata(&self) -> &AssetMetadata {
        &self.metadata.asset_metadata
    }
}

#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct AppMetadata {
    asset_metadata: AssetMetadata,
    name: CommonMeta,
    description: CommonDesc,
}

// Implement Asset trait for Game
#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct Game {
    metadata: GameMetadata,
}

impl Asset for Game {
    fn get_metadata(&self) -> &AssetMetadata {
        &self.metadata.asset_metadata
    }
}

#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct GameMetadata {
    asset_metadata: AssetMetadata,
    name: CommonMeta,
    genre: CommonMeta,
}

// Implement Asset trait for Book
#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct Book {
    metadata: BookMetadata,
}

impl Asset for Book {
    fn get_metadata(&self) -> &AssetMetadata {
        &self.metadata.asset_metadata
    }
}

#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct BookMetadata {
    asset_metadata: AssetMetadata,
    title: CommonMeta,
    author: CommonMeta,
    // Book-specific metadata
}

#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub enum AssetWrapper {
    App(App),
    Game(Game),
    Book(Book),
}