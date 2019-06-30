use rxrs::observable::interval;
use std::thread;
use std::time::Duration;

fn main() {
    let observable = interval(1000);
    let mut sub = observable.subscribe(
        |value| println!("{}", value),
        |error| println!("{}", error),
        || println!("completed")
    );

    println!("subscribed");

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        sub.unsubscribe();
        println!("unsubscribed");
    });

    loop { }
}