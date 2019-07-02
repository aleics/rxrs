use crate::error::RxError;
use crate::subscription::{SubjectSubscription};
use crate::subscriber::{NextHandler, ErrorHandler, CompleteHandler, Subscriber, Observer};
use crate::observable::ObservableLike;
use std::cell::RefCell;

pub struct Subject<T> {
    observers: RefCell<Vec<Subscriber<T>>>
}

impl<T> Subject<T> {
    pub fn new() -> Subject<T> {
        Subject { observers: RefCell::new(Vec::new()) }
    }
}

impl<T> ObservableLike<T> for Subject<T> {
    type Subscription = SubjectSubscription;

    fn subscribe(
        &self,
        next_handler: NextHandler<T>,
        error_handler:  ErrorHandler<RxError>,
        complete_handler: CompleteHandler
    ) -> SubjectSubscription {
        // generate a subscriber from the input events
        let subscriber = Subscriber::<T>::new(
            next_handler, error_handler, complete_handler
        );

        self.observers.borrow_mut().push(subscriber);

        SubjectSubscription::new()
    }
}

impl<T> Observer<T> for Subject<T> {
    fn next(&self, value: &T) {
        for observer in self.observers.borrow().iter() {
            observer.next(value);
        }
    }

    fn error(&self, e: &RxError) {
        for observer in self.observers.borrow().iter() {
            observer.error(e);
        }
    }

    fn complete(&mut self) -> () {
        for observer in self.observers.borrow_mut().iter_mut() {
            observer.complete();
        }
    }
}
