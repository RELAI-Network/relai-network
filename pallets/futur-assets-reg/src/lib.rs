#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub use relai_primitives::creatorsreg;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::Currency,
	};

	use frame_system::pallet_prelude::*;

	use relai_primitives::assetsreg::{AssetId, AssetWrapper};

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

	#[pallet::storage]
	#[pallet::getter(fn asset_regisitry)]
	pub(super) type AssetRegistry<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		AssetId,
		AssetWrapper,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn is_asset_published)]
	pub(super) type AssetPublicationStatus<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetId, bool, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AssetSubmited { creator: T::AccountId, id: AssetId },
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight({10000})]
		pub fn submit_asset(
			origin: OriginFor<T>,
			asset: AssetWrapper,
			to_publish: bool,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Self::do_submit_asset(&who, &asset, to_publish)?;

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn do_submit_asset(
			creator: &T::AccountId,
			asset: &AssetWrapper,
			to_publish: bool,
		) -> DispatchResult {
			let asset_id = Self::handle_id();

			AssetRegistry::<T>::insert(&creator, asset_id, &asset);

			AssetPublicationStatus::<T>::insert(asset_id, to_publish);

			Self::deposit_event(Event::AssetSubmited { creator: creator.clone(), id: asset_id });

			Ok(())
		}

		fn handle_id() -> u32 {
			let id = Self::next_asset_id() + 1;
			NextAssetId::<T>::put(id);
			id
		}
	}
}
