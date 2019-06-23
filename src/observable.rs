use std::thread;

use crossbeam::{Sender, Receiver, unbounded};

use crate::subscription::Subscription;
use crate::error::RxError;
use crate::observer::{Observer, NextHandler, ErrorHandler, CompleteHandler};

pub struct Observable<T> {
    tx: Sender<T>,
    rx: Receiver<T>
}

impl<T: 'static + Sized + Send> Observable<T> {

    pub fn of(value: T) -> Observable<T> {
        let (tx, rx): (Sender<T>, Receiver<T>) = unbounded();
        let observable = Observable { tx, rx: Arc::new(Mutex::new(rx)) };

        let tx_thread = observable.tx.clone();
        thread::spawn(move || {
            tx_thread.send(value).unwrap();
        });

        observable
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

        // generate an observer from the input events
        let observer = Observer::<T>::new(
            next_handler, error_handler, complete_handler
        );

        // open a new thread and wait for events of the observable
        let rx = self.rx.clone();
        thread::spawn(move || {
            match rx.recv() {
                Err(e) => observer.error(RxError::SubscribeError(e)),
                Ok(data) => observer.next(data)
            };
        });

        subscription
    }
}
