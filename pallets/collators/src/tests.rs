#![allow(unused_imports)]
#![cfg(test)]
use super::*;
use crate::mock::{RuntimeCall, RuntimeEvent::Collators, RuntimeOrigin, *};
use frame_support::{assert_noop, assert_ok, debug, log::debug};
use nimbus_primitives::NimbusPair;
use sp_core::{Pair, U256};

fn close_pos_test() {
	assert_ok!(Pallet::<Test>::close_pos(RuntimeOrigin::root()));
}

fn open_pos_test() {
	assert_ok!(Pallet::<Test>::open_pos(RuntimeOrigin::root()));
}

fn set_collators_test() {
	assert!(Pallet::<Test>::set_collators(
		RuntimeOrigin::root(),
		vec![
			CollatorInfo {
				validator: 1,
				nimbus_id: NimbusPair::from_seed(&U256::from(1).into()).public(),
			},
			CollatorInfo {
				validator: 2,
				nimbus_id: NimbusPair::from_seed(&U256::from(2).into()).public(),
			},
			CollatorInfo {
				validator: 3,
				nimbus_id: NimbusPair::from_seed(&U256::from(3).into()).public(),
			}
		]
	)
	.is_err());
	assert!(Pallet::<Test>::set_collators(RuntimeOrigin::root(), vec![]).is_err());
	close_pos_test();
	assert_ok!(Pallet::<Test>::set_collators(
		RuntimeOrigin::root(),
		vec![
			CollatorInfo {
				validator: 1,
				nimbus_id: NimbusPair::from_seed(&U256::from(1).into()).public(),
			},
			CollatorInfo {
				validator: 2,
				nimbus_id: NimbusPair::from_seed(&U256::from(2).into()).public(),
			},
			CollatorInfo {
				validator: 3,
				nimbus_id: NimbusPair::from_seed(&U256::from(3).into()).public(),
			}
		]
	));
	assert!(Pallet::<Test>::get_collators().len() == 3);
}

fn add_collator_test() {
	assert!(Pallet::<Test>::get_collators().len() == 1);
	assert!(Pallet::<Test>::add_collator(
		RuntimeOrigin::root(),
		CollatorInfo {
			validator: 2,
			nimbus_id: NimbusPair::from_seed(&U256::from(2).into()).public(),
		}
	)
	.is_err());
	close_pos_test();
	assert!(Pallet::<Test>::add_collator(
		RuntimeOrigin::root(),
		CollatorInfo {
			validator: 2,
			nimbus_id: NimbusPair::from_seed(&U256::from(1).into()).public(),
		}
	)
	.is_err());
	assert!(Pallet::<Test>::add_collator(
		RuntimeOrigin::root(),
		CollatorInfo {
			validator: 1,
			nimbus_id: NimbusPair::from_seed(&U256::from(1).into()).public(),
		}
	)
	.is_err());
	assert_ok!(Pallet::<Test>::add_collator(
		RuntimeOrigin::root(),
		CollatorInfo {
			validator: 2,
			nimbus_id: NimbusPair::from_seed(&U256::from(2).into()).public(),
		}
	));
	assert!(Pallet::<Test>::get_collators().len() == 2);
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
		assert_ok!(Pallet::<Test>::open_pos(RuntimeOrigin::root()));
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
		add_collator_test();
	});
}

#[test]
fn remove_collators() {
	new_test_ext().execute_with(|| {
		assert!(Pallet::<Test>::remove_collator(RuntimeOrigin::root(), 1).is_err());
		close_pos_test();
		assert!(Pallet::<Test>::remove_collator(RuntimeOrigin::root(), 1).is_err());
		open_pos_test();
		set_collators_test();
		assert_ok!(Pallet::<Test>::remove_collator(RuntimeOrigin::root(), 1));
		assert!(Pallet::<Test>::get_collators().len() == 2);
	});
}

#[test]
fn set_nimbus_id_should_work() {
	new_test_ext().execute_with(|| {
		assert!(Pallet::<Test>::set_nimbus_id(
			RuntimeOrigin::signed(1),
			NimbusPair::from_seed(&U256::from(1).into()).public()
		)
		.is_err());
		close_pos_test();
		assert!(Pallet::<Test>::set_nimbus_id(
			RuntimeOrigin::signed(1),
			NimbusPair::from_seed(&U256::from(1).into()).public()
		)
		.is_err());
		assert_ok!(Pallet::<Test>::set_nimbus_id(
			RuntimeOrigin::signed(1),
			NimbusPair::from_seed(&U256::from(2).into()).public()
		));
	});
}
