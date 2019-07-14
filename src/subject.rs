use std::cell::RefCell;

use crate::error::RxError;
use crate::subscription::{SubjectSubscription, TrackedSubjectObservers};
use crate::subscriber::{Observer, Subscriber};
use crate::observable::ObservableLike;

#[derive(Default)]
pub struct Subject<T, O> where O: Observer<Value=T, Error=RxError> {
    observers: TrackedSubjectObservers<O>
}

impl<'a, T> Subject<T, Subscriber<T>> {
    pub fn new() -> Subject<T, Subscriber<T>> {
        Subject { observers: RefCell::new(Vec::new()) }
    }

    pub fn subscribe_all<F, E, C>(
        &'a self,
        next_handler: F,
        error_handler:  E,
        complete_handler: C
    ) -> SubjectSubscription<'a, Subscriber<T>>
        where F: Fn(&T) + 'static + Send,
              E: Fn(&RxError) + 'static + Send,
              C: Fn() + 'static + Send {
        // generate a subscriber from the input events
        let subscriber = Subscriber::<T>::new(
            next_handler, error_handler, complete_handler
        );

        self.observers.borrow_mut().push(Some(subscriber));

        SubjectSubscription::new(&self.observers)
    }
}

impl<'a, T: 'static, O: 'a> ObservableLike<'a, O> for Subject<T, O>
    where O: Observer<Value=T, Error=RxError> {
    type Subscription = SubjectSubscription<'a, O>;

    fn subscribe(
        &'a self,
        _observer: O
    ) -> SubjectSubscription<'a, O> {
        unimplemented!()
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
