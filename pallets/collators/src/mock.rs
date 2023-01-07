#![allow(dead_code)]
#![allow(unused_variables)]
use crate as collators;
use frame_support::traits::{ConstU16, ConstU32, ConstU64, GenesisBuild};
use frame_system::{self, EnsureRoot};
use nimbus_primitives::NimbusPair;
use nimbus_primitives::{NimbusId, NimbusPair};
use sp_core::{Pair, H256, U256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
use sp_std::{prelude::Vec, result::Result};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Collators: collators::{Pallet, Call, Config<T>, Storage, Event<T>},
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
}

impl collators::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AuthorityOrigin = EnsureRoot<u64>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	collators::GenesisConfig::<Test> {
		mapping: vec![(1, NimbusPair::from_seed(&U256::from(1).into()).public())],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}
