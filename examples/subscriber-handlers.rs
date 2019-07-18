use rxrs::observable::{Observable, ObservableLike};
use rxrs::subscriber::{Observer, Subscriber};

fn main() {
	let subscriber_fn = |mut subscriber: Subscriber<String>, _| {
		subscriber.next(&String::from("Oh"));
		subscriber.next(&String::from("hi"));
		subscriber.next(&String::from("Mark!"));
		subscriber.complete();

		subscriber.next(&String::from("This should not be printed."));
	};

	Observable::new(subscriber_fn).subscribe(Subscriber::new(
		|value| println!("{}", value),
		|error| println!("{}", error),
		|| println!("completed")
	));
}