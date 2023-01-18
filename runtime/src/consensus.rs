use super::*;
use primitives::origin::EnsureRootOrHalfCouncil;

impl pallet_author_inherent::Config for Runtime {
	type AccountLookup = Collators;
	type CanAuthor = AuthorFilter;
	// We start a new slot each time we see a new relay block.
	type SlotBeacon = cumulus_pallet_parachain_system::RelaychainBlockNumberProvider<Self>;
	type WeightInfo = ();
}

impl pallet_author_slot_filter::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RandomnessSource = RandomnessCollectiveFlip;
	type PotentialAuthors = Collators;
	type WeightInfo = ();
}

impl pallet_collators::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AuthorityOrigin = EnsureRootOrHalfCouncil;
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type WeightInfo = ();
}
