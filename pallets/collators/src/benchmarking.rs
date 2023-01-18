#![allow(unused_imports)]

use super::*;
use crate::{Config, Pallet};
use frame_benchmarking::{
	account, benchmarks, benchmarks_instance, impl_benchmark_test_suite, whitelisted_caller,
};
use frame_support::dispatch::Pays::No;
use frame_system::RawOrigin as SystemOrigin;
use sp_runtime::RuntimeAppPublic;
use sp_std::vec;

fn get_alice<T: Config>() -> CollatorInfo<T::AccountId, NimbusId> {
	let alice = account("alice", 1, 1);
	CollatorInfo { validator: alice, nimbus_id: NimbusId::generate_pair(None) }
}

fn get_bob<T: Config>() -> CollatorInfo<T::AccountId, NimbusId> {
	let bob = account("bob", 2, 2);
	CollatorInfo { validator: bob, nimbus_id: NimbusId::generate_pair(None) }
}

fn get_dave<T: Config>() -> CollatorInfo<T::AccountId, NimbusId> {
	let dave = account("dave", 3, 3);
	CollatorInfo { validator: dave, nimbus_id: NimbusId::generate_pair(None) }
}

fn close_pos_test<T: Config>() {
	IsClosedPoS::<T>::put(true);
}

fn set_collators_test<T: Config>() {
	vec![get_alice::<T>(), get_bob::<T>()].iter().for_each(|c| {
		Collators::<T>::append(&c.validator);
		Mapping::<T>::insert(&c.nimbus_id, &c.validator);
	})
}

benchmarks! {
	close_pos {

	}:_(SystemOrigin::Root)

	open_pos {

	}:_(SystemOrigin::Root)

	set_collators {
		close_pos_test::<T>();
	}:_(SystemOrigin::Root, vec![get_alice::<T>(), get_bob::<T>()])

	add_collator {
		close_pos_test::<T>();
	}:_(SystemOrigin::Root, get_dave::<T>())

	remove_collator {
		close_pos_test::<T>();
		set_collators_test::<T>();

	}:_(SystemOrigin::Root, account("alice", 1, 1))

	set_nimbus_id {
		close_pos_test::<T>();
		set_collators_test::<T>();
	}:_(SystemOrigin::Signed(account("alice", 1, 1)), NimbusId::generate_pair(None))
}
