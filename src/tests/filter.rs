use crate::operators::of;
use crate::tests::utils::{values_sent, is_completed};

#[test]
fn filter_i32() {
	let obs = of(&[1, 2, 3])
		.filter(|item| item > &1);


	assert!(values_sent(&obs, &[2, 3]));
	assert!(is_completed(&obs));
}

#[test]
fn filter_data() {
	#[derive(PartialEq, Clone)]
	struct DataItem {
		value: i32
	};

	let data = [
		DataItem { value: 1 },
		DataItem { value: 2 },
		DataItem { value: 3 }
	];

	let obs = of(&data)
		.filter(|item: &DataItem| item.value > 1);

	assert!(values_sent(&obs, &[
		DataItem { value: 2 },
		DataItem { value: 3 }
	]));
	assert!(is_completed(&obs));
}