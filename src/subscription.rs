use std::sync::mpsc::Sender;

#[derive(Default)]
pub struct Subscription {
    pub closed: bool,
    unsubscribe_dispatcher: Option<Sender<()>>
}

impl Subscription {
    pub fn new() -> Subscription {
        Subscription { closed: true, unsubscribe_dispatcher: None }
    }

    pub fn subscribe(&mut self, unsubscribe_dispatcher: Sender<()>) {
        self.closed = false;
        self.unsubscribe_dispatcher = Some(unsubscribe_dispatcher);
    }

    pub fn unsubscribe(&mut self) {
        if let Some(tx) = &self.unsubscribe_dispatcher {
            match tx.send(()) {
                Ok(_) => self.closed = true,
                Err(e) => println!("Unsubscribe action could not be dispatched: {}", e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Subscription;
    use std::sync::mpsc::channel;

    #[test]
    fn new() {
        let subscription = Subscription::new();
        assert_eq!(subscription.closed, true);
    }

    #[test]
    fn subscribe() {
        let mut subscription = Subscription::new();

        let (action, _) = channel();
        subscription.subscribe(action);
        assert_eq!(subscription.closed, false);
    }

    #[test]
    fn unsubscribe() {
        let mut subscription = Subscription::new();

        let (action, _) = channel();
        subscription.subscribe(action);
        subscription.unsubscribe();
        assert_eq!(subscription.closed, false);
    }
}
