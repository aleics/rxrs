use crate::error::RxError;
use crate::observer::{Observer, ObserverLike};
use crate::operators::delay::DelayObserver;
use crate::operators::filter::{FilterObserver, FilterPredicate};
use crate::operators::map::{MapObserver, MapPredicate};
use crate::subscription::{Subscription, Unsubscribable};

/// `Unsubscriber` is a container for the function that should be called, once an `Observable`
/// is unsubscribed. The `Unsubscriber` instance of an `Obseervable` is created once the
/// observable has been subscribed.
pub struct Unsubscriber {
    func: Box<dyn FnMut()>
}

impl Unsubscriber {
    pub fn new<F: 'static>(func: F) -> Unsubscriber where F: FnMut() {
        Unsubscriber { func: Box::new(func) }
    }

    pub fn call(&mut self) {
        (self.func)();
    }
}

/// `ObservableConstructor` is a container for the logic of the Observable's creation.
/// This function excepts as a parameter an `ObserverLike` variable and returns an `Unsubscriber`.
struct ObservableConstructor<'a, O: ObserverLike> {
    func: Box<dyn Fn(O) -> Unsubscriber + 'a>
}

impl<'a, O: ObserverLike> ObservableConstructor<'a, O> {
    pub fn new<F>(func: F) -> ObservableConstructor<'a, O>
        where F: Fn(O) -> Unsubscriber + 'a {
        ObservableConstructor { func: Box::new(func) }
    }

    pub fn call(&self, observer: O) -> Unsubscriber {
        (self.func)(observer)
    }
}

/// `Observable` is a representation of a collection of values over a period of time. Observables
/// define event streams that can be subscribed to.
pub struct Observable<'a, T, O> where O: ObserverLike<Value=T, Error=RxError> {
    observer_fn: ObservableConstructor<'a, O>
}

impl<'a, T, O> Observable<'a, T, O> where O: ObserverLike<Value=T, Error=RxError> {
    /// Creates a new `Observable` defined by a subscriber function.
    pub fn new<F>(func: F) -> Observable<'a, T, O>
        where F: Fn(O) -> Unsubscriber + 'a {
        Observable { observer_fn: ObservableConstructor::new(func) }
    }
}

impl<'a, T: 'a, U: 'a, D> Observable<'a, T, MapObserver<T, U, D>>
    where D: ObserverLike<Value=U, Error=RxError> + 'a {
    /// Maps the instance of an `Observable` into a new instance by mapping its internal value.
    /// This mapping is defined by the `predicate` input parameter.
    ///
    /// ```rust
    /// use rxrs::operators::of;
    /// use rxrs::observable::ObservableLike;
    ///
    /// let obs = of(&[1, 2, 3])
    /// 	.map(|item| item.to_string());
    ///
    /// obs.subscribe_next(|string| {
    /// 	println!("{}", string);
    /// });
    /// ```
    pub fn map(self, predicate: MapPredicate<T, U>) -> Observable<'a, U, D>
        where D: ObserverLike<Value=U, Error=RxError> {
        Observable::new(move |destination: D| {
            let map_observer = MapObserver::new(destination, predicate);
            let mut subscription = self.subscribe(map_observer);

            Unsubscriber::new(move || subscription.unsubscribe())
        })
    }
}

impl<'a, T: 'a, O: 'a> Observable<'a, T, FilterObserver<T, O>>
    where O: ObserverLike<Value=T, Error=RxError> {
    /// Filters the stream of values of an `Observable` and returns a new instance with the same
    /// type definition. The filtering strategy is defined by the `predicate` input variable.
    ///
    /// ```rust
    /// use rxrs::operators::of;
    ///
    /// let even = of(&[1, 2, 3])
    /// 	.filter(|item| item % 2 == 0);
    ///
    /// even.subscribe_next(|number| println!("{}", number));
    ///
    /// let odds = of(&[1, 2, 3])
    /// 	.filter(|item| item % 2 == 1);
    ///
    /// odds.subscribe_next(|number| println!("{}", number));
    /// ```
    pub fn filter(self, predicate: FilterPredicate<T>) -> Observable<'a, T, O> {
        Observable::new(move |destination: O| {
            let filter_observer = FilterObserver::new(destination, predicate);
            let mut subscription = self.subscribe(filter_observer);

            Unsubscriber::new(move || subscription.unsubscribe())
        })
    }
}

