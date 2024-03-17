#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

pub use pallet::*;

pub use relai_primitives::creatorsreg;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		ensure,
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement},
	};

	use frame_system::pallet_prelude::*;

	use relai_primitives::assetsreg::{Asset, AssetId};

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type WeightInfo: WeightInfo;

		type Currency: Currency<Self::AccountId, Balance = u128>;
	}

	#[pallet::storage]
	#[pallet::getter(fn next_asset_id)]
	pub(super) type NextAssetId<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub(super) type AssetRegistry<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetId, Asset<T::AccountId, BalanceOf<T>>, OptionQuery>;

	#[pallet::storage]
	pub(super) type AssetByCreator<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetId, T::AccountId, OptionQuery>;

	#[pallet::storage]
	pub(super) type AssetPublicationStatus<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetId, bool, OptionQuery>;

	#[pallet::storage]
	pub(super) type AssetPurchases<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		AssetId,
		bool,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AssetSubmited { creator: T::AccountId, id: AssetId },
		AssetPublished { creator: T::AccountId, id: AssetId },
		AssetUnPublished { creator: T::AccountId, id: AssetId },
		AssetDeleted { creator: T::AccountId, id: AssetId },
		AssetBought { buyer: T::AccountId, id: AssetId },
	}

	#[pallet::error]
	pub enum Error<T> {
		AssetNotFound,
		AssetNotUnpublished,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::submit_asset())]
		pub fn submit_asset(
			origin: OriginFor<T>,
			asset: Asset<T::AccountId, BalanceOf<T>>,
			to_publish: bool,
		) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;

			Self::do_submit_asset(&creator, &asset, to_publish)?;

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::publish_asset())]
		pub fn publish_asset(
			origin: OriginFor<T>,
			asset_id: AssetId,
		) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;

			ensure!(AssetRegistry::<T>::contains_key(&asset_id), Error::<T>::AssetNotFound);

			AssetPublicationStatus::<T>::insert(asset_id, true);

			Self::deposit_event(Event::AssetPublished { creator, id: asset_id });

			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::un_publish_asset())]
		pub fn un_publish_asset(
			origin: OriginFor<T>,
			asset_id: AssetId,
		) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;

			ensure!(AssetRegistry::<T>::contains_key(asset_id), Error::<T>::AssetNotFound);

			AssetPublicationStatus::<T>::insert(asset_id, false);

			Self::deposit_event(Event::AssetUnPublished { creator, id: asset_id });

			Ok(().into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::delete_asset())]
		pub fn delete_asset(origin: OriginFor<T>, asset_id: AssetId) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;

			ensure!(
				AssetRegistry::<T>::contains_key(asset_id) &&
					AssetPublicationStatus::<T>::contains_key(asset_id),
				Error::<T>::AssetNotFound
			);

			ensure!(
				AssetPublicationStatus::<T>::get(asset_id)
					.map(|status| status.eq(&false))
					.unwrap_or(true),
				Error::<T>::AssetNotUnpublished
			);

			AssetRegistry::<T>::remove(asset_id);

			AssetPublicationStatus::<T>::remove(asset_id);

			Self::deposit_event(Event::AssetDeleted { creator, id: asset_id });

			Ok(().into())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::buy_asset())]
		pub fn buy_asset(origin: OriginFor<T>, asset_id: AssetId) -> DispatchResultWithPostInfo {
			let buyer = ensure_signed(origin)?;

			Self::do_buy_asset(&buyer, asset_id)?;

			Self::deposit_event(Event::AssetBought { buyer, id: asset_id });

			Ok(().into())
		}

		//TODO: Add later Asset Ownership Transfer feature
	}

	impl<T: Config> Pallet<T> {
		fn do_submit_asset(
			creator: &T::AccountId,
			asset: &Asset<T::AccountId, BalanceOf<T>>,
			to_publish: bool,
		) -> DispatchResult {
			let asset_id = Self::handle_id();

			AssetRegistry::<T>::insert(asset_id, &asset);

			AssetPublicationStatus::<T>::insert(asset_id, to_publish);

			Self::deposit_event(Event::AssetSubmited { creator: creator.clone(), id: asset_id });

			Ok(())
		}

		fn do_buy_asset(buyer: &T::AccountId, asset_id: AssetId) -> DispatchResult {
			if let Some(asset) = AssetRegistry::<T>::get(asset_id) {
				if asset.price.is_some() {
					let _ = T::Currency::transfer(
						buyer,
						&asset.creator.clone(),
						asset.price.unwrap(),
						ExistenceRequirement::KeepAlive,
					);
				}
			} else {
				Err(Error::<T>::AssetNotFound)?;
			}

			AssetPurchases::<T>::insert(buyer, asset_id, true);

			Ok(())
		}

		fn handle_id() -> u32 {
			let id = Self::next_asset_id() + 1;
			NextAssetId::<T>::put(id);
			id
		}
	}
}
