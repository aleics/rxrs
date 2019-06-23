// Will receive call to Subscriber.onSubscribe(Subscription) once after passing an instance of
// Subscriber to Publisher.subscribe(Subscriber).
pub trait Subscriber<T, S> where S : Subscription {
    fn next(value: T);
    fn error();
    fn complete();
    fn subscribe(s: S);
}

// A Subscription represents a one-to-one lifecycle of a Subscriber subscribing to a Publisher.
pub trait Subscription {
    fn cancel();
    fn request(n: i64);
}

// A Publisher is a provider of a potentially unbounded number of sequenced elements, publishing
// them according to the demand received from its Subscriber(s).
pub trait Publisher<T, S, R> where R : Subscriber<T, S>, S : Subscription {
    fn subscribe(s: R);
}

// A Processor represents a processing stage—which is both a Subscriber and a Publisher and obeys
// the contracts of both.
pub trait Processor<T, S, R>: Subscriber<T, S> + Publisher<T, S, R> where
    R : Subscriber<T, S>,
    S : Subscription {}