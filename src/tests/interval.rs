use crate::operators::interval;
use std::thread::sleep;
use std::time::Duration;
use crate::tests::utils::{values_sent, is_completed};

#[test]
fn create() {
	let obs = interval(1);

	sleep(Duration::from_millis(2));
	assert!(values_sent(&obs, &[0, 1, 2]));
	assert_eq!(is_completed(&obs), false);
}