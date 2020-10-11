use std::cell::RefCell;

use crate::error::RxError;
use crate::observable::Unsubscriber;
use crate::observer::ObserverLike;

pub trait Unsubscribable {
    fn unsubscribe(&mut self);
}

pub struct Subscription {
    pub closed: bool,
    unsubscriber: Unsubscriber,
}

impl Subscription {
    pub fn new(unsubscriber: Unsubscriber) -> Subscription {
        Subscription { closed: false, unsubscriber }
    }
}

impl Unsubscribable for Subscription {
    fn unsubscribe(&mut self) {
        if !self.closed {
            self.unsubscriber.call();
            self.closed = true;
        }
    }
}

pub type TrackedSubjectObservers<O> = RefCell<Vec<Option<O>>>;

pub struct SubjectSubscription<'a, O> {
    pub closed: bool,
    pub subject_ref: &'a TrackedSubjectObservers<O>,
    pub item: usize,
}

impl<'a, T, O> SubjectSubscription<'a, O> where O: ObserverLike<Value=T, Error=RxError> {
    pub fn new(subject_ref: &'a TrackedSubjectObservers<O>) -> SubjectSubscription<'a, O> {
        let item = subject_ref.borrow().len() - 1;
        SubjectSubscription { closed: false, subject_ref, item }
    }
}

impl<'a, T, O> Unsubscribable for SubjectSubscription<'a, O> where O: ObserverLike<Value=T, Error=RxError> {
    fn unsubscribe(&mut self) {
        if !self.closed {
            let mut observers = self.subject_ref.borrow_mut();
            observers[self.item] = None;
            self.closed = true;
        }
    }
}
