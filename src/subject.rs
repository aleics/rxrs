use crate::error::RxError;
use crate::subscription::Subscription;
use crate::subscriber::{NextHandler, ErrorHandler, CompleteHandler, Subscriber, Observer};
use crate::observable::ObservableLike;

pub struct Subject<T> {
    observers: Vec<Subscriber<T>>
}

impl<T> Subject<T> {
    pub fn new() -> Subject<T> {
        Subject { observers: Vec::new() }
    }
}

impl<T> ObservableLike<T> for Subject<T> {
    fn subscribe(
        &mut self,
        next_handler: NextHandler<T>,
        error_handler:  ErrorHandler<RxError>,
        complete_handler: CompleteHandler
    ) -> Subscription {
        // generate a subscriber from the input events
        let subscriber = Subscriber::<T>::new(
            next_handler, error_handler, complete_handler
        );

        self.observers.push(subscriber);

        Subscription::new()
    }
}

impl<T> Observer<T> for Subject<T> {
    fn next(&self, value: &T) {
        for observer in &self.observers {
            observer.next(value);
        }
    }

    fn error(&self, e: &RxError) {
        for observer in &self.observers {
            observer.error(e);
        }
    }

    fn complete(&mut self) -> () {
        for observer in &mut self.observers {
            observer.complete();
        }
    }
}
