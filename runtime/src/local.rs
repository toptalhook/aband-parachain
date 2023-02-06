use super::*;

// Configure the pallet-template in pallets/template.
impl pallet_template::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = 0;
}

impl pallet_group::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MultiCurrency = Currencies;
	type GroupIdConvertToAccountId = pallet_group::group_id::GroupId<u64>;
	type GetNativeCurrencyId = GetNativeCurrencyId;
}
