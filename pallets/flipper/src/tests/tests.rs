use super::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};

#[test]
fn set_value_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(FlipperModule::set_value(Origin::signed(1), true));
		assert_eq!(FlipperModule::value(), Some(true));
	});
}

#[test]
fn set_value_err_already_set() {
	new_test_ext().execute_with(|| {
		assert_ok!(FlipperModule::set_value(Origin::signed(1), true));
		assert_noop!(
			FlipperModule::set_value(Origin::signed(1), true),
			Error::<Test>::AlreadySet
		);
	});
}

#[test]
fn flip_value_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(FlipperModule::set_value(Origin::signed(1), true));
		assert_ok!(FlipperModule::flip_value(Origin::signed(1)));

		assert_eq!(FlipperModule::value(), Some(false))
	})
}

#[test]
fn flip_value_no_value() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			FlipperModule::flip_value(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	})
}
