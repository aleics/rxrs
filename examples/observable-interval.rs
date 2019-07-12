use std::thread;
use std::time::Duration;

use rxrs::subscription::Subscription;
use rxrs::operators::interval;

fn main() {
    let observable = interval(1);

    let mut first_subscription = observable.subscribe_fn(
        |value| println!("first: {}", value),
        |error| println!("{}", error),
        || println!("completed")
    );

    let mut second_subscription = observable.subscribe_fn(
        |value| println!("second: {}", value),
        |error| println!("{}", error),
        || println!("completed")
    );

    let j = thread::spawn(move || {
        thread::sleep(Duration::from_millis(5));
        first_subscription.unsubscribe();
        println!("first unsubscribed");

        thread::sleep(Duration::from_millis(5));
        second_subscription.unsubscribe();
        println!("second unsubscribed");
    });

    j.join().unwrap();
}