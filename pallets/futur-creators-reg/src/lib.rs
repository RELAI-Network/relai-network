#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub use relai_primitives::creatorsreg;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, sp_runtime, traits::Currency,
	};

	use frame_system::pallet_prelude::*;

	use relai_primitives::creatorsreg::DevInfo;

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
	#[pallet::getter(fn next_dev_id)]
	pub(super) type NextDevId<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn reg_fee)]
	pub(super) type RegFee<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn dev_regisitry)]
	pub(super) type DevRegistry<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, DevInfo, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RegistrationFeeSet,
		DevRegistered { who: T::AccountId, id: u32 },
		DevUnRegistered { who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		DevAlreadyRegistered,
		DevNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight({10000})]
		pub fn set_registration_fee(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			RegFee::<T>::set(amount);

			Self::deposit_event(Event::RegistrationFeeSet);

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight({10000})]
		pub fn register_developer(
			origin: OriginFor<T>,
			name: BoundedVec<u8, ConstU32<100>>,
			email: BoundedVec<u8, ConstU32<100>>,
			website: BoundedVec<u8, ConstU32<100>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			//TODO: Add team feature to have more that one dev + RBAC on client side

			// Check if the developer's address already exists in the registry
			ensure!(!DevRegistry::<T>::contains_key(&who), Error::<T>::DevAlreadyRegistered);

			// Generate a new unique ID for the developer
			let id = Self::next_dev_id() + 1;

			let dev_infos: DevInfo = DevInfo { id, name, email, website };

			// Increment the counter
			NextDevId::<T>::put(id);

			// Insert the new developer's information into the registry
			DevRegistry::<T>::insert(&who, dev_infos);

			//TODO: Withdraw from developar after setting where to put funds
			/*
			// Transfer the balance from the developer's account to the treasury
			T::Currency::transfer(
				&who,
				&T::TreasuryAccount::get(),
				amount,
				ExistenceRequirement::AllowDeath,
			)?;
			*/

			// Emit an event
			Self::deposit_event(Event::DevRegistered { who, id });

			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight({10000})]
		pub fn unregister_developer(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			// Check if the developer's address exists in the registry
			ensure!(!DevRegistry::<T>::contains_key(&who), Error::<T>::DevNotFound);

			// Remove developer's information from the registry
			DevRegistry::<T>::remove(&who);

			Self::deposit_event(Event::DevUnRegistered { who });

			Ok(().into())
		}
	}

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub registration_fee: BalanceOf<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			RegFee::<T>::put(self.registration_fee);
		}
	}
}