impl<'a, T: 'a, O: 'a> Observable<'a, T, DelayObserver<T, O>>
    where O: ObserverLike<Value=T, Error=RxError> {
    /// Delays the `Observable` stream items by a `value` amount of time (in ms);
    ///
    /// ```rust
    /// use std::time::Instant;
    /// use rxrs::operators::of;
    ///
    /// let delayed = of(&[1, 2, 3])
    /// 	.delay(10);
    ///
    /// let start = Instant::now();
    /// delayed.subscribe_next(move |_| {
    /// 	let diff = start.elapsed().as_millis();
    /// 	println!("delay: {}", diff);
    /// });
    /// ```
    pub fn delay(self, value: u64) -> Observable<'a, T, O> {
        Observable::new(move |destination: O| {
            let delay_observer = DelayObserver::new(destination, value);
            let mut subscription = self.subscribe(delay_observer);

            Unsubscriber::new(move || subscription.unsubscribe())
        })
    }
}

impl<'a, T> Observable<'a, T, Observer<T>> {
    /// `subscribe_next` subscribes to the event stream of next values from an `Observable` instance.
    pub fn subscribe_next<N>(&self, next: N) -> Subscription
        where N: Fn(&T) + 'static + Send {
        self.subscribe_all(next, |_| {}, || {})
    }

    /// `subscribe_error` subscribes to the event stream of errors from an `Observable` instance.
    pub fn subscribe_error<E>(&self, error: E) -> Subscription
        where E: Fn(&RxError) + 'static + Send {
        self.subscribe_all(|_| {}, error, || {})
    }

    /// `subscribe_complete` subscribes to the complete event from an `Observable` instance.
    pub fn subscribe_complete<C>(&self, complete: C) -> Subscription
        where C: Fn() + 'static + Send {
        self.subscribe_all(|_| {}, |_| {}, complete)
    }

    /// `subscribe_all` subscribes to the event stream of an `Observable` instance, and provides
    /// a `next`, `error` and `complete` function handlers for the different type of events.
    pub fn subscribe_all<N, E, C>(&self, next: N, error: E, complete: C) -> Subscription
        where N: Fn(&T) + 'static + Send,
              E: Fn(&RxError) + 'static + Send,
              C: Fn() + 'static + Send {
        // generate a subscriber from the input events
        let subscriber = Observer::<T>::new(
            Box::new(next), Box::new(error), Box::new(complete),
        );

        self.subscribe(subscriber)
    }
}

/// `ObservableLike` is a trait definition for data structures that implement the `Observable`
/// logic, and thus, can be subscribed to. Logically, `Observable` is an `ObservableLike` struct,
/// but also `Subject` can be also subscribed to.
pub trait ObservableLike<'a> {
    type Observer: ObserverLike;
    type Subscription: Unsubscribable;

    /// `subscribe` executes the `Observable` instance and returns the generated event values to
    /// the `Observer` input parameter. A `Subscription` instance is returned, that can be
    /// unsubscribed, and no further events would be listened to.
    fn subscribe(&'a self, observer: Self::Observer) -> Self::Subscription;
}

impl<'a, T, O> ObservableLike<'a> for Observable<'a, T, O>
    where O: ObserverLike<Value=T, Error=RxError> {
    type Observer = O;
    type Subscription = Subscription;

    /// Subscribes to the event stream of the `Observable` instance. The `Subscriber` function
    /// provided when creating the `Observable` instance is called, and a `Subscription` is created.
    fn subscribe(&self, observer: Self::Observer) -> Self::Subscription {
        let unsubscriber = self.observer_fn.call(observer);
        Subscription::new(unsubscriber)
    }
}
