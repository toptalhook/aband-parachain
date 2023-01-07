// Copyright 2023 Aband-TEAM.
// This file is part of substrate-parachain-PoS-template.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The role of the Collators pallet is to provide a collators set for consensus.
//! The validator can come from the staking module, or can also be set by `AuthorityOrigin` in this module.
//! It means that with this template, you can also use the Staking function in the case of PoA,
//! which is very useful if you just only want to reward collators.


#![cfg_attr(not(feature = "std"), no_std)]
#![allow(deprecated)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
mod weights;

use weights::WeightInfo;
use frame_support::{
	pallet,
	traits::{EnsureOrigin, OneSessionHandler},
};
use nimbus_primitives::{NimbusId};
pub use pallet::*;
use sp_std::prelude::Vec;

#[pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	#[cfg(feature = "std")]
	use log::warn;
	use nimbus_primitives::{AccountLookup, CanAuthor, NimbusId};
	use sp_std::vec::Vec;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		///
		type AuthorityOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		///
		type WeightInfo: WeightInfo;
	}

	impl<T> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
		type Public = NimbusId;
	}

	/// Whether PoS is turned off.
	///
	/// `set_collators`, `add_collator` and `remove_collator` can be executed only when PoS is turned off.
	#[pallet::storage]
	#[pallet::getter(fn is_closed_pos)]
	pub type IsClosedPoS<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// The set of collators.
	#[pallet::storage]
	#[pallet::getter(fn get_collators)]
	pub type Collators<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	impl<T: Config> Get<Vec<T::AccountId>> for Pallet<T> {
		fn get() -> Vec<T::AccountId> {
			Collators::<T>::get()
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn account_id_of)]
	/// A mapping from the AuthorIds used in the consensus layer
	/// to the AccountIds runtime.
	pub type Mapping<T: Config> = StorageMap<_, Twox64Concat, NimbusId, T::AccountId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClosePoS,
		OpenPoS,
		SetCollators { collators: Vec<T::AccountId> },
		AddCollator { new_collator: T::AccountId },
		RemoveCollator { old_collator: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		ShouldUnderPoA,
	}

	#[pallet::genesis_config]
	/// Genesis config for author mapping pallet
	pub struct GenesisConfig<T: Config> {
		/// The associations that should exist at chain genesis
		pub mapping: Vec<(T::AccountId, NimbusId)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { mapping: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if self.mapping.is_empty() {
				warn!(target: "account-set", "No mappings at genesis. Your chain will have no valid authors.");
			}
			for (account_id, author_id) in &self.mapping {
				Mapping::<T>::insert(author_id, account_id);
				Collators::<T>::append(account_id);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Turn off PoS to use PoA.
		#[pallet::weight(<T as pallet::Config>::WeightInfo::close_pos())]
		pub fn close_pos(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			T::AuthorityOrigin::ensure_origin(origin)?;
			IsClosedPoS::<T>::put(true);
			Self::deposit_event(Event::ClosePoS);
			Ok(().into())
		}

		/// Reopen PoS.
		#[pallet::weight(<T as pallet::Config>::WeightInfo::open_pos())]
		pub fn open_pos(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			T::AuthorityOrigin::ensure_origin(origin)?;
			IsClosedPoS::<T>::put(false);
			Self::deposit_event(Event::OpenPoS);
			Ok(().into())
		}

		///Set collators set.
		///
		/// only PoA can be used.
		#[pallet::weight(<T as pallet::Config>::WeightInfo::set_collators())]
		pub fn set_collators(
			origin: OriginFor<T>,
			collators: Vec<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			T::AuthorityOrigin::ensure_origin(origin)?;
			ensure!(Self::is_closed_pos(), Error::<T>::ShouldUnderPoA);
			Collators::<T>::put(&collators);
			Self::deposit_event(Event::SetCollators { collators });
			Ok(().into())
		}

		/// Add collator.
		///
		/// only PoA can be used.
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_collator())]
		pub fn add_collator(
			origin: OriginFor<T>,
			new_collator: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::AuthorityOrigin::ensure_origin(origin)?;
			ensure!(Self::is_closed_pos(), Error::<T>::ShouldUnderPoA);
			Collators::<T>::append(&new_collator);
			Self::deposit_event(Event::AddCollator { new_collator });
			Ok(().into())
		}

		/// Remove collator.
		///
		/// only PoA can be used.
		#[pallet::weight(<T as pallet::Config>::WeightInfo::remove_collator())]
		pub fn remove_collator(
			origin: OriginFor<T>,
			old_collator: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::AuthorityOrigin::ensure_origin(origin)?;
			ensure!(Self::is_closed_pos(), Error::<T>::ShouldUnderPoA);
			Collators::<T>::mutate(|v| {
				v.retain(|h| h != &old_collator);
			});
			Self::deposit_event(Event::RemoveCollator { old_collator });
			Ok(().into())
		}
	}

	impl<T: Config> CanAuthor<T::AccountId> for Pallet<T> {
		fn can_author(author: &T::AccountId, _slot: &u32) -> bool {
			Collators::<T>::get().contains(author)
		}
	}

	impl<T: Config> AccountLookup<T::AccountId> for Pallet<T> {
		fn lookup_account(author: &NimbusId) -> Option<T::AccountId> {
			Mapping::<T>::get(author)
		}
	}
}

impl<T: Config> OneSessionHandler<T::AccountId> for Pallet<T> {
	type Key = NimbusId;

	fn on_genesis_session<'a, I: 'a>(validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, NimbusId)>,
	{
		let authorities = validators.map(|(n, k)| (n, k)).collect::<Vec<_>>();
		assert!(!authorities.is_empty(), "authorities set is empty.");
		if !authorities.is_empty() {
			authorities.iter().for_each(|(x, y)| {
				Collators::<T>::append(x);
				Mapping::<T>::insert(y, x)
			});
		}
	}

	fn on_new_session<'a, I: 'a>(_changed: bool, validators: I, _queued_validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, NimbusId)>,
	{
		let authorities = validators.map(|(n, k)| (n, k)).collect::<Vec<_>>();
		if !authorities.is_empty() {
			// update collators set
			if !Self::is_closed_pos() {
				Collators::<T>::kill();
				Mapping::<T>::remove_all(None);
				authorities.iter().for_each(|(x, y)| {
					Collators::<T>::append(x);
					Mapping::<T>::insert(y, x)
				});
			} else {
				// update session-key
				authorities.iter().for_each(|(x, y)| Mapping::<T>::insert(y, x));
			}
		}
	}

	fn on_disabled(_i: u32) {}
}
