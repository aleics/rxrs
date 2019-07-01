use rxrs::subject::Subject;
use rxrs::subscriber::Observer;
use rxrs::observable::ObservableLike;

fn main() {
    let mut subject = Subject::<i32>::new();
    let mut subscription = subject.subscribe(
        |value| println!("{}", value),
        |e| println!("{}", e),
        || println!("complete")
    );

    subject.next(&0);

    subject.next(&1);

    subject.complete();

    subject.next(&2);

    subscription.unsubscribe();
}