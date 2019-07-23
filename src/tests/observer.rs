use crate::error::RxError;
use crate::observer::{Observer, ObserverLike};

#[test]
fn new() {
	let observer = Observer::<u32>::new(
		|value| println!("{}", value),
		|err| println!("{}", err),
		|| println!("complete")
	);
	assert_eq!(observer.stopped, false);
}

#[test]
fn next() {
	let observer = Observer::<u32>::new(
		|value| assert_eq!(&1, value),
		|_err| assert_eq!(true, false),
		|| assert_eq!(true, false)
	);

	observer.next(&1);
}

#[test]
fn error() {
	let observer = Observer::<u32>::new(
		|_value| assert_eq!(true, false),
		|_err| assert_eq!(true, true),
		|| assert_eq!(true, false)
	);

	observer.error(&RxError::CustomError("some error".to_string()));
}

#[test]
fn complete() {
	let mut observer = Observer::<u32>::new(
		|_value| assert_eq!(true, false),
		|_err| assert_eq!(true, false),
		|| assert_eq!(true, true)
	);

	observer.complete();
	assert_eq!(observer.stopped, true);
}