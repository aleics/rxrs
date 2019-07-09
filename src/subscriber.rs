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

pub struct Subscriber<T> {
    next_fn: Box<dyn Fn(&T) + Send>,
    error_fn: Box<dyn Fn(&RxError) + Send>,
    complete_fn: Box<dyn Fn() + Send>,
    stopped: bool
}

impl<T> Subscriber<T> {
    pub fn new<F, E, C>(next: F, error: E, complete: C) -> Subscriber<T>
        where F: Fn(&T) + 'static + Send,
              E: Fn(&RxError) + 'static + Send,
              C: Fn() + 'static + Send {
        Subscriber {
            next_fn: Box::new(next),
            error_fn: Box::new(error),
            complete_fn: Box::new(complete),
            stopped: false
        }
    }
}

impl<T> Observer for Subscriber<T> {
    type Value = T;
    type Error = RxError;

    fn next(&self, t: &Self::Value) {
        if !self.stopped {
            (self.next_fn)(t);
        }
    }
    fn error(&self, e: &Self::Error) {
        if !self.stopped {
            (self.error_fn)(e);
        }
    }
    fn complete(&mut self) {
        if !self.stopped {
            self.stopped = true;
            (self.complete_fn)();
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