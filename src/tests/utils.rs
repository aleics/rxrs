use crate::observable::Observable;
use crate::subscriber::Subscriber;
use std::sync::mpsc::channel;

pub fn values_sent<T>(observable: &Observable<T, Subscriber<T>>, expected: &[T]) -> bool
    where T: 'static + Clone + PartialEq + Sync + Send {
    let (tx, rx) = channel();

    let mut result = Vec::new();
    observable.subscribe_next(move |value| {
        tx.send(value.clone()).unwrap();
    });

    for _ in expected {
        if let Ok(value) = rx.recv() {
            result.push(value);
        } else {
            return false;
        }
    }

    for i in 0..expected.len() {
        if result[i] != expected[i] {
            return false;
        }
    }
    true
}

pub fn is_completed<T>(observable: &Observable<T, Subscriber<T>>) -> bool {
    let (tx, rx) = channel();
    observable.subscribe_complete(move || tx.send(true).unwrap());

    rx.try_recv().is_ok()
}