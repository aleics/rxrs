use crate::subscriber::Observer;
use crate::error::RxError;

pub type MapPredicate<T, U> = fn(&T) -> U;

pub struct MapSubscriber<T, U, D> where D: Observer<Value=U, Error=RxError> {
    destination: D,
    predicate: MapPredicate<T, U>
}

impl<T, U, D> MapSubscriber<T, U, D> where D: Observer<Value=U, Error=RxError> {
    pub fn new(destination: D, predicate: MapPredicate<T, U>)
               -> MapSubscriber<T, U, D> {
        MapSubscriber { destination, predicate }
    }
}

impl<T, U, D> Observer for MapSubscriber<T, U, D>
    where D: Observer<Value=U, Error=RxError> {

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
