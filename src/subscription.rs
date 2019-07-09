use std::sync::mpsc::Sender;
use std::cell::RefCell;

use crate::subscriber::Subscriber;

pub trait Subscription {
    fn unsubscribe(&mut self);
}

pub struct ObservableSubscription {
    pub closed: bool,
    unsubscribe_dispatcher: Sender<()>
}

impl ObservableSubscription {
    pub fn new(unsubscribe_dispatcher: Sender<()>) -> ObservableSubscription {
        ObservableSubscription { closed: false, unsubscribe_dispatcher }
    }
}

impl Subscription for ObservableSubscription {
    fn unsubscribe(&mut self) {
        match &self.unsubscribe_dispatcher.send(()) {
            Ok(_) => self.closed = true,
            Err(e) => println!("Unsubscribe action could not be dispatched: {}", e)
        }
    }
}

pub type TrackedSubjectObservers<T> = RefCell<Vec<Option<Subscriber<T>>>>;

pub struct SubjectSubscription<'a, T> {
    pub closed: bool,
    pub subject_ref: &'a TrackedSubjectObservers<T>,
    pub item: usize
}

impl<'a, T> SubjectSubscription<'a, T> {
    pub fn new(subject_ref: &'a TrackedSubjectObservers<T>) -> SubjectSubscription<'a, T> {
        let item = subject_ref.borrow().len() - 1;
        SubjectSubscription { closed: false, subject_ref, item }
    }
}

impl<'a, T> Subscription for SubjectSubscription<'a, T> {
    fn unsubscribe(&mut self) {
        if !self.closed {
            let mut observers = self.subject_ref.borrow_mut();
            observers[self.item] = None;
            self.closed = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ObservableSubscription;
    use std::sync::mpsc::channel;
    use crate::subscription::{Subscription, SubjectSubscription};
    use std::cell::RefCell;
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
}
