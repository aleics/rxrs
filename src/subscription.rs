use std::sync::mpsc::Sender;

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

pub struct SubjectSubscription {
    pub closed: bool
}

impl SubjectSubscription {
    pub fn new() -> SubjectSubscription {
        SubjectSubscription { closed: false }
    }
}

impl Subscription for SubjectSubscription {
    fn unsubscribe(&mut self) {
        self.closed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::ObservableSubscription;
    use std::sync::mpsc::channel;
    use crate::subscription::{Subscription, SubjectSubscription};

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
        let subscription = SubjectSubscription::new();
        assert_eq!(subscription.closed, false);
    }

    #[test]
    fn subject_unsubscribe() {
        let mut subscription = SubjectSubscription::new();
        subscription.unsubscribe();
        assert_eq!(subscription.closed, true);
    }
}
