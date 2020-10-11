use crate::error::RxError;
use crate::observer::ObserverLike;

pub type MapPredicate<T, U> = fn(&T) -> U;

pub struct MapObserver<T, U, D> where D: ObserverLike<Value=U, Error=RxError> {
    destination: D,
    predicate: MapPredicate<T, U>,
}

impl<T, U, D> MapObserver<T, U, D> where D: ObserverLike<Value=U, Error=RxError> {
    pub fn new(
		destination: D,
		predicate: MapPredicate<T, U>,
	) -> MapObserver<T, U, D> {
        MapObserver { destination, predicate }
    }
}

impl<T, U, D> ObserverLike for MapObserver<T, U, D>
    where D: ObserverLike<Value=U, Error=RxError> {
    type Value = T;
    type Error = RxError;

    fn next(&self, value: &Self::Value) {
        let result = (self.predicate)(value);
        self.destination.next(&result);
    }

    fn error(&self, e: &Self::Error) {
        self.destination.error(e);
    }

    fn complete(&mut self) {
        self.destination.complete();
    }
}
