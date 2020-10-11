use std::thread::sleep;
use std::time::Duration;

use crate::error::RxError;
use crate::observer::ObserverLike;

pub struct DelayObserver<T, D> where D: ObserverLike<Value=T, Error=RxError> {
    delay: u64,
    destination: D,
}

impl<T, D> DelayObserver<T, D> where D: ObserverLike<Value=T, Error=RxError> {
    pub fn new(destination: D, delay: u64) -> DelayObserver<T, D> {
        DelayObserver { destination, delay }
    }
}

impl<T, D> ObserverLike for DelayObserver<T, D>
    where D: ObserverLike<Value=T, Error=RxError> {
    type Value = T;
    type Error = RxError;

    fn next(&self, value: &Self::Value) {
        sleep(Duration::from_millis(self.delay));
        self.destination.next(value)
    }

    fn error(&self, e: &Self::Error) {
        self.destination.error(e);
    }

    fn complete(&mut self) {
        self.destination.complete();
    }
}
