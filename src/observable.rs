use crate::subscription::Subscription;
use crate::error::RxError;
use crate::subscriber::{Subscriber, NextHandler, ErrorHandler, CompleteHandler};

type Observer<T> = Box<dyn Fn(Subscriber<T>) -> ()>;

pub struct Observable<T> {
    observer: Observer<T>
}

impl<T: 'static> Observable<T> {

    pub fn new(observer: Observer<T>) -> Observable<T> {
        Observable { observer }
    }

    pub fn of(values: &'static [T]) -> Observable<T> {
        let observer = Box::new(move |subscriber: Subscriber<T>| {
            for value in values {
                subscriber.next(value);
            }
            subscriber.complete();
        });
        Observable::new(observer)
    }

    pub fn subscribe(
        &self,
        next_handler: NextHandler<T>,
        error_handler:  ErrorHandler<RxError>,
        complete_handler: CompleteHandler
    ) -> Subscription {
        // create a subscription
        let mut subscription = Subscription::new();
        subscription.subscribe();

        // generate a subscriber from the input events
        let subscriber = Subscriber::<T>::new(
            next_handler, error_handler, complete_handler
        );

        (self.observer)(subscriber);

        subscription
    }
}
