#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::tokens::Balance;
use frame_support::{sp_runtime::{Perbill, Permill, RuntimeDebug}, BoundedVec};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::{
	fmt::Debug,
};
use sp_std::vec::Vec;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
use orml_traits::{
	arithmetic::{Signed, SimpleArithmetic},
	currency::TransferAll,
	BalanceStatus, BasicCurrency, BasicCurrencyExtended, BasicLockableCurrency, BasicReservableCurrency,
	LockIdentifier, MultiCurrency, MultiCurrencyExtended, MultiLockableCurrency, MultiReservableCurrency,
	NamedBasicReservableCurrency, NamedMultiReservableCurrency,
};
use sp_runtime::traits::{AccountIdConversion, BlockNumberProvider, CheckedAdd};
use sp_core::{ConstU32, Get};
use sp_std::vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod group_id;

pub type AssetId = u64;
pub type GroupId = u64;
pub type CandyId = u64;
pub type ServerId = u64;
pub type MemberCount = u32;

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub enum Visibility {
	Private,
	Public,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct MultiAsset<AssetId, Balance> {
	asset_id: AssetId,
	amount: Balance,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct Liquidity<MultiAsset> {
	a_asset: MultiAsset,
	b_asset: MultiAsset,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub enum GroupStatus {
	Active,
	Inactive,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct GroupInfo<AccountId, BlockNumber, Visibility, Liquidity, MultiAsset, GroupStatus> {
	owner: Option<AccountId>,
	commission: Perbill,
	group_account_id: AccountId,
	create_block_high: BlockNumber,
	visibility: Visibility,
	min_liquidity: Option<Liquidity>,
	max_members_number: u32,
	join_fee: Option<MultiAsset>,
	status: GroupStatus,
	members: Vec<AccountId>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct CandyInfo<MultiAsset, Balance, BlockNumber, AccountId> {
	group_id: u64,
	owner: AccountId,
	asset: MultiAsset,
	claimed_amount: Balance,
	max_lucky_number: MemberCount,
	claim_detail: Vec<(AccountId, Balance)>,
	end_block: BlockNumber,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	pub(crate) type BalanceOf<T> =
		<<T as Config>::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;
	pub(crate) type CurrencyIdOf<T> =
		<<T as Config>::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::CurrencyId;
	pub(crate) type MultiAssetOf<T> = MultiAsset<CurrencyIdOf<T>, BalanceOf<T>>;
	pub(crate) type GroupInfoOf<T> = GroupInfo<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, Visibility, Liquidity<MultiAssetOf<T>>, MultiAssetOf<T>, GroupStatus>;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type MultiCurrency: TransferAll<Self::AccountId>
			+ MultiCurrencyExtended<Self::AccountId>
			+ MultiLockableCurrency<Self::AccountId>
			+ MultiReservableCurrency<Self::AccountId>
			+ NamedMultiReservableCurrency<Self::AccountId>;
		type CandyExpire: Get<Self::BlockNumber>;
		type GroupIdConvertToAccountId: From<GroupId> + AccountIdConversion<Self::AccountId>;
		#[pallet::constant]
		type GetNativeCurrencyId: Get<CurrencyIdOf<Self>>;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn groups)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Groups<T: Config> = StorageMap<_, Twox64Concat, GroupId, GroupInfoOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_group_id)]
	pub type NextGroupId<T: Config> = StorageValue<_, GroupId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_candy_id)]
	pub type NextCandyId<T: Config> = StorageValue<_, CandyId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn candies)]
	pub type Candies<T: Config> = StorageMap<_, Twox64Concat, CandyId, CandyInfo<MultiAssetOf<T>, BalanceOf<T>, T::BlockNumber, T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn candies_of_group)]
	pub type CandiesOfGroup<T: Config> = StorageMap<_, Twox64Concat, GroupId, Vec<CandyId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn black_list_of_group)]
	pub type BlackListOfGroup<T: Config> = StorageMap<_, Twox64Concat, GroupId, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn invitees_of_group)]
	pub type InviteesOfGroup<T: Config> = StorageMap<_, Twox64Concat, GroupId, Vec<T::AccountId>, ValueQuery>;

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
		GroupNotExists,
		CandyNotExists,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn create_group(origin: OriginFor<T>, server_id: ServerId, min_liquidity: Option<Liquidity<MultiAssetOf<T>>>, max_members_number: u32, commission: Perbill, visibility: Visibility, join_fee: Option<MultiAssetOf<T>>) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;
			// todo group_account_id now()
			let next_group_id = NextGroupId::<T>::get();
			let group_account_id = T::GroupIdConvertToAccountId::from(next_group_id).into_account_truncating();
			Groups::<T>::insert(next_group_id, GroupInfo {
				owner: Some(creator.clone()),
				commission,
				group_account_id,
				create_block_high: Self::now(),
				visibility,
				min_liquidity,
				max_members_number,
				join_fee,
				status: GroupStatus::Active,
				members: vec![creator.clone()],
			});
			NextGroupId::<T>::put(next_group_id.checked_add(1u64).ok_or(Error::<T>::StorageOverflow)?);
			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn enter_group(origin: OriginFor<T>, group_id: GroupId, new_member: T::AccountId, liquidity: Option<Liquidity<MultiAssetOf<T>>>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Groups::<T>::mutate_exists(group_id, |g| -> DispatchResultWithPostInfo{
				let mut group = g.take().ok_or(Error::<T>::GroupNotExists)?;
				group.members.push(new_member.clone());
				*g = Some(group);
				Ok(().into())
			})?;
			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn leave_group(origin: OriginFor<T>, group_id: GroupId, old_member: T::AccountId) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Groups::<T>::mutate_exists(group_id, |g| -> DispatchResultWithPostInfo{
				let mut group = g.take().ok_or(Error::<T>::GroupNotExists)?;
				group.members.retain(|w| w != &old_member);
				*g = Some(group);
				Ok(().into())
			})?;
			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn disband_group(origin: OriginFor<T>, group_id: GroupId) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn give_candy(origin: OriginFor<T>, group_id: GroupId, asset: MultiAssetOf<T>, max_lucky_number: MemberCount) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let next_candy_id = NextCandyId::<T>::get();
			Candies::<T>::insert(next_candy_id, CandyInfo {
				group_id,
				owner,
				asset: asset,
				claimed_amount: BalanceOf::<T>::from(0u8),
				max_lucky_number: max_lucky_number,
				claim_detail: vec![],
				end_block: Self::now().checked_add(&T::CandyExpire::get()).ok_or(Error::<T>::StorageOverflow)?,
			});
			CandiesOfGroup::<T>::mutate(group_id, |h| h.push(next_candy_id));
			NextCandyId::<T>::put(next_candy_id.checked_add(1u64).ok_or(Error::<T>::StorageOverflow)?);
			//
			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn get_candy(origin: OriginFor<T>, group_id: GroupId, candy_id: CandyId, detail: Vec<(T::AccountId, BalanceOf<T>)>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			// detail.iter().for_each(|d| {
			// 	Candies::<T>::mutate_exists(candy_id, |c| -> DispatchResultWithPostInfo {
			// 		let mut candy = c.take().ok_or(Error::<T>::CandyNotExists)?;
			// 		let asset_id = candy.asset.asset_id.clone();
			// 		// T::MultiCurrency::repatriate_reserved_named()
			// 		Ok(().into())
			// 	})?;
			// });
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn now() -> T::BlockNumber {
			frame_system::Pallet::<T>::current_block_number()
		}
	}
}
