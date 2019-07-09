use crate::error::RxError;
use crate::subscription::{SubjectSubscription, TrackedSubjectObservers};
use crate::subscriber::{NextHandler, ErrorHandler, CompleteHandler, Subscriber, Observer};
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

impl<'a, T: 'a> ObservableLike<'a, T> for Subject<T> {
    type Subscription = SubjectSubscription<'a, T>;

    fn subscribe(
        &'a self,
        next_handler: NextHandler<T>,
        error_handler:  ErrorHandler<RxError>,
        complete_handler: CompleteHandler
    ) -> SubjectSubscription<'a, T> {
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
