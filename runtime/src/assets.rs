use super::*;
use frame_support::traits::Contains;
use orml_currencies::BasicCurrencyAdapter;
use orml_traits::parameter_type_with_key;
use sp_runtime::traits::Zero;

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
	fn contains(a: &AccountId) -> bool {
		get_all_module_accounts().contains(a)
	}
}

parameter_types! {
	pub const TokensMaxReserves: u32 = 50;
}

pub fn get_all_module_accounts() -> Vec<AccountId> {
	vec![]
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Zero::zero()
	};
}

impl orml_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CurrencyHooks = ();
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = MaxLocks;
	type MaxReserves = TokensMaxReserves;
	type ReserveIdentifier = [u8; 8];
	type DustRemovalWhitelist = DustRemovalWhitelist;
}

impl orml_currencies::Config for Runtime {
	type MultiCurrency = Tokens;
	type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = ();
}

parameter_types! {
	pub const CreateReserveBalance: Balance = 10*UNIT;
	pub const MaxStringLen: u32 = 20;
	pub const GetNativeCurrencyId: CurrencyId = 0;
	pub const CreateAssetReserveIdentifier: [u8; 8] = *b" c-asset";
}

impl pallet_assets_manage::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MultiCurrency = Currencies;
	type CurrencyIdConvertToAccountId = pallet_assets_manage::asset_id::AssetId<CurrencyId>;
	type CreateReserveBalance = CreateReserveBalance;
	type MaxStringLen = MaxStringLen;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type CreateAssetReserveIdentifier = CreateAssetReserveIdentifier;
}
