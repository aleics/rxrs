use rxrs::observable::{interval, ObservableLike};
use std::thread;
use std::time::Duration;

fn main() {
    let mut subscription = interval(1)
        .subscribe(
            |value| println!("{}", value),
            |error| println!("{}", error),
            || println!("completed")
        );

    let j = thread::spawn(move || {
        thread::sleep(Duration::from_millis(5));
        subscription.unsubscribe();
        println!("unsubscribed");
    });

    j.join().unwrap();
}