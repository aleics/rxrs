use std::sync::mpsc::{channel, Sender, Receiver};

use crate::subscription::{ObservableSubscription, Subscription};
use crate::error::RxError;
use crate::subscriber::{Subscriber, Observer};

struct ObservableConstructor<'a, O: Observer> {
    func: Box<dyn Fn(O, Receiver<()>) + 'a>
}

impl<'a, O: Observer> ObservableConstructor<'a, O> {
    pub fn new<F>(func: F) -> ObservableConstructor<'a, O>
        where F: Fn(O, Receiver<()>) + 'a {
        ObservableConstructor { func: Box::new(func) }
    }

    pub fn call(&self, subscriber: O, unsubscriber: Receiver<()>) {
        (self.func)(subscriber, unsubscriber)
    }
}

/// `Observable` is a representation of a collection of values over a period of time. Observables
/// define event streams that can be subscribed to.
pub struct Observable<'a, T, O> where O: Observer<Value=T, Error=RxError> {
    observer_fn: ObservableConstructor<'a, O>
}

impl<'a, T, O> Observable<'a, T, O> where O: Observer<Value=T, Error=RxError> {
    /// Creates a new `Observable` defined by a subscriber function.
    pub fn new<F>(func: F) -> Observable<'a, T, O>
        where F: Fn(O, Receiver<()>) + 'a {
        Observable { observer_fn: ObservableConstructor::new(func) }
    }

    pub fn pipe<U, D, F>(self, func: F) -> Observable<'a, U, D>
        where F: FnOnce(Observable<T, O>) -> Observable<U, D>,
              D: Observer<Value=U, Error=RxError> {
        (func)(self)
    }
}

impl<'a, T> Observable<'a, T, Subscriber<T>> {

    pub fn subscribe_next<N>(&self, next: N) -> ObservableSubscription
        where N: Fn(&T) + 'static + Send {
        self.subscribe_all(next, |_| {}, || {})
    }

    pub fn subscribe_error<E>(&self, error: E) -> ObservableSubscription
        where E: Fn(&RxError) + 'static + Send {
        self.subscribe_all(|_| {}, error, || {})
    }

    pub fn subscribe_complete<C>(&self, complete: C) -> ObservableSubscription
        where C: Fn() + 'static + Send {
        self.subscribe_all(|_| {}, |_| {}, complete)
    }

    pub fn subscribe_all<N, E, C>(&self, next: N, error:  E, complete: C) -> ObservableSubscription
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

impl<'a, T, O> ObservableLike<'a, O> for Observable<'a, T, O>
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
