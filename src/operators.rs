use crate::observable::{ObservableLike, Observable};
use crate::subscription::{ObservableSubscription, Subscription};
use crate::subscriber::{Subscriber, Observer};
use crate::error::RxError;

trait Operator<'a, T, U, O, P, S> where O: ObservableLike<'a, S>, P: Subscription, S: Observer<Value=T, Error=RxError> {
    fn call(&self, source: O, subscriber: Subscriber<U>) -> P;
}

type MapPredicate<T, U> = fn(&T) -> U;

struct ObservableMapOperator<T, U> {
    observable: Observable<T, ObservableMapSubscriber<T, U>>,
    predicate: MapPredicate<T, U>
}

impl<'a, T: 'static, U: 'static>
    Operator<'a, T, U, Observable<T, ObservableMapSubscriber<T, U>>, ObservableSubscription, ObservableMapSubscriber<T, U>>
    for ObservableMapOperator<T, U> {
    fn call(&self, source: Observable<T, ObservableMapSubscriber<T, U>>, destination: Subscriber<U>) -> ObservableSubscription {
        let map_subscriber = ObservableMapSubscriber::<T, U>::new(destination, self.predicate);
        source.subscribe(map_subscriber)
    }
}

struct ObservableMapSubscriber<T, U> {
    destination: Subscriber<U>,
    predicate: MapPredicate<T, U>
}

impl<T, U> ObservableMapSubscriber<T, U> {
    pub fn new(destination: Subscriber<U>, predicate: MapPredicate<T, U>)
        -> ObservableMapSubscriber<T, U> {
        ObservableMapSubscriber { destination, predicate }
    }
}

impl<T, U> Observer for ObservableMapSubscriber<T, U> {
    type Value = T;
    type Error = RxError;

    fn next(&self, value: &Self::Value) {
        let result = (self.predicate)(value);
        self.destination.next(&result);
    }

    fn error(&self, e: &Self::Error) {
        unimplemented!()
    }

    fn complete(&mut self) {
        unimplemented!()
    }
}
