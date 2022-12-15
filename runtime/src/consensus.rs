use super::*;
impl pallet_author_inherent::Config for Runtime {
	// We start a new slot each time we see a new relay block.
	type SlotBeacon = cumulus_pallet_parachain_system::RelaychainBlockNumberProvider<Self>;
	type AccountLookup = Validators;
	type CanAuthor = AuthorFilter;
	type WeightInfo = ();
}

impl pallet_author_slot_filter::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RandomnessSource = RandomnessCollectiveFlip;
	type PotentialAuthors = Validators;
	type WeightInfo = ();
}

impl validators::Config for Runtime {}
