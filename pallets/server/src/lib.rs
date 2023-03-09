#![cfg_attr(not(feature = "std"), no_std)]

use crate::traits::{GetServerInfo, ServerManager};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{dispatch::DispatchResult, ensure, RuntimeDebug};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{AccountIdConversion, BlockNumberProvider, CheckedAdd},
	DispatchError, Perbill,
};
use sp_std::vec::Vec;

pub mod server_id;
pub mod traits;

// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;
//
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub type ServerId = u64;
pub type GroupId = u64;
pub type Balance = u128;

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub enum Fees<Balance> {
	MiningCommission(Perbill),
	OneByte(Balance),
}

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

		type ServerIdConvertToAccountId: From<ServerId> + AccountIdConversion<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn servers)]
	pub type Servers<T: Config> =
		StorageMap<_, Twox64Concat, ServerId, ServerDetails<T::AccountId, Vec<u8>>>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn next_server_id)]
	pub type NextServerId<T: Config> = StorageValue<_, ServerId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn groups_of_Server)]
	pub type GroupsOfServer<T: Config> =
		StorageMap<_, Twox64Concat, ServerId, Vec<GroupId>, ValueQuery>;

	#[pallet::type_value]
	pub fn MaxGroupOfServerOnEmpty<T: Config>() -> GroupId {
		1000 as GroupId
	}

	#[pallet::storage]
	#[pallet::getter(fn max_group)]
	pub type MaxGroupOfServer<T: Config> =
		StorageMap<_, Twox64Concat, ServerId, GroupId, ValueQuery, MaxGroupOfServerOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn one_byte_pay_of_server)]
	pub type OneBytePayOfServer<T: Config> =
		StorageMap<_, Twox64Concat, ServerId, Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn mining_commission_of_server)]
	pub type MiningCommissionOfServer<T: Config> =
		StorageMap<_, Twox64Concat, ServerId, Perbill, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		Register {
			server_id: ServerId,
			creator: T::AccountId,
		},
		SetServerOwner {
			server_id: ServerId,
			new_owner: Option<T::AccountId>,
		},
		SetServerFees {
			server_id: ServerId,
			fee: Fees<Balance>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		ServerNotExists,
		NotServerOwner,
		UndefinedFee,
		ServerAtCapacity,
		GroupAlreadyExists,
		GroupNotInServer,
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
			let creator = ensure_signed(origin)?;
			let next_server_id = NextServerId::<T>::get();
			Servers::<T>::insert(
				next_server_id,
				ServerDetails {
					creator: creator.clone(),
					owner: Some(creator.clone()),
					server_account_id: T::ServerIdConvertToAccountId::from(next_server_id)
						.into_account_truncating(),
					metadata,
				},
			);
			NextServerId::<T>::put(
				next_server_id.checked_add(1 as ServerId).ok_or(Error::<T>::StorageOverflow)?,
			);

			Self::deposit_event(Event::Register { server_id: next_server_id, creator });

			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn set_server_owner(
			origin: OriginFor<T>,
			server_id: ServerId,
			new_owner: Option<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			let old_owner = ensure_signed(origin)?;
			let mut server_info =
				Servers::<T>::get(server_id).ok_or(Error::<T>::ServerNotExists)?;
			ensure!(Some(old_owner) == server_info.owner, Error::<T>::NotServerOwner);
			server_info.owner = new_owner.clone();
			Servers::<T>::insert(server_id, server_info);
			Self::deposit_event(Event::SetServerOwner { server_id, new_owner });
			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn set_server_fees(
			origin: OriginFor<T>,
			server_id: ServerId,
			fee: Fees<Balance>,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let server_info = Servers::<T>::get(server_id).ok_or(Error::<T>::ServerNotExists)?;
			ensure!(Some(owner.clone()) == server_info.owner, Error::<T>::NotServerOwner);

			match fee {
				Fees::MiningCommission(p) => {
					MiningCommissionOfServer::<T>::insert(server_id, p);
				},
				Fees::OneByte(b) => {
					OneBytePayOfServer::<T>::insert(server_id, b);
				},
				_ => return Err(Error::<T>::UndefinedFee)?,
			};
			Self::deposit_event(Event::SetServerFees { server_id, fee });
			Ok(().into())
		}
	}
}

impl<T: Config> traits::GetServerInfo<ServerId, GroupId, T::AccountId> for Pallet<T> {
	fn try_get_server_owner(server_id: ServerId) -> Result<Option<T::AccountId>, DispatchError> {
		let server = Servers::<T>::get(server_id).ok_or(Error::<T>::ServerNotExists)?;
		Ok(server.owner)
	}

	fn try_get_server_creator(server_id: ServerId) -> Result<T::AccountId, DispatchError> {
		let server = Servers::<T>::get(server_id).ok_or(Error::<T>::ServerNotExists)?;
		Ok(server.creator)
	}

	fn try_get_server_account_id(server_id: ServerId) -> Result<T::AccountId, DispatchError> {
		let server = Servers::<T>::get(server_id).ok_or(Error::<T>::ServerNotExists)?;
		Ok(server.server_account_id)
	}

	fn is_at_capacity(server_id: ServerId) -> bool {
		let max = MaxGroupOfServer::<T>::get(server_id);
		let len: GroupId = GroupsOfServer::<T>::get(server_id).len() as GroupId;
		if len >= max {
			return true
		}
		false
	}
}

impl<T: Config> ServerManager<ServerId, GroupId> for Pallet<T> {
	fn try_add_new_group(server_id: ServerId, group_id: GroupId) -> DispatchResult {
		ensure!(!Self::is_at_capacity(server_id), Error::<T>::ServerAtCapacity);

		GroupsOfServer::<T>::mutate_exists(server_id, |gs| -> DispatchResult {
			let mut gss = gs.take().ok_or(Error::<T>::GroupAlreadyExists)?;
			if gss.iter().position(|p| p == &group_id).is_some() {
				return Err(Error::<T>::GroupAlreadyExists)?
			}
			gss.push(group_id);
			*gs = Some(gss);
			Ok(())
		})
	}

	fn try_remove_old_group(server_id: ServerId, group_id: GroupId) -> DispatchResult {
		GroupsOfServer::<T>::mutate_exists(server_id, |gs| -> DispatchResult {
			let mut gss = gs.take().ok_or(Error::<T>::GroupAlreadyExists)?;
			if let Some(p) = gss.iter().position(|p| p == &group_id) {
				gss.remove(p);
			} else {
				return Err(Error::<T>::GroupNotInServer)?
			}
			*gs = Some(gss);
			Ok(())
		})
	}
}
