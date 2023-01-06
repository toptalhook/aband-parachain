#![allow(unused_imports)]
#![cfg(test)]
use super::*;
use crate::mock::{RuntimeCall, RuntimeOrigin, *};
use frame_support::{assert_noop, assert_ok, debug, log::debug,};
use crate::mock::RuntimeEvent::Collators;

fn close_pos_test() {
	Pallet::<Test>::close_pos(RuntimeOrigin::root(), ).unwrap();
}

fn set_collators_test() {
	assert!(Pallet::<Test>::set_collators(RuntimeOrigin::root(), vec![1, 2, 3]).is_err());
	close_pos_test();
	assert_ok!(Pallet::<Test>::set_collators(RuntimeOrigin::root(), vec![1, 2, 3]));
	assert!(Pallet::<Test>::get_collators().len() == 3);
}

#[test]
fn close_pos_should_work() {
	new_test_ext().execute_with(|| {
		close_pos_test();
		assert!(IsClosedPoS::<Test>::get() == true);
	});
}

#[test]
fn open_pos_should_work() {
	new_test_ext().execute_with(|| {
		close_pos_test();
		Pallet::<Test>::open_pos(RuntimeOrigin::root(),);
		assert!(IsClosedPoS::<Test>::get() == false);
	});
}

#[test]
fn set_collators_should() {
	new_test_ext().execute_with(|| {
		set_collators_test();

	});
}

#[test]
fn add_collators_should_work() {
	new_test_ext().execute_with(|| {
		close_pos_test();
		assert_ok!(Pallet::<Test>::add_collator(RuntimeOrigin::root(), 1));
		// fixme
		// assert!(Pallet::<Test>::get_collators().len() == 1);
	});
}

#[test]
fn remove_collators() {
	new_test_ext().execute_with(|| {
		set_collators_test();
		Pallet::<Test>::remove_collator(RuntimeOrigin::root(), 1).unwrap();
		assert!(Pallet::<Test>::get_collators().len() == 2);
	});
}


