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

pub struct SubjectSubscription<'a, T> {
    pub closed: bool,
    pub subject_ref: &'a RefCell<Vec<Subscriber<T>>>,
    pub item: usize
}

impl<'a, T> SubjectSubscription<'a, T> {
    pub fn new(subject_ref: &'a RefCell<Vec<Subscriber<T>>>) -> SubjectSubscription<'a, T> {
        let item = subject_ref.borrow().len() - 1;
        SubjectSubscription { closed: false, subject_ref, item }
    }
}

impl<'a, T> Subscription for SubjectSubscription<'a, T> {
    fn unsubscribe(&mut self) {
        if !self.closed {
            self.subject_ref.borrow_mut().remove(self.item);
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
        let observers = RefCell::new(Vec::<Subscriber<i32>>::new());
        let subscription = SubjectSubscription::new(&observers);
        assert_eq!(subscription.closed, false);
    }

    #[test]
    fn subject_unsubscribe() {
        let observers = RefCell::new(Vec::<Subscriber<i32>>::new());
        let mut subscription = SubjectSubscription::new(&observers);

        subscription.unsubscribe();

        assert_eq!(subscription.closed, true);
    }
}
