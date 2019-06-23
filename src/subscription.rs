use time::Tm;

pub struct Subscription {
    subscribed_at: Option<Tm>,
    unsubscribed_at: Option<Tm>
}

impl Subscription {
    pub fn new() -> Subscription {
        Subscription {
            subscribed_at: None,
            unsubscribed_at: None
        }
    }

    pub fn subscribe(&mut self) {
        self.subscribed_at = Some(time::now())
    }

    pub fn unsubscribe(&mut self) {
        self.unsubscribed_at = Some(time::now())
    }
}

#[cfg(test)]
mod tests {
    use super::Subscription;

    #[test]
    fn new() {
        let subscription = Subscription::new();
        assert_eq!(subscription.subscribed_at, None);
        assert_eq!(subscription.unsubscribed_at, None);
    }

    #[test]
    fn subscribe() {
        let mut subscription = Subscription::new();

        let sub_time_sec = time::now().tm_sec;
        subscription.subscribe();

        assert_eq!(
            Some(sub_time_sec),
            subscription.subscribed_at.map(|s| s.tm_sec)
        );
    }

    #[test]
    fn unsubscribe() {
        let mut subscription = Subscription::new();

        let unsub_time_sec = time::now().tm_sec;
        subscription.unsubscribe();

        assert_eq!(
            Some(unsub_time_sec),
            subscription.unsubscribed_at.map(|s| s.tm_sec)
        );
    }
}