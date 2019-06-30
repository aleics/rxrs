use std::thread::{spawn, sleep};
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver};

use crate::subscription::Subscription;
use crate::error::RxError;
use crate::subscriber::{Subscriber, Observer, NextHandler, ErrorHandler, CompleteHandler, SubscriberFn};

/// `Observable` is a representation of a collection of values over a period of time. Observables
/// define event streams that can be subscribed to.
pub struct Observable<T> {
    observer: SubscriberFn<T>
}

impl<T> Observable<T> {
    /// Creates a new `Observable` defined by a subscriber function.
    pub fn new(observer: SubscriberFn<T>) -> Observable<T> {
        Observable { observer }
    }
}

pub trait ObservableLike<T> {
    fn subscribe(
        &self,
        next_handler: NextHandler<T>,
        error_handler:  ErrorHandler<RxError>,
        complete_handler: CompleteHandler
    ) -> Subscription;
}

impl<T> ObservableLike<T> for Observable<T> {
    /// Subscribes to the event stream of the `Observable` instance. The `Subscriber` function
    /// provided when creating the `Observable` instance is called, and a `Subscription` is created.
    fn subscribe(
        &self,
        next_handler: NextHandler<T>,
        error_handler:  ErrorHandler<RxError>,
        complete_handler: CompleteHandler
    ) -> Subscription {
        // generate a subscriber from the input events
        let subscriber = Subscriber::<T>::new(
            next_handler, error_handler, complete_handler
        );

        // call the observer callback function and include a channel receiver for the
        // a possible unsubscribe action
        let (tx, rx): (Sender<()>, Receiver<()>) = channel();
        (self.observer)(subscriber, rx);

        // create a subscription and subscribe to the previous callback
        // a channel sender is sent to the subscription so that it can be unsubscribed
        let mut subscription = Subscription::new();
        subscription.subscribe(tx);

        subscription
    }
}

/// `of` creates a finite number of observables with a defined value.
/// ```rust
/// use rxrs::observable::{of, ObservableLike};
///
/// let observable = of(&[1, 2, 3]);
/// observable.subscribe(
///   |value| println!("{}", value),
///   |error| println!("{}", error),
///   || println!("completed")
/// );
/// ```
pub fn of<T>(values: &'static [T]) -> Observable<T> {
    let observer = move |mut subscriber: Subscriber<T>, _: Receiver<()>| {
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
/// use rxrs::observable::interval;
/// use std::thread;
/// use std::time::Duration;
///
/// let observable = interval(1);
///
/// let mut subscription = observable.subscribe(
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
pub fn interval(interval_time: u64) -> Observable<u64> {
    let observer = move |subscriber: Subscriber<u64>, unsubscriber: Receiver<()>| {
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
