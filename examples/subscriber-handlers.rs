use rxrs::observable::{Observable, SubscriberFn};
use rxrs::subscriber::Observer;

fn main() {
    let subscriber_fn: SubscriberFn<String> = Box::new(|mut subscriber| {
        subscriber.next(&String::from("Oh"));
        subscriber.next(&String::from("hi"));
        subscriber.next(&String::from("Mark!"));
        subscriber.complete();

        subscriber.next(&String::from("This should not be printed."));
    });

    Observable::new(subscriber_fn).subscribe(
        |value| println!("{}", value),
        |error| println!("{}", error),
        || println!("completed")
    ).unwrap();
}