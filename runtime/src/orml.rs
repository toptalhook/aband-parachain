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
    vec![
    ]
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

parameter_types! {
	pub const NativeAssetId: CurrencyId = 0;
	pub const MaxStringLen: u32 = 10;
}

impl orml_currencies::Config for Runtime {
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
	type GetNativeCurrencyId = NativeAssetId;
    type WeightInfo = ();
}

