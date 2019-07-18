use std::sync::mpsc::Sender;
use std::cell::RefCell;

use crate::subscriber::Observer;
use crate::error::RxError;

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

pub type TrackedSubjectObservers<O> = RefCell<Vec<Option<O>>>;

pub struct SubjectSubscription<'a, O> {
	pub closed: bool,
	pub subject_ref: &'a TrackedSubjectObservers<O>,
	pub item: usize
}

impl<'a, T, O> SubjectSubscription<'a, O> where O: Observer<Value=T, Error=RxError> {
	pub fn new(subject_ref: &'a TrackedSubjectObservers<O>) -> SubjectSubscription<'a, O> {
		let item = subject_ref.borrow().len() - 1;
		SubjectSubscription { closed: false, subject_ref, item }
	}
}

impl<'a, T, O> Subscription for SubjectSubscription<'a, O> where O: Observer<Value=T, Error=RxError> {
	fn unsubscribe(&mut self) {
		if !self.closed {
			let mut observers = self.subject_ref.borrow_mut();
			observers[self.item] = None;
			self.closed = true;
		}
	}
}
