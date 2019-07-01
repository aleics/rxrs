use rxrs::observable::interval;
use std::thread;
use std::time::Duration;

fn main() {
    let observable = interval(1);
    let mut subscription = observable.subscribe(
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