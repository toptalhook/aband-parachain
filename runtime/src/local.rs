use super::*;

// Configure the pallet-template in pallets/template.
impl pallet_template::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = 0;
	pub const CandyExpire: BlockNumber = DAYS;
}

impl pallet_group::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MultiCurrency = Currencies;
	type CandyExpire = CandyExpire;
	type GroupIdConvertToAccountId = pallet_group::group_id::GroupId<u64>;
	type GetNativeCurrencyId = GetNativeCurrencyId;
}

impl pallet_server::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ServerIdConvertToAccountId = pallet_server::server_id::ServerId<u64>;
}
