use rxrs::subject::Subject;
use rxrs::subscriber::Observer;
use rxrs::subscription::Subscription;

fn main() {
    let subject = Subject::new();
    let mut first = subject.subscribe_next(
        |value| println!("first {}", value)
    );

    let mut second = subject.subscribe_next(
        |value| println!("second {}", value)
    );

    subject.next(&0);

    first.unsubscribe();

    subject.next(&1);

    second.unsubscribe();

    subject.next(&2); // this should not be printed
}