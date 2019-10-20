use std::thread::sleep;
use std::time::Duration;

use rxrs::operators::interval;
use rxrs::subscription::Unsubscribable;

fn main() {
	let obs = interval(1);
	let mut sub = obs.subscribe_next(|item| { println!("{}", item) });

	sleep(Duration::from_millis(5));
	sub.unsubscribe();

	// Let's wait if some values come after 5 ms (it shouldn't print more than 5)
	sleep(Duration::from_millis(10));
}