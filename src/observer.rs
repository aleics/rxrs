use crate::error::RxError;

pub trait ObserverLike {
    type Value;
    type Error;

    fn next(&self, value: &Self::Value) -> ();
    fn error(&self, e: &Self::Error) -> ();
    fn complete(&mut self) -> ();
}

pub struct Observer<T> {
    next_fn: Box<dyn Fn(&T) + Send>,
    error_fn: Box<dyn Fn(&RxError) + Send>,
    complete_fn: Box<dyn Fn() + Send>,
    pub stopped: bool,
}

impl<T> Observer<T> {
    pub fn new<N, E, C>(next: N, error: E, complete: C) -> Observer<T>
        where N: Fn(&T) + 'static + Send,
              E: Fn(&RxError) + 'static + Send,
              C: Fn() + 'static + Send {
        Observer {
            next_fn: Box::new(next),
            error_fn: Box::new(error),
            complete_fn: Box::new(complete),
            stopped: false,
		}
    }
}

impl<T> ObserverLike for Observer<T> {
    type Value = T;
    type Error = RxError;

    fn next(&self, t: &Self::Value) {
        if !self.stopped {
            (self.next_fn)(t);
        }
    }
    fn error(&self, e: &Self::Error) {
        if !self.stopped {
            (self.error_fn)(e);
        }
    }
    fn complete(&mut self) {
        if !self.stopped {
            self.stopped = true;
            (self.complete_fn)();
        }
    }
}