#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	sp_runtime::{Perbill, Permill, RuntimeDebug},
	traits::tokens::Balance,
	BoundedVec,
};
use orml_traits::{
	arithmetic::{Signed, SimpleArithmetic},
	currency::TransferAll,
	BalanceStatus, BasicCurrency, BasicCurrencyExtended, BasicLockableCurrency,
	BasicReservableCurrency, LockIdentifier, MultiCurrency, MultiCurrencyExtended,
	MultiLockableCurrency, MultiReservableCurrency, NamedBasicReservableCurrency,
	NamedMultiReservableCurrency,
};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
use pallet_server::traits::{GetServerInfo, ServerManager};
use scale_info::TypeInfo;
use sp_core::{ConstU32, Get};
use sp_runtime::traits::{
	AccountIdConversion, BlockNumberProvider, CheckedAdd, CheckedMul, CheckedSub, Zero,
};
use sp_std::{fmt::Debug, result::Result, vec, vec::Vec};

// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;
//
//
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
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
	Disbanded,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct GroupInfo<AccountId, BlockNumber, Visibility, Liquidity, MultiAsset, GroupStatus> {
	owner: Option<AccountId>,
	creator: AccountId,
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
	use sp_runtime::Saturating;

	pub(crate) type BalanceOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;
	pub(crate) type CurrencyIdOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<
		<T as frame_system::Config>::AccountId,
	>>::CurrencyId;
	pub(crate) type MultiAssetOf<T> = MultiAsset<CurrencyIdOf<T>, BalanceOf<T>>;
	pub(crate) type GroupInfoOf<T> = GroupInfo<
		<T as frame_system::Config>::AccountId,
		<T as frame_system::Config>::BlockNumber,
		Visibility,
		Liquidity<MultiAssetOf<T>>,
		MultiAssetOf<T>,
		GroupStatus,
	>;
	pub(crate) type ReserveIdentifierOf<T> =
		<<T as Config>::MultiCurrency as NamedMultiReservableCurrency<
			<T as frame_system::Config>::AccountId,
		>>::ReserveIdentifier;

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
		type CandyReserveIdentifier: Get<ReserveIdentifierOf<Self>>;
		type ServerManager: ServerManager<ServerId, GroupId>;
		type GetServerInfo: GetServerInfo<ServerId, GroupId, Self::AccountId>;
		#[pallet::constant]
		type GetNativeCurrencyId: Get<CurrencyIdOf<Self>>;
		#[pallet::constant]
		type OneHundredSReserve: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn groups)]
	pub type Groups<T: Config> = StorageMap<_, Twox64Concat, GroupId, GroupInfoOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_group_id)]
	pub type NextGroupId<T: Config> = StorageValue<_, GroupId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_candy_id)]
	pub type NextCandyId<T: Config> = StorageValue<_, CandyId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn candies)]
	pub type Candies<T: Config> = StorageMap<
		_,
		Twox64Concat,
		CandyId,
		CandyInfo<MultiAssetOf<T>, BalanceOf<T>, T::BlockNumber, T::AccountId>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn candies_of_group)]
	pub type CandiesOfGroup<T: Config> =
		StorageMap<_, Twox64Concat, GroupId, Vec<CandyId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn server_of)]
	pub type ServerOf<T: Config> = StorageMap<_, Twox64Concat, GroupId, ServerId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn black_list_of_group)]
	pub type BlackListOfGroup<T: Config> =
		StorageMap<_, Twox64Concat, GroupId, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn invitees_of_group)]
	pub type InviteesOfGroup<T: Config> =
		StorageMap<_, Twox64Concat, GroupId, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn create_reserve)]
	pub type CreateReserveOfGroup<T: Config> =
		StorageMap<_, Twox64Concat, GroupId, BalanceOf<T>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		CreateGroup {
			creator: T::AccountId,
			group_id: GroupId,
		},
		Invite {
			group_id: GroupId,
			who: T::AccountId,
			invitee: T::AccountId,
		},

		EnterGroup {
			who: T::AccountId,
			group_id: GroupId,
		},

		KickSomeOne {
			group_id: GroupId,
			who: T::AccountId,
			someone_kicked: T::AccountId,
		},
		LeaveGroup {
			group_id: GroupId,
			old_member: T::AccountId,
		},
		DisbandGroup {
			group_id: GroupId,
		},
		GiveCandy {
			boss: T::AccountId,
			group_id: GroupId,
			candy: CandyId,
		},
		RemoveExpiredCandy {
			group_id: GroupId,
			candy_id: CandyId,
			asset_id: CurrencyIdOf<T>,
			remain_amount: BalanceOf<T>,
		},
		GetCandy {
			lucky_man: T::AccountId,
			group_id: GroupId,
			candy_id: GroupId,
			amount: MultiAssetOf<T>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		StorageOverflow,
		GroupNotExists,
		CandyNotExists,
		ServerAtCapacity,
		InBlackList,
		NotGroupOwner,
		GroupAtCapacity,
		PrivateGroup,
		PermissionDenied,
		OnlyOneMember,
		OwnerCannotLeave,
		MemberNotInGroup,
		GroupDisbanded,
		AmountIsZero,
		AlreadyGetCandy,
		CandyNotInGroup,
		RemainAmountIsZero,
		LuckyNumberUpMax,
		ClaimedAmountUpMax,
		MemberAlreadyGetCandy,
		NotServerOwner,
		GroupNotInAnyServer,
		ServerNotExists,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn create_group(
			origin: OriginFor<T>,
			server_id: Option<ServerId>,
			min_liquidity: Option<Liquidity<MultiAssetOf<T>>>,
			max_members_number: u32,
			commission: Perbill,
			visibility: Visibility,
			join_fee: Option<MultiAssetOf<T>>,
		) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;

			let next_group_id = NextGroupId::<T>::get();
			let server_id = server_id.unwrap_or_else(|| Self::get_official_server());
			ensure!(T::GetServerInfo::is_server_exists(server_id), Error::<T>::ServerNotExists);
			let group_account_id =
				T::GroupIdConvertToAccountId::from(next_group_id).into_account_truncating();

			let reserve_balance = BalanceOf::<T>::from((max_members_number as u32) / 100 + 1)
				.checked_mul(&T::OneHundredSReserve::get())
				.ok_or(Error::<T>::StorageOverflow)?;
			T::MultiCurrency::reserve(T::GetNativeCurrencyId::get(), &creator, reserve_balance)?;
			CreateReserveOfGroup::<T>::insert(next_group_id, reserve_balance);
			Groups::<T>::insert(
				next_group_id,
				GroupInfo {
					owner: Some(creator.clone()),
					creator: creator.clone(),
					commission,
					group_account_id,
					create_block_high: Self::now(),
					visibility,
					min_liquidity,
					max_members_number,
					join_fee,
					status: GroupStatus::Active,
					members: vec![creator.clone()],
				},
			);
			NextGroupId::<T>::put(
				next_group_id.checked_add(1u64).ok_or(Error::<T>::StorageOverflow)?,
			);
			Self::try_update_server_info(server_id, next_group_id, false)?;
			Self::deposit_event(Event::CreateGroup { creator, group_id: next_group_id });

			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn enter_group(
			origin: OriginFor<T>,
			group_id: GroupId,
			new_member: T::AccountId,
			liquidity: Option<Liquidity<MultiAssetOf<T>>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let mut group_info = Self::try_get_exists_and_active_group(group_id)?;

			let is_invited = who != new_member;
			ensure!(
				BlackListOfGroup::<T>::get(group_id)
					.iter()
					.position(|p| p == &new_member)
					.is_none(),
				Error::<T>::InBlackList
			);

			if is_invited {
				// must be group owner
				ensure!(Some(who.clone()) == group_info.owner, Error::<T>::NotGroupOwner);
				InviteesOfGroup::<T>::mutate(group_id, |g| {
					g.retain(|w| w != &new_member);
					g.push(new_member.clone());
				});
				Self::deposit_event(Event::Invite { group_id, who, invitee: new_member });
				return Ok(().into())
			}

			ensure!(
				group_info.max_members_number > group_info.members.len() as u32,
				Error::<T>::GroupAtCapacity
			);
			ensure!(
				group_info.visibility == Visibility::Private &&
					InviteesOfGroup::<T>::get(group_id)
						.iter()
						.position(|p| p == &new_member)
						.is_some() || group_info.visibility == Visibility::Public,
				Error::<T>::PermissionDenied
			);

			Self::try_add_liquidity(group_id, liquidity)?;

			group_info.members.retain(|w| w != &new_member);
			group_info.members.push(new_member.clone());
			group_info
				.max_members_number
				.checked_add(1 as u32)
				.ok_or(Error::<T>::StorageOverflow)?;
			Groups::<T>::insert(group_id, group_info);

			Self::deposit_event(Event::EnterGroup { who: new_member, group_id });
			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn leave_group(
			origin: OriginFor<T>,
			group_id: GroupId,
			old_member: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let mut group_info = Self::try_get_exists_and_active_group(group_id)?;

			let is_kick = old_member != who;
			if is_kick {
				ensure!(Some(who.clone()) == group_info.owner, Error::<T>::NotGroupOwner);
				BlackListOfGroup::<T>::mutate(group_id, |b| {
					b.retain(|r| r != &old_member);
					b.push(old_member.clone());
				});
			}

			ensure!(group_info.members.len() > 1 as usize, Error::<T>::OnlyOneMember);
			ensure!(group_info.owner != Some(old_member.clone()), Error::<T>::OwnerCannotLeave);

			if let Some(pos) = group_info.members.iter().position(|p| p == &old_member) {
				group_info.members.remove(pos);
			} else {
				return Err(Error::<T>::MemberNotInGroup)?
			}

			Groups::<T>::insert(group_id, group_info);

			if is_kick {
				Self::deposit_event(Event::KickSomeOne {
					group_id,
					who,
					someone_kicked: old_member,
				});
			} else {
				Self::deposit_event(Event::LeaveGroup { group_id, old_member })
			}

			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn disband_group(
			origin: OriginFor<T>,
			group_id: GroupId,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;

			let mut group_info = Self::try_get_exists_and_active_group(group_id)?;
			ensure!(group_info.owner == Some(owner.clone()), Error::<T>::NotGroupOwner);
			group_info.status = GroupStatus::Disbanded;

			CandiesOfGroup::<T>::try_mutate(group_id, |c_ids| -> DispatchResult {
				c_ids.iter().for_each(|c_id| {
					if let Some(c) = Candies::<T>::take(c_id) {
						let remain_amount = c.asset.amount.saturating_sub(c.claimed_amount);
						if !remain_amount.is_zero() {
							T::MultiCurrency::unreserve_named(
								&T::CandyReserveIdentifier::get(),
								c.asset.asset_id,
								&c.owner,
								remain_amount,
							);
						}
					}
				});
				c_ids.clear();
				Ok(())
			})?;
			let unreserve_balance = CreateReserveOfGroup::<T>::get(group_id);
			if !unreserve_balance.is_zero() {
				T::MultiCurrency::unreserve(
					T::GetNativeCurrencyId::get(),
					&group_info.creator,
					unreserve_balance,
				);
			}
			Groups::<T>::insert(group_id, group_info);

			BlackListOfGroup::<T>::remove(group_id);
			InviteesOfGroup::<T>::remove(group_id);

			if let Some(server_id) = Self::get_server_id(group_id) {
				Self::try_update_server_info(server_id, group_id, true)?;
			}

			Self::deposit_event(Event::<T>::DisbandGroup { group_id });

			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn give_candy(
			origin: OriginFor<T>,
			group_id: GroupId,
			asset: MultiAssetOf<T>,
			max_lucky_number: MemberCount,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;

			ensure!(!asset.amount.is_zero(), Error::<T>::AmountIsZero);
			let next_candy_id = NextCandyId::<T>::get();
			// T::MultiCurrency::reserve_named()
			Candies::<T>::insert(
				next_candy_id,
				CandyInfo {
					group_id,
					owner: owner.clone(),
					asset: asset.clone(),
					claimed_amount: BalanceOf::<T>::from(0u8),
					max_lucky_number,
					claim_detail: vec![],
					end_block: Self::now()
						.checked_add(&T::CandyExpire::get())
						.ok_or(Error::<T>::StorageOverflow)?,
				},
			);
			CandiesOfGroup::<T>::mutate(group_id, |h| h.push(next_candy_id));
			NextCandyId::<T>::put(
				next_candy_id.checked_add(1u64).ok_or(Error::<T>::StorageOverflow)?,
			);
			Self::deposit_event(Event::GiveCandy {
				boss: owner.clone(),
				group_id,
				candy: next_candy_id,
			});
			T::MultiCurrency::reserve_named(
				&T::CandyReserveIdentifier::get(),
				asset.asset_id,
				&owner,
				asset.amount,
			)?;
			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn get_candy(
			origin: OriginFor<T>,
			group_id: GroupId,
			candy_id: CandyId,
			detail: Vec<(T::AccountId, BalanceOf<T>)>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let group = Self::try_get_exists_and_active_group(group_id)?;

			ensure!(
				CandiesOfGroup::<T>::get(group_id).iter().position(|p| p == &candy_id).is_some(),
				Error::<T>::CandyNotInGroup
			);

			let mut candy = Candies::<T>::get(candy_id).ok_or(Error::<T>::CandyNotExists)?;

			if candy.end_block < Self::now() {
				let remain_amount = candy.asset.amount.saturating_sub(candy.claimed_amount);
				if !remain_amount.is_zero() {
					T::MultiCurrency::unreserve_named(
						&T::CandyReserveIdentifier::get(),
						candy.asset.asset_id,
						&candy.owner,
						remain_amount,
					);
				}
				CandiesOfGroup::<T>::mutate(group_id, |cs| cs.retain(|c| c != &candy_id));
				Candies::<T>::remove(group_id);
				Self::deposit_event(Event::<T>::RemoveExpiredCandy {
					group_id,
					candy_id,
					asset_id: candy.asset.asset_id,
					remain_amount,
				});
				return Ok(().into())
			}

			if let Some(server_id) = ServerOf::<T>::get(group_id) {
				ensure!(Self::is_server_owner(server_id, who.clone()), Error::<T>::NotServerOwner);
			} else {
				return Err(Error::<T>::GroupNotInAnyServer)?
			}

			for (lucky_man, amount) in detail {
				if let None = candy.claim_detail.iter().position(|p| p.0 == lucky_man) {
					ensure!(
						group.members.iter().position(|p| p == &lucky_man).is_some(),
						Error::<T>::MemberNotInGroup
					);
					ensure!(
						candy.claim_detail.iter().position(|p| p.0 == lucky_man).is_none(),
						Error::<T>::MemberAlreadyGetCandy
					);

					ensure!(
						candy.max_lucky_number > candy.claim_detail.len() as u32,
						Error::<T>::LuckyNumberUpMax
					);
					let new_claimed_amount = candy
						.claimed_amount
						.checked_add(&amount)
						.ok_or(Error::<T>::StorageOverflow)?;

					ensure!(
						new_claimed_amount <= candy.asset.amount,
						Error::<T>::ClaimedAmountUpMax
					);

					candy.claimed_amount = new_claimed_amount;
					candy.claim_detail.push((lucky_man.clone(), amount));

					T::MultiCurrency::repatriate_reserved_named(
						&T::CandyReserveIdentifier::get(),
						candy.asset.asset_id,
						&candy.owner,
						&lucky_man,
						amount,
						BalanceStatus::Free,
					)?;
					Self::deposit_event(Event::GetCandy {
						lucky_man,
						group_id,
						candy_id,
						amount: MultiAsset { asset_id: candy.asset.asset_id, amount },
					});
				} else {
					return Err(Error::<T>::AlreadyGetCandy)?
				}
			}

			Candies::<T>::insert(candy_id, candy);

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn now() -> T::BlockNumber {
			frame_system::Pallet::<T>::current_block_number()
		}

		fn try_get_exists_and_active_group(
			group_id: GroupId,
		) -> Result<GroupInfoOf<T>, DispatchError> {
			let group_info = Groups::<T>::get(group_id).ok_or(Error::<T>::GroupNotExists)?;
			if group_info.status == GroupStatus::Active {
				return Ok(group_info)
			}
			return Err(Error::<T>::GroupDisbanded)?
		}

		pub fn is_server_owner(server_id: ServerId, maybe_owner: T::AccountId) -> bool {
			if let Ok(owner) = T::GetServerInfo::try_get_server_owner(server_id) {
				if owner == Some(maybe_owner) {
					return true
				}
			}
			false
		}

		pub fn get_official_server() -> ServerId {
			0 as ServerId
		}

		pub fn is_server_at_capacity(server_id: ServerId) -> bool {
			T::GetServerInfo::is_at_capacity(server_id)
		}

		pub fn try_update_server_info(
			server_id: ServerId,
			group_id: GroupId,
			is_remove_group: bool,
		) -> DispatchResult {
			if is_remove_group {
				// todo
				T::ServerManager::try_remove_old_group(server_id, group_id)
			} else {
				ServerOf::<T>::insert(group_id, server_id);
				T::ServerManager::try_add_new_group(server_id, group_id)
			}
		}

		pub fn try_add_liquidity(
			group_id: GroupId,
			liquidity: Option<Liquidity<MultiAssetOf<T>>>,
		) -> DispatchResult {
			Ok(())
		}

		pub fn get_server_id(group_id: GroupId) -> Option<ServerId> {
			ServerOf::<T>::get(group_id)
		}
	}
}
