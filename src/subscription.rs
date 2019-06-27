

pub struct Subscription {
    pub closed: bool
}

impl Subscription {
    pub fn new() -> Subscription {
        Subscription { closed: true }
    }

    pub fn subscribe(&mut self) {
        self.closed = false;
    }

    pub fn unsubscribe(&mut self) {
        self.closed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::Subscription;

    #[test]
    fn new() {
        let subscription = Subscription::new();
        assert_eq!(subscription.closed, true);
    }

    #[test]
    fn subscribe() {
        let mut subscription = Subscription::new();
        subscription.subscribe();
        assert_eq!(subscription.closed, false);
    }

    #[test]
    fn unsubscribe() {
        let mut subscription = Subscription::new();
        subscription.subscribe();
        subscription.unsubscribe();
        assert_eq!(subscription.closed, true);
    }
}