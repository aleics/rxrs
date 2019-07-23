use crate::subscription::{Subscription, Unsubscribable};
use crate::error::RxError;
use crate::observer::{Observer, ObserverLike};
use crate::operators::map::{MapPredicate, MapObserver};
use crate::operators::filter::{FilterObserver, FilterPredicate};

pub struct Unsubscriber {
	func: Box<dyn FnMut() + Send>
}

impl Unsubscriber {
	pub fn new<F: 'static>(func: F) -> Unsubscriber where F: FnMut() + Send {
		Unsubscriber { func: Box::new(func) }
	}

	pub fn call(&mut self) {
		(self.func)();
	}
}

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

	pub fn map(self, predicate: MapPredicate<T, U>) -> Observable<'a, U, D>
		where D: ObserverLike<Value=U, Error=RxError> {
		Observable::new( move |destination: D| {
			let map_observer = MapObserver::new(destination, predicate);
			let mut subscription = self.subscribe(map_observer);

			Unsubscriber::new(move || subscription.unsubscribe())
		})
	}
}

impl<'a, T: 'a, O: 'a> Observable<'a, T, FilterObserver<T, O>>
	where O: ObserverLike<Value=T, Error=RxError> {

	pub fn filter(self, predicate: FilterPredicate<T>) -> Observable<'a, T, O> {
		Observable::new( move |destination: O| {
			let filter_observer = FilterObserver::new(destination, predicate);
			let mut subscription = self.subscribe(filter_observer);

			Unsubscriber::new(move || subscription.unsubscribe())
		})
	}
}

impl<'a, T> Observable<'a, T, Observer<T>> {

	pub fn subscribe_next<N>(&self, next: N) -> Subscription
		where N: Fn(&T) + 'static + Send {
		self.subscribe_all(next, |_| {}, || {})
	}

	pub fn subscribe_error<E>(&self, error: E) -> Subscription
		where E: Fn(&RxError) + 'static + Send {
		self.subscribe_all(|_| {}, error, || {})
	}

	pub fn subscribe_complete<C>(&self, complete: C) -> Subscription
		where C: Fn() + 'static + Send {
		self.subscribe_all(|_| {}, |_| {}, complete)
	}

	pub fn subscribe_all<N, E, C>(&self, next: N, error:  E, complete: C) -> Subscription
		where N: Fn(&T) + 'static + Send,
					E: Fn(&RxError) + 'static + Send,
					C: Fn() + 'static + Send {
		// generate a subscriber from the input events
		let subscriber = Observer::<T>::new(
			Box::new(next), Box::new(error),Box::new(complete)
		);

		self.subscribe(subscriber)
	}
}

pub trait ObservableLike<'a> {
	type Observer: ObserverLike;
	type Subscription: Unsubscribable;

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
