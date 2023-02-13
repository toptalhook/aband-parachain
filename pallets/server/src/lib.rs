#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
use codec::{Encode, Decode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

pub mod server_id;
pub mod traits;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub type ServerId = u64;
pub type GroupId = u64;

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct ServerDetails<AccountId, Metadata> {
	creator: AccountId,
	owner: Option<AccountId>,
	server_account_id: AccountId,
	metadata: Metadata,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn servers)]
	pub type Servers<T: Config> = StorageMap<_, Twox64Concat, ServerId, ServerDetails<T::AccountId, Vec<u8>>>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn next_server_id)]
	pub type NextServerId<T: Config> = StorageValue<_, ServerId>;

	#[pallet::storage]
	#[pallet::getter(fn groups_of_Server)]
	pub type GroupsOfServer<T: Config> = StorageMap<_, Twox64Concat, ServerId, Vec<GroupId>, ValueQuery>;

	#[pallet::type_value]
	pub fn MaxGroupOfServerOnEmpty<T: Config>() -> GroupId {
		1000 as GroupId
	}

	#[pallet::storage]
	#[pallet::getter(fn max_group)]
	pub type MaxGroupOfServer<T: Config> = StorageMap<_, Twox64Concat, ServerId, GroupId, ValueQuery, MaxGroupOfServerOnEmpty<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn register(origin: OriginFor<T>, metadata: Vec<u8>) -> DispatchResultWithPostInfo {

			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn set_server_owner(origin: OriginFor<T>, owner: T::AccountId) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}
}
