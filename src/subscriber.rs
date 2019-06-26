use crate::error::RxError;

pub type NextHandler<T> = fn(T) -> ();
pub type ErrorHandler<E> = fn(E) -> ();
pub type CompleteHandler = fn() -> ();

pub struct Subscriber<T> {
    next_handler: NextHandler<T>,
    error_handler: ErrorHandler<RxError>,
    complete_handler: CompleteHandler
}

impl<T: Sized> Subscriber<T> {

    pub fn new(
        next_handler: NextHandler<T>,
        error_handler:  ErrorHandler<RxError>,
        complete_handler: CompleteHandler
    ) -> Subscriber<T> {
        Subscriber { next_handler, error_handler, complete_handler }
    }

    pub fn next(&self, t: T) {
        (self.next_handler)(t);
    }
    pub fn error(&self, e: RxError) {
        (self.error_handler)(e);
    }
    pub fn complete(&self) {
        (self.complete_handler)();
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Display, Formatter, Result};
    use super::{Subscriber};
    use crate::error::RxError;

    #[derive(Debug)]
    struct TestError;

    impl Display for TestError {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "something happened")
        }
    }

    #[test]
    fn new() {
        let observer = Subscriber::<u32>::new(
            |value| println!("{}", value),
            |err| println!("{}", err),
            || println!("complete")
        );
    }

    #[test]
    fn next() {
        let observer = Subscriber::<u32>::new(
            |value| assert_eq!(1, value),
            |_err| assert_eq!(true, false),
            || assert_eq!(true, false)
        );

        observer.next(1);
    }

    #[test]
    fn error() {
        let observer = Subscriber::<u32>::new(
            |_value| assert_eq!(true, false),
            |_err| assert_eq!(true, true),
            || assert_eq!(true, false)
        );

        observer.error(RxError::CustomError("some error".to_string()));
    }

    #[test]
    fn complete() {
        let observer = Subscriber::<u32>::new(
            |_value| assert_eq!(true, false),
            |_err| assert_eq!(true, false),
            || assert_eq!(true, true)
        );

        observer.complete();
    }
}