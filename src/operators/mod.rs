use std::thread::{spawn, sleep};
use std::time::Duration;
use std::sync::mpsc::channel;

use crate::observable::{Observable, Unsubscriber};
use crate::observer::{ObserverLike};
use crate::error::RxError;

pub(crate) mod delay;
pub(crate) mod filter;
pub(crate) mod map;

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
	where O: ObserverLike<Value=T, Error=RxError> {
	Observable::new(move |mut subscriber: O| {
		for value in values {
			subscriber.next(value);
		}
		subscriber.complete();

		Unsubscriber::new(|| {})
	})
}

/// `interval` creates an infinite observable that emits sequential numbers every specified
/// interval of time.
/// ```rust
/// use std::thread;
/// use std::time::Duration;
/// use rxrs::observable::ObservableLike;
/// use rxrs::subscription::Unsubscribable;
/// use rxrs::operators::interval;
///
///
/// let mut subscription = interval(1).subscribe_all(
///   |value| println!("{}", value),
///   |error| println!("{}", error),
///   || println!("completed")
/// );
///
/// thread::sleep(Duration::from_millis(5));
/// subscription.unsubscribe();
/// ```
pub fn interval<'a, O>(interval_time: u64) -> Observable<'a, u64, O>
	where O: ObserverLike<Value=u64, Error=RxError> + Send + 'static {
	let observer = move |subscriber: O| {
		let (tx, rx) = channel();
		spawn(move || {
			let mut count = 0;

			loop {
				sleep(Duration::from_millis(interval_time));
				subscriber.next(&count);

				count += 1;

				if rx.try_recv().is_ok() {
					break;
				}
			}
		});

		Unsubscriber::new(move || {
			tx.send(()).unwrap();
		})
	};
	Observable::new(Box::new(observer))
}