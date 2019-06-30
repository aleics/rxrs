#[derive(Default)]
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
}
