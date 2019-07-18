use std::cell::RefCell;

use crate::error::RxError;
use crate::subscription::{SubjectSubscription, TrackedSubjectObservers};
use crate::subscriber::{Observer, Subscriber};
use crate::observable::{ObservableLike, Observable};

#[derive(Default)]
pub struct Subject<T, O> where O: Observer<Value=T, Error=RxError> {
	observers: TrackedSubjectObservers<O>
}

impl<'a, T, O> Subject<T, O> where O: Observer<Value=T, Error=RxError> {
	pub fn new() -> Subject<T, O> {
		Subject { observers: RefCell::new(Vec::new()) }
	}

	pub fn as_observable(&self) -> Observable<T, O> {
		Observable::new(move |destination, _| {
			self.subscribe(destination);
		})
	}
}

impl<'a, T> Subject<T, Subscriber<T>> {

	pub fn subscribe_next<N>(&'a self, next: N) -> SubjectSubscription<'a, Subscriber<T>>
		where N: Fn(&T) + 'static + Send {
		self.subscribe_all(next, |_| {}, || {})
	}

	pub fn subscribe_error<E>(&'a self, error: E) -> SubjectSubscription<'a, Subscriber<T>>
		where E: Fn(&RxError) + 'static + Send {
		self.subscribe_all(|_| {}, error, || {})
	}

	pub fn subscribe_complete<C>(&'a self, complete: C) -> SubjectSubscription<'a, Subscriber<T>>
		where C: Fn() + 'static + Send {
		self.subscribe_all(|_| {}, |_| {}, complete)
	}

	pub fn subscribe_all<F, E, C>(
		&'a self,
		next_handler: F,
		error_handler: E,
		complete_handler: C,
	) -> SubjectSubscription<'a, Subscriber<T>>
		where F: Fn(&T) + 'static + Send,
					E: Fn(&RxError) + 'static + Send,
					C: Fn() + 'static + Send {
		// generate a subscriber from the input events
		let subscriber = Subscriber::<T>::new(
			next_handler, error_handler, complete_handler,
		);

		self.subscribe(subscriber)
	}
}

impl<'a, T, O: 'a> ObservableLike<'a, O> for Subject<T, O>
	where O: Observer<Value=T, Error=RxError> {
	type Subscription = SubjectSubscription<'a, O>;

	fn subscribe(
		&'a self,
		observer: O,
	) -> SubjectSubscription<'a, O> {
		self.observers.borrow_mut().push(Some(observer));
		SubjectSubscription::new(&self.observers)
	}
}

impl<T, O> Observer for Subject<T, O> where O: Observer<Value=T, Error=RxError> {
	type Value = T;
	type Error = RxError;

	fn next(&self, value: &Self::Value) {
		self.observers.borrow().iter()
			.for_each(|item| {
				if let Some(observer) = item {
					observer.next(value);
				}
			});
	}

	fn error(&self, e: &Self::Error) {
		self.observers.borrow().iter()
			.for_each(|item| {
				if let Some(observer) = item {
					observer.error(e);
				}
			});
	}

	fn complete(&mut self) {
		self.observers.borrow_mut().iter_mut()
			.for_each(|item| {
				if let Some(observer) = item {
					observer.complete();
				}
			});
	}
}
