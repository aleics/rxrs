use std::thread::{spawn, sleep};
use std::time::Duration;
use std::sync::mpsc::Receiver;

use crate::observable::{Observable, Unsubscriber};
use crate::subscriber::Observer;
use crate::error::RxError;

pub(crate) mod map;
pub(crate) mod filter;

/// `of` creates a finite number of observables with a defined value.
/// ```rust
/// use rxrs::observable::ObservableLike;
/// use rxrs::operators::of;
///
///  of(&[1, 2, 3]).subscribe_all(
///   |value| println!("{}", value),
///   |error| println!("{}", error),
///   || println!("completed")
/// );
/// ```
pub fn of<T, O>(values: &[T]) -> Observable<T, O>
	where O: Observer<Value=T, Error=RxError> {
	let observer = move |mut subscriber: O, _| {
		for value in values {
			subscriber.next(value);
		}
		subscriber.complete();

		Unsubscriber::new(|| {})
	};
	Observable::new(Box::new(observer))
}

/// `interval` creates an infinite observable that emits sequential numbers every specified
/// interval of time.
/// ```rust
/// use std::thread;
/// use std::time::Duration;
/// use rxrs::observable::ObservableLike;
/// use rxrs::subscription::Subscription;
/// use rxrs::operators::interval;
///
///
/// let mut subscription = interval(1).subscribe_all(
///   |value| println!("{}", value),
///   |error| println!("{}", error),
///   || println!("completed")
/// );
///
/// let j = thread::spawn(move || {
///   thread::sleep(Duration::from_millis(5));
///   subscription.unsubscribe();
/// });
///
/// j.join().unwrap();
/// ```
pub fn interval<'a, O>(interval_time: u64) -> Observable<'a, u64, O>
	where O: Observer<Value=u64, Error=RxError> + Send + 'static {
	let observer = move |subscriber: O, unsubscriber: Receiver<()>| {
		spawn(move || {
			let mut count = 0;

			loop {
				sleep(Duration::from_millis(interval_time));
				subscriber.next(&count);

				count += 1;

				if unsubscriber.try_recv().is_ok() {
					break;
				}
			}
		});

		Unsubscriber::new(|| {})
	};
	Observable::new(Box::new(observer))
}