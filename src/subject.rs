use crate::error::RxError;
use crate::subscription::{SubjectSubscription};
use crate::subscriber::{NextHandler, ErrorHandler, CompleteHandler, Subscriber, Observer};
use crate::observable::ObservableLike;
use std::cell::RefCell;

pub struct Subject<T> {
    observers: RefCell<Vec<Subscriber<T>>>
}

impl<'b, T> Subject<T> {
    pub fn new() -> Subject<T> {
        Subject { observers: RefCell::new(Vec::new()) }
    }

    pub fn subscribe<'d>(
        &'b self,
        next_handler: NextHandler<T>,
        error_handler:  ErrorHandler<RxError>,
        complete_handler: CompleteHandler
    ) -> SubjectSubscription<'b, T> {
        // generate a subscriber from the input events
        let subscriber = Subscriber::<T>::new(
            next_handler, error_handler, complete_handler
        );

        self.observers.borrow_mut().push(subscriber);

        SubjectSubscription::new(&self.observers)
    }
}

impl<T> Observer<T> for Subject<T> {
    fn next(&self, value: &T) {
        self.observers.borrow().iter()
            .for_each(|observer| observer.next(value));
    }

    fn error(&self, e: &RxError) {
        self.observers.borrow().iter()
            .for_each(|observer| observer.error(e));
    }

    fn complete(&mut self) -> () {
        self.observers.borrow_mut().iter_mut()
            .for_each(|observer| observer.complete());
    }
}
