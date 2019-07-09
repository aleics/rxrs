use std::thread::{spawn, sleep};
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver};

use crate::subscription::{ObservableSubscription, Subscription};
use crate::error::RxError;
use crate::subscriber::{Subscriber, Observer};

struct ObservableConstructor<T> {
    func: Box<dyn Fn(Subscriber<T>, Receiver<()>)>
}

impl<'a, T> ObservableConstructor<T> {
    pub fn new<F>(func: F) -> ObservableConstructor<T>
        where F: Fn(Subscriber<T>, Receiver<()>) + 'static {
        ObservableConstructor { func: Box::new(func) }
    }

    pub fn call(&self, subscriber: Subscriber<T>, unsubscriber: Receiver<()>) {
        (self.func)(subscriber, unsubscriber)
    }
}

/// `Observable` is a representation of a collection of values over a period of time. Observables
/// define event streams that can be subscribed to.
pub struct Observable<T> {
    observer_fn: ObservableConstructor<T>
}

impl<T> Observable<T> {
    /// Creates a new `Observable` defined by a subscriber function.
    pub fn new<F>(func: F) -> Observable<T>
        where F: Fn(Subscriber<T>, Receiver<()>) + 'static {
        Observable { observer_fn: ObservableConstructor::new(func) }
    }
}

pub trait ObservableLike<'a, T> {
    type Subscription: Subscription;

    fn subscribe<N, E, C>(&'a self, next: N, error:  E, complete: C) -> Self::Subscription
        where N: Fn(&T) + 'static + Send,
              E: Fn(&RxError) + 'static + Send,
              C: Fn() + 'static + Send;
}

impl<'a, T: 'static> ObservableLike<'a, T> for Observable<T> {
    type Subscription = ObservableSubscription;

    /// Subscribes to the event stream of the `Observable` instance. The `Subscriber` function
    /// provided when creating the `Observable` instance is called, and a `Subscription` is created.
    fn subscribe<N, E, C>(&self, next: N, error:  E, complete: C) -> Self::Subscription
        where N: Fn(&T) + 'static + Send,
              E: Fn(&RxError) + 'static + Send,
              C: Fn() + 'static + Send {
        // generate a subscriber from the input events
        let subscriber = Subscriber::<T>::new(
             Box::new(next), Box::new(error),Box::new(complete)
        );

        // call the observer callback function and include a channel receiver for the
        // a possible unsubscribe action
        let (tx, rx): (Sender<()>, Receiver<()>) = channel();
        self.observer_fn.call(subscriber, rx);

        // create a subscription and subscribe to the previous callback
        // a channel sender is sent to the subscription so that it can be unsubscribed
        ObservableSubscription::new(tx)
    }
}

/// `of` creates a finite number of observables with a defined value.
/// ```rust
/// use rxrs::observable::{of, ObservableLike};
///
///  of(&[1, 2, 3]).subscribe(
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
/// use rxrs::observable::{interval, ObservableLike};
/// use std::thread;
/// use std::time::Duration;
/// use rxrs::subscription::Subscription;
///
///
/// let mut subscription = interval(1).subscribe(
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
