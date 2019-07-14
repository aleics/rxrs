use std::thread::{spawn, sleep};
use std::time::Duration;
use std::sync::mpsc::Receiver;

use crate::observable::{Observable, ObservableLike};
use crate::subscriber::Observer;
use crate::error::RxError;
use crate::operators::map::{MapPredicate, MapSubscriber};

mod map;

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
pub fn of<T, O: Observer<Value=T, Error=RxError>>(values: &'static [T]) -> Observable<T, O> {
    let observer = move |mut subscriber: O, _| {
        for value in values {
            subscriber.next(value);
        }
        subscriber.complete();
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
pub fn interval<O>(interval_time: u64) -> Observable<u64, O>
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
    };
    Observable::new(Box::new(observer))
}


pub fn map<T: 'static, U: 'static, D: 'static>(predicate: MapPredicate<T, U>)
    -> impl FnOnce(Observable<T, MapSubscriber<T, U, D>>) -> Observable<U, D>
    where D: Observer<Value=U, Error=RxError> {

    move |upstream| {
        Observable::new( move |destination: D, _| {
            let map_subscriber = MapSubscriber::new(destination, predicate);
            upstream.subscribe(map_subscriber);
        })
    }
}