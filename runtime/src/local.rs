use super::*;

// Configure the pallet-template in pallets/template.
impl pallet_template::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = 0;
	pub const CandyExpire: BlockNumber = DAYS;
	pub const OneHundredSReserve: Balance = 2 * UNIT;
	pub const CandyReserveIdentifier: [u8; 8] = *b"candy   ";
}

impl pallet_group::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MultiCurrency = Currencies;
	type CandyExpire = CandyExpire;
	type GroupIdConvertToAccountId = pallet_group::group_id::GroupId<u64>;
	type CandyReserveIdentifier = CandyReserveIdentifier;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type OneHundredSReserve = OneHundredSReserve;
}

impl pallet_server::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ServerIdConvertToAccountId = pallet_server::server_id::ServerId<u64>;
}
