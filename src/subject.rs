use crate::error::RxError;
use crate::subscription::{SubjectSubscription, TrackedSubjectObservers};
use crate::subscriber::{Observer, Subscriber};
use crate::observable::ObservableLike;
use std::cell::RefCell;

pub struct Subject<T> {
    observers: TrackedSubjectObservers<T>
}

impl<'b, T> Subject<T> {
    pub fn new() -> Subject<T> {
        Subject { observers: RefCell::new(Vec::new()) }
    }
}

impl<'a, T: 'static> ObservableLike<'a, T> for Subject<T> {
    type Subscription = SubjectSubscription<'a, T>;

    fn subscribe<F, E, C>(
        &'a self,
        next_handler: F,
        error_handler:  E,
        complete_handler: C
    ) -> SubjectSubscription<'a, T>
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

impl<T> Observer for Subject<T> {
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
