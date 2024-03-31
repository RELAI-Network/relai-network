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

	use sp_runtime::offchain::{http, Duration};

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
		AssetDeleted { id: AssetId },
		AssetBought { buyer: T::AccountId, id: AssetId },
		AssetUpdated { id: AssetId },
	}

	#[pallet::error]
	pub enum Error<T> {
		AssetNotFound,
		AssetIsPublished,
		AssetNotOwnedByCaller,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
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

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::un_publish_asset())]
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

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::buy_asset())]
		pub fn buy_asset(origin: OriginFor<T>, asset_id: AssetId) -> DispatchResultWithPostInfo {
			let buyer = ensure_signed(origin)?;

			Self::do_buy_asset(&buyer, asset_id)?;

			Self::deposit_event(Event::AssetBought { buyer, id: asset_id });

			Ok(().into())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::submit_asset())]
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
		fn offchain_worker(_block_number: BlockNumberFor<T>) {
			log::info!("☎️ ☎️ ☎️ Fetching Reviews");
			let _ = Self::do_fetch_reviews();
		}
	}

	impl<T: Config> Pallet<T> {
		fn do_submit_asset(
			creator: &T::AccountId,
			asset: &Asset<T::AccountId, BalanceOf<T>>,
		) -> DispatchResult {
			let asset_id = Self::handle_id();

			AssetRegistry::<T>::insert(asset_id, &asset);

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

		fn fetch_reviews() -> Result<(), http::Error> {
			// Define the URL of your Firebase Cloud Function
			let url = "http://127.0.0.1:5001/future-console/us-central1/reviews";

			let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));

			let request = http::Request::get(url);

			let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;

			let response =
				pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;

			log::info!(" ✅ Response from reviews : {:?}", response);

			Ok(())
		}

		/// A helper function to fetch the price and send signed transaction.
		fn do_fetch_reviews() -> Result<(), &'static str> {
			let _ = Self::fetch_reviews().map_err(|_| "Failed to Reviews")?;

			Ok(())
		}
	}
}
