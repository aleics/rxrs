use std::sync::mpsc::channel;
use std::cell::RefCell;

use crate::subscription::{Subscription, SubjectSubscription, ObservableSubscription};
use crate::subscriber::Subscriber;

#[test]
fn observable_new() {
    let (tx, _) = channel();
    let subscription = ObservableSubscription::new(tx);
    assert_eq!(subscription.closed, false);
}

#[test]
fn observable_unsubscribe() {
    let (tx, rx) = channel();
    let mut subscription = ObservableSubscription::new(tx);

    subscription.unsubscribe();

    if let Ok(_) = rx.recv() {
        assert_eq!(subscription.closed, true);
    } else {
        assert_eq!(subscription.closed, false);
    }
}

#[test]
fn subject_new() {
    let subscriber: Subscriber<i32> = Subscriber::new(
        |value| println!("{}", value),
        |e| println!("{}", e),
        || println!("complete")
    );
    let mut observers = Vec::new();
    observers.push(Some(subscriber));

    let observers_ref = &RefCell::new(observers);
    let subscription = SubjectSubscription::new(observers_ref);

    assert_eq!(subscription.closed, false);
}

#[test]
fn subject_unsubscribe() {
    let subscriber: Subscriber<i32> = Subscriber::new(
        |value| println!("{}", value),
        |e| println!("{}", e),
        || println!("complete")
    );
    let mut observers = Vec::new();
    observers.push(Some(subscriber));

    let observers_ref = &RefCell::new(observers);
    let mut subscription = SubjectSubscription::new(observers_ref);

    subscription.unsubscribe();

    assert_eq!(subscription.closed, true);
}

#[test]
fn subject_multiple_unsubscribe() {
    let subscriber_a: Subscriber<i32> = Subscriber::new(
        |value| println!("{}", value),
        |e| println!("{}", e),
        || println!("complete")
    );
    let mut observers = Vec::new();
    observers.push(Some(subscriber_a));

    let observers_ref = RefCell::new(observers);
    let mut first = SubjectSubscription::new(&observers_ref);

    let subscriber_b: Subscriber<i32> = Subscriber::new(
        |value| println!("{}", value),
        |e| println!("{}", e),
        || println!("complete")
    );
    observers_ref.borrow_mut().push(Some(subscriber_b));
    let mut second = SubjectSubscription::new(&observers_ref);

    first.unsubscribe();
    assert_eq!(first.closed, true);

    second.unsubscribe();
    assert_eq!(second.closed, true);
}