use std::cell::RefCell;

use crate::subscription::{Unsubscribable, SubjectSubscription, Subscription};
use crate::subscriber::Subscriber;
use crate::observable::Unsubscriber;

#[test]
fn observable_new() {
	let unsubscriber = Unsubscriber::new(|| {});
	let subscription = Subscription::new(unsubscriber);
	assert_eq!(subscription.closed, false);
}

#[test]
fn observable_unsubscribe() {
	let unsubscriber = Unsubscriber::new(|| {});
	let mut subscription = Subscription::new(unsubscriber);

	subscription.unsubscribe();
	assert_eq!(subscription.closed, true);
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