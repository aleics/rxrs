use std::thread::{spawn, sleep};
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver};

use crate::subscription::Subscription;
use crate::error::RxError;
use crate::subscriber::{Subscriber, Observer, NextHandler, ErrorHandler, CompleteHandler};

pub type SubscriberFn<T> = Box<dyn Fn(Subscriber<T>, Receiver<()>) -> ()>;

pub struct Observable<T> {
    observer: SubscriberFn<T>
}

impl<T: 'static> Observable<T> {

    pub fn new(observer: SubscriberFn<T>) -> Observable<T> {
        Observable { observer }
    }

    pub fn subscribe(
        &self,
        next_handler: NextHandler<T>,
        error_handler:  ErrorHandler<RxError>,
        complete_handler: CompleteHandler
    ) -> Subscription {
        // generate a subscriber from the input events
        let subscriber = Subscriber::<T>::new(
            next_handler, error_handler, complete_handler
        );

        // call the observer callback function and include a channel receiver for the
        // a possible unsubscribe action
        let (tx, rx): (Sender<()>, Receiver<()>) = channel();
        (self.observer)(subscriber, rx);

        // create a subscription and subscribe to the previous callback
        // a channel sender is sent to the subscription so that it can be unsubscribed
        let mut subscription = Subscription::new();
        subscription.subscribe(tx);

        subscription
    }
}

pub fn of<T>(values: &'static [T]) -> Observable<T> {
    let observer = Box::new(move |mut subscriber: Subscriber<T>, _: Receiver<()>| {
        for value in values {
            subscriber.next(value);
        }
        subscriber.complete();
    });
    Observable::new(observer)
}

pub fn interval(interval_time: u64) -> Observable<u64> {
    let observer = Box::new(move |subscriber: Subscriber<u64>, unsubscriber: Receiver<()>| {
        spawn(move || {
            let mut count = 0;

            loop {
                sleep(Duration::from_millis(interval_time));
                subscriber.next(&count);

                count += 1;

                if unsubscriber.try_recv().is_ok() {
                    break;
                }
            }
        });
    });
    Observable::new(observer)
}