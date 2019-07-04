use rxrs::subject::Subject;
use rxrs::subscriber::Observer;
use rxrs::subscription::Subscription;

fn main() {
    let subject = Subject::<i32>::new();
    subject.subscribe(
        |value| println!("{}", value),
        |e| println!("{}", e),
        || println!("complete")
    );

    let mut subscription = subject.subscribe(
        |value| println!("{}", value),
        |e| println!("{}", e),
        || println!("complete")
    );

    subject.next(&0);

    subject.next(&1);

    subscription.unsubscribe();

    subject.next(&2);
}