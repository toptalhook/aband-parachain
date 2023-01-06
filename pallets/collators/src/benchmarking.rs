#![allow(unused_imports)]

use super::*;
use crate::{Config, Pallet};
use frame_benchmarking::{
	account, benchmarks, benchmarks_instance, impl_benchmark_test_suite, whitelisted_caller,
};
use frame_system::RawOrigin as SystemOrigin;
use sp_std::vec;

fn get_alice<T: Config>() -> T::AccountId {
	let alice = account("alice", 1, 1);
	alice
}

fn close_pos_test<T: Config>() {
	IsClosedPoS::<T>::put(true);
}

fn set_collators_test<T: Config>() {
	Collators::<T>::put(vec![get_alice::<T>(), get_bob::<T>()])
}

fn get_bob<T: Config>() -> T::AccountId {
	let bob = account("bob", 1, 1);
	bob
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
	}:_(SystemOrigin::Root, get_alice::<T>())

	remove_collator {
		close_pos_test::<T>();
		set_collators_test::<T>();

	}:_(SystemOrigin::Root, get_alice::<T>())

}
