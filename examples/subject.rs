use rxrs::subject::Subject;
use rxrs::subscriber::Observer;
use rxrs::subscription::Subscription;

fn main() {
    let subject = Subject::new();
    let mut first = subject.subscribe_fn(
        |value| println!("first {}", value),
        |e| println!("first {}", e),
        || println!("complete")
    );

    let mut second = subject.subscribe_fn(
        |value| println!("second {}", value),
        |e| println!("second {}", e),
        || println!("complete")
    );

    subject.next(&0);

    first.unsubscribe();

    subject.next(&1);

    second.unsubscribe();

    subject.next(&2); // this should not be printed
}