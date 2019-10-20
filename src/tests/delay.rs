use crate::operators::of;
use crate::tests::utils::{values_sent, is_completed};
use std::time::Instant;

#[test]
fn delay_send_all_values() {
	let obs = of(&[1, 2, 3])
		.delay(0);


	assert!(values_sent(&obs, &[1, 2, 3]));
	assert!(is_completed(&obs));
}

#[test]
fn delay_time_on_next() {
	let obs = of(&[1, 2 ,3])
		.delay(200)
		.filter(|value| value == &1); // `subscribe_next` triggers only once

	let start = Instant::now();
	obs.subscribe_next(move |_| {
		let diff = start.elapsed().as_millis();
		assert_eq!(diff, 200);
	});
}

#[test]
fn delay_time_on_complete() {
	let obs = of(&[1, 2 ,3])
		.delay(200);

	let start = Instant::now();
	obs.subscribe_complete(move || {
		let diff = start.elapsed().as_millis();
		assert_eq!(diff, 600);
	});
}