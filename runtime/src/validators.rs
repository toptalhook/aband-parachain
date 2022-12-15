use frame_support::pallet;

use frame_support::traits::OneSessionHandler;
use nimbus_primitives::{NimbusId, NIMBUS_ENGINE_ID};
pub use pallet::*;
use sp_std::prelude::Vec;

#[pallet]
pub mod pallet {

	use frame_support::pallet_prelude::*;
	#[cfg(feature = "std")]
	use log::warn;
	use nimbus_primitives::{AccountLookup, CanAuthor, NimbusId};
	use sp_std::vec::Vec;

	/// The Account Set pallet
	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	/// Configuration trait of this pallet.
	#[pallet::config]
	pub trait Config: frame_system::Config {}

	impl<T> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
		type Public = NimbusId;
	}

	/// The set of accounts that is stored in this pallet.
	#[pallet::storage]
	pub type StoredAccounts<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	impl<T: Config> Get<Vec<T::AccountId>> for Pallet<T> {
		fn get() -> Vec<T::AccountId> {
			StoredAccounts::<T>::get()
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn account_id_of)]
	/// A mapping from the AuthorIds used in the consensus layer
	/// to the AccountIds runtime.
	pub type Mapping<T: Config> = StorageMap<_, Twox64Concat, NimbusId, T::AccountId, OptionQuery>;

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
				StoredAccounts::<T>::append(account_id);
			}
		}
	}

	/// This pallet is compatible with nimbus's author filtering system. Any account stored in this pallet
	/// is a valid author. Notice that this implementation does not have an inner filter, so it
	/// can only be the beginning of the nimbus filter pipeline.
	impl<T: Config> CanAuthor<T::AccountId> for Pallet<T> {
		fn can_author(author: &T::AccountId, _slot: &u32) -> bool {
			StoredAccounts::<T>::get().contains(author)
		}
	}

	impl<T: Config> AccountLookup<T::AccountId> for Pallet<T> {
		fn lookup_account(author: &NimbusId) -> Option<T::AccountId> {
			Mapping::<T>::get(&author)
		}
	}
}

impl<T: Config> OneSessionHandler<T::AccountId> for Pallet<T> {
	type Key = NimbusId;

	fn on_genesis_session<'a, I: 'a>(validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, NimbusId)>,
	{
	}

	fn on_new_session<'a, I: 'a>(changed: bool, validators: I, _queued_validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, NimbusId)>,
	{
		let authorities = validators.map(|(n, k)| (n, k)).collect::<Vec<_>>();
		if authorities.len() > 0 {
			StoredAccounts::<T>::kill();
			Mapping::<T>::remove_all(None);
			authorities.iter().for_each(|(x, y)| {
				StoredAccounts::<T>::append(x.clone());
				Mapping::<T>::insert(y, x.clone())
			})
		}
	}

	fn on_disabled(i: u32) {}
}
