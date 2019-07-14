use crate::subscriber::Observer;
use crate::error::RxError;

pub type FilterPredicate<T> = fn(&T) -> bool;

pub struct FilterSubscriber<T, D> where D: Observer<Value=T, Error=RxError> {
    destination: D,
    predicate: FilterPredicate<T>
}

impl<T, D> FilterSubscriber<T, D> where D: Observer<Value=T, Error=RxError> {
    pub fn new(
        destination: D,
        predicate: FilterPredicate<T>
    ) -> FilterSubscriber<T, D> {
        FilterSubscriber { destination, predicate }
    }
}

impl<T, D> Observer for FilterSubscriber<T, D>
    where D: Observer<Value=T, Error=RxError> {

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
