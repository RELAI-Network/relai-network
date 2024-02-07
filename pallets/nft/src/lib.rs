#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;
use sp_std::vec;

use orml_nft::Pallet as NftModule;


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	pub type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;


	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config
			+ orml_nft::Config<TokenData = (), ClassData = ()>
	 {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]

	pub type Something<T> = StorageValue<_, u32>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NftClassCreated {owner: T::AccountId},
	}

	#[pallet::error]
	pub enum Error<T> {}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_class())]
		pub fn create_class(origin: OriginFor<T>) -> DispatchResult {

			let owner = ensure_signed(origin)?;

            NftModule::<T>::create_class(&owner, vec![], ())?;

			Self::deposit_event(Event::NftClassCreated { owner });

			Ok(())
		}

	}
}
