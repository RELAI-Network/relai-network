#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub use relai_primitives::creatorsreg;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{Currency}
		};

	use frame_system::pallet_prelude::*;


	use relai_primitives::assetsreg::AssetWrapper;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: Currency<Self::AccountId, Balance = u128>;
	}

	#[pallet::storage]
	#[pallet::getter(fn next_asset_id)]
	pub(super) type NextAssetId<T: Config> = StorageValue<_, u32, ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {


		#[pallet::call_index(0)]
		#[pallet::weight({10000})]
		pub fn submit_asset(
			origin: OriginFor<T>,
			asset: AssetWrapper
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			// Generate a new unique ID for the developer
			let id = Self::next_asset_id() + 1;

			// Increment the counter
			NextAssetId::<T>::put(id);

			Ok(().into())
		}

	}

}
