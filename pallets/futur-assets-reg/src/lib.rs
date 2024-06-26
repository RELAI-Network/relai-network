#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

mod mock;

mod tests;

pub use pallet::*;

pub use relai_primitives::{
	common::{CommonDesc, CommonMeta},
	creatorsreg,
};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		ensure,
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement},
	};

	use sp_runtime::offchain::{http, Duration};

	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	use relai_primitives::assetsreg::{Asset, AssetId, ReviewsResponse};

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
	#[pallet::getter(fn asset_regisitry)]
	pub(super) type AssetRegistry<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetId, Asset<T::AccountId, BalanceOf<T>>, OptionQuery>;

	#[pallet::storage]
	pub(super) type AssetByCreator<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetId, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn asset_purchases)]
	pub(super) type AssetPurchases<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		AssetId,
		bool,
		OptionQuery,
	>;

	#[pallet::storage]
	pub(super) type Reviews<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CommonMeta,
		Blake2_128Concat,
		AssetId,
		CommonDesc,
		OptionQuery,
	>;

	#[pallet::storage]
	pub(super) type Revs<T: Config> =
		StorageMap<_, Blake2_128Concat, [u8; 48], bool, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AssetSubmited { creator: T::AccountId, id: AssetId },
		AssetPublished { creator: T::AccountId, id: AssetId },
		AssetUnPublished { creator: T::AccountId, id: AssetId },
		AssetDeleted { id: AssetId },
		AssetBought { buyer: T::AccountId, id: AssetId },
		AssetUpdated { id: AssetId },
		ReviewSubmitted { reviewer: T::AccountId, id: AssetId },
	}

	#[pallet::error]
	pub enum Error<T> {
		AssetNotFound,
		AssetIsPublished,
		AssetNotOwnedByCaller,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Extrinsic for submitting an asset
		/// takes asset object as argument
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::submit_asset())]
		pub fn submit_asset(
			origin: OriginFor<T>,
			asset: Asset<T::AccountId, BalanceOf<T>>,
		) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;

			Self::do_submit_asset(&creator, &asset)?;

			Ok(().into())
		}

		/// Extrinsic for publishing or unpublishing an asset in the store
		/// the asset will be available in FuturStore mobile app
		/// if pub_unpub argment is set to true
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::pub_unpub_asset())]
		pub fn pub_unpub_asset(
			origin: OriginFor<T>,
			asset_id: AssetId,
			pub_unpub: bool,
		) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;

			AssetRegistry::<T>::try_mutate(asset_id, |maybe_asset| -> DispatchResult {
				let asset = maybe_asset.as_mut().ok_or(Error::<T>::AssetNotFound)?;

				ensure!(asset.creator == creator, Error::<T>::AssetNotOwnedByCaller);

				asset.published = pub_unpub;

				Ok(())
			})?;

			match pub_unpub {
				true => Self::deposit_event(Event::AssetPublished { creator, id: asset_id }),
				false => Self::deposit_event(Event::AssetUnPublished { creator, id: asset_id }),
			};

			Ok(().into())
		}

		/// Extrinsic for Deleting an asset
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::delete_asset())]
		pub fn delete_asset(origin: OriginFor<T>, asset_id: AssetId) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;

			let maybe_asset = match AssetRegistry::<T>::try_get(asset_id) {
				Ok(asset) => asset,
				Err(_) => return Err(Error::<T>::AssetNotFound.into()),
			};

			ensure!(maybe_asset.creator == creator, Error::<T>::AssetNotOwnedByCaller);
			ensure!(maybe_asset.published == false, Error::<T>::AssetIsPublished);

			AssetRegistry::<T>::remove(asset_id);

			Self::deposit_event(Event::AssetDeleted { id: asset_id });

			Ok(().into())
		}

		/// Extrinsic for buying an asset
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::buy_asset())]
		pub fn buy_asset(origin: OriginFor<T>, asset_id: AssetId) -> DispatchResultWithPostInfo {
			let buyer = ensure_signed(origin)?;

			Self::do_buy_asset(&buyer, asset_id)?;

			Self::deposit_event(Event::AssetBought { buyer, id: asset_id });

			Ok(().into())
		}

		/// Extrinsic for updating an asset
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::update_asset())]
		pub fn update_asset(
			origin: OriginFor<T>,
			asset_id: AssetId,
			asset: Asset<T::AccountId, BalanceOf<T>>,
		) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;

			let supposed_creator =
				AssetByCreator::<T>::get(asset_id).ok_or(Error::<T>::AssetNotFound)?;

			ensure!(supposed_creator == creator, Error::<T>::AssetNotOwnedByCaller);

			Self::do_update_asset(asset_id, asset)?;

			Ok(().into())
		}

		//TODO: Add later Asset Ownership Transfer feature
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: BlockNumberFor<T>) {}
	}

	impl<T: Config> Pallet<T> {
		fn do_submit_asset(
			creator: &T::AccountId,
			asset: &Asset<T::AccountId, BalanceOf<T>>,
		) -> DispatchResult {
			let asset_id = Self::handle_id();

			AssetRegistry::<T>::insert(asset_id, &asset);

			AssetByCreator::<T>::insert(asset_id, creator.clone());

			Self::deposit_event(Event::AssetSubmited { creator: creator.clone(), id: asset_id });

			Ok(())
		}

		fn do_update_asset(
			asset_id: AssetId,
			asset: Asset<T::AccountId, BalanceOf<T>>,
		) -> DispatchResult {
			AssetRegistry::<T>::mutate(asset_id, |a| {
				let a = a.as_mut().unwrap();
				*a = asset
			});

			Self::deposit_event(Event::AssetUpdated { id: asset_id });

			Ok(())
		}

		fn do_buy_asset(buyer: &T::AccountId, asset_id: AssetId) -> DispatchResult {
			if let Some(asset) = AssetRegistry::<T>::get(asset_id) {
				if asset.price > 0 {
					let _ = T::Currency::transfer(
						buyer,
						&asset.creator.clone(),
						asset.price,
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
