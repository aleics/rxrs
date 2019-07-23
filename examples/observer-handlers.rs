use rxrs::observable::{Observable, ObservableLike, Unsubscriber};
use rxrs::observer::{ObserverLike, Observer};

fn main() {
	let subscriber_fn = |mut observer: Observer<String>| {
		observer.next(&String::from("Oh"));
		observer.next(&String::from("hi"));
		observer.next(&String::from("Mark!"));
		observer.complete();

		observer.next(&String::from("This should not be printed."));

		Unsubscriber::new(|| {})
	};

	Observable::new(subscriber_fn).subscribe(Observer::new(
		|value| println!("{}", value),
		|error| println!("{}", error),
		|| println!("completed")
	));
}