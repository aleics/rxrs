use crate::observer::ObserverLike;
use crate::error::RxError;

pub type FilterPredicate<T> = fn(&T) -> bool;

pub struct FilterObserver<T, D> where D: ObserverLike<Value=T, Error=RxError> {
	destination: D,
	predicate: FilterPredicate<T>
}

impl<T, D> FilterObserver<T, D> where D: ObserverLike<Value=T, Error=RxError> {
	pub fn new(
		destination: D,
		predicate: FilterPredicate<T>
	) -> FilterObserver<T, D> {
		FilterObserver { destination, predicate }
	}
}

impl<T, D> ObserverLike for FilterObserver<T, D>
	where D: ObserverLike<Value=T, Error=RxError> {

	type Value = T;
	type Error = RxError;

	fn next(&self, value: &Self::Value) {
		if (self.predicate)(value) {
			self.destination.next(value);
		}
	}

	fn error(&self, e: &Self::Error) {
		self.destination.error(e);
	}

	fn complete(&mut self) {
		self.destination.complete();
	}
}
