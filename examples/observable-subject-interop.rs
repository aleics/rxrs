use std::thread::sleep;
use std::time::Duration;

use rxrs::subject::Subject;
use rxrs::observable::ObservableLike;
use rxrs::subscription::Unsubscribable;
use rxrs::operators::{of, interval};

fn main() {
	let first_subject = Subject::new();
	first_subject.subscribe_all(
		|value| println!("from first_subject observer: {}", value),
		|e| println!("error: {}", e),
		|| println!("complete")
	);

	let of_observable = of(&[1, 2, 3]);
	of_observable.subscribe(first_subject);

	let second_subject = Subject::new();
	second_subject.subscribe_all(
		|value| println!("from second_subject observer: {}", value),
		|e| println!("error: {}", e),
		|| println!("complete")
	);

	let interval_observable = interval(1);
	let mut second_sub = interval_observable.subscribe(second_subject);

	sleep(Duration::from_millis(5));
	second_sub.unsubscribe();
}