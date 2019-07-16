use crate::error::RxError;
use crate::subscriber::{Subscriber, Observer};

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