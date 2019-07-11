use std::thread::{spawn, sleep};
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver};

use crate::subscription::{ObservableSubscription, Subscription};
use crate::error::RxError;
use crate::subscriber::{Subscriber, Observer};

struct ObservableConstructor<O: Observer> {
    func: Box<dyn Fn(O, Receiver<()>)>
}

impl<'a, O: Observer> ObservableConstructor<O> {
    pub fn new<F>(func: F) -> ObservableConstructor<O>
        where F: Fn(O, Receiver<()>) + 'static {
        ObservableConstructor { func: Box::new(func) }
    }

    pub fn call(&self, subscriber: O, unsubscriber: Receiver<()>) {
        (self.func)(subscriber, unsubscriber)
    }
}

/// `Observable` is a representation of a collection of values over a period of time. Observables
/// define event streams that can be subscribed to.
pub struct Observable<T, O> where O: Observer<Value=T, Error=RxError> {
    observer_fn: ObservableConstructor<O>
}

impl<T: 'static> Observable<T, Subscriber<T>> {
    /// Creates a new `Observable` defined by a subscriber function.
    pub fn new<F>(func: F) -> Observable<T, Subscriber<T>>
        where F: Fn(Subscriber<T>, Receiver<()>) + 'static {
        Observable { observer_fn: ObservableConstructor::new(func) }
    }

    /// Subscribes to the event stream of the `Observable` instance. The `Subscriber` function
    /// provided when creating the `Observable` instance is called, and a `Subscription` is created.
    pub fn subscribe_fn<N, E, C>(&self, next: N, error:  E, complete: C) -> ObservableSubscription
        where N: Fn(&T) + 'static + Send,
              E: Fn(&RxError) + 'static + Send,
              C: Fn() + 'static + Send {
        // generate a subscriber from the input events
        let subscriber = Subscriber::<T>::new(
            Box::new(next), Box::new(error),Box::new(complete)
        );

        self.subscribe(subscriber)
    }
}

pub trait ObservableLike<'a, O> {
    type Subscription: Subscription;

    fn subscribe(&'a self, observer: O) -> Self::Subscription;
}

impl<'a, T: 'static, O> ObservableLike<'a, O> for Observable<T, O>
    where O: Observer<Value=T, Error=RxError> {

    type Subscription = ObservableSubscription;

    /// Subscribes to the event stream of the `Observable` instance. The `Subscriber` function
    /// provided when creating the `Observable` instance is called, and a `Subscription` is created.
    fn subscribe(&self, observer: O) -> Self::Subscription {
        // call the observer callback function and include a channel receiver for the
        // a possible unsubscribe action
        let (tx, rx): (Sender<()>, Receiver<()>) = channel();
        self.observer_fn.call(observer, rx);

        // create a subscription and subscribe to the previous callback
        // a channel sender is sent to the subscription so that it can be unsubscribed
        ObservableSubscription::new(tx)
    }
}

/// `of` creates a finite number of observables with a defined value.
/// ```rust
/// use rxrs::observable::{of, ObservableLike};
///
///  of(&[1, 2, 3]).subscribe_fn(
///   |value| println!("{}", value),
///   |error| println!("{}", error),
///   || println!("completed")
/// );
/// ```
pub fn of<T>(values: &'static [T]) -> Observable<T, Subscriber<T>> {
    let observer = move |mut subscriber: Subscriber<T>, _| {
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
/// let mut subscription = interval(1).subscribe_fn(
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
pub fn interval(interval_time: u64) -> Observable<u64, Subscriber<u64>> {
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
