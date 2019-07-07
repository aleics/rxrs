use std::sync::mpsc::Receiver;

use crate::error::RxError;

pub trait Observer {
    type Value;
    type Error;

    fn next(&self, value: &Self::Value) -> ();
    fn error(&self, e: &Self::Error) -> ();
    fn complete(&mut self) -> ();
}

pub type NextHandler<T> = fn(&T);
pub type ErrorHandler<E> = fn(&E);
pub type CompleteHandler = fn();

pub type SubscriberFn<T> = Box<dyn Fn(Subscriber<T>, Receiver<()>) -> ()>;

pub struct Subscriber<T> {
    next_handler: NextHandler<T>,
    error_handler: ErrorHandler<RxError>,
    complete_handler: CompleteHandler,
    stopped: bool
}

impl<T> Subscriber<T> {
    pub fn new(
        next_handler: NextHandler<T>,
        error_handler: ErrorHandler<RxError>,
        complete_handler: CompleteHandler
    ) -> Subscriber<T> {
        Subscriber { next_handler, error_handler, complete_handler, stopped: false }
    }
}

impl<T> Observer for Subscriber<T> {
    type Value = T;
    type Error = RxError;

    fn next(&self, t: &Self::Value) {
        if !self.stopped {
            (self.next_handler)(t);
        }
    }
    fn error(&self, e: &Self::Error) {
        if !self.stopped {
            (self.error_handler)(e);
        }
    }
    fn complete(&mut self) {
        if !self.stopped {
            self.stopped = true;
            (self.complete_handler)();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Subscriber, Observer};
    use crate::error::RxError;

    #[test]
    fn new() {
        let subscriber = Subscriber::<u32>::new(
            |value| println!("{}", value),
            |err| println!("{}", err),
            || println!("complete")
        );
        assert_eq!(subscriber.stopped, false);
    }

    #[test]
    fn next() {
        let subscriber = Subscriber::<u32>::new(
            |value| assert_eq!(&1, value),
            |_err| assert_eq!(true, false),
            || assert_eq!(true, false)
        );

        subscriber.next(&1);
    }

    #[test]
    fn error() {
        let subscriber = Subscriber::<u32>::new(
            |_value| assert_eq!(true, false),
            |_err| assert_eq!(true, true),
            || assert_eq!(true, false)
        );

        subscriber.error(&RxError::CustomError("some error".to_string()));
    }

    #[test]
    fn complete() {
        let mut subscriber = Subscriber::<u32>::new(
            |_value| assert_eq!(true, false),
            |_err| assert_eq!(true, false),
            || assert_eq!(true, true)
        );

        subscriber.complete();
        assert_eq!(subscriber.stopped, true);
    }
}