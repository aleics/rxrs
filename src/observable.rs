use crate::subscription::Subscription;
use crate::error::RxError;
use crate::subscriber::{Subscriber, Observer, NextHandler, ErrorHandler, CompleteHandler};

type SubscribeFn<T> = Box<dyn Fn(Subscriber<T>) -> ()>;

pub struct Observable<T> {
    observer: SubscribeFn<T>
}

impl<T: 'static> Observable<T> {

    pub fn new(observer: SubscribeFn<T>) -> Observable<T> {
        Observable { observer }
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

pub fn of<T>(values: &'static [T]) -> Observable<T> {
    let observer = Box::new(move |mut subscriber: Subscriber<T>| {
        for value in values {
            subscriber.next(value);
        }
        subscriber.complete();
    });
    Observable::new(observer)
}