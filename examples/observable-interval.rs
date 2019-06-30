use rxrs::observable::interval;
use std::thread;
use std::time::Duration;

fn main() {
    let observable = interval(1000);
    let mut sub = observable.subscribe(
        |value| println!("{}", value),
        |error| println!("{}", error),
        || println!("completed")
    ).unwrap();

    println!("after subscribe");

    thread::sleep(Duration::from_secs(3));

    sub.unsubscribe();
    println!("after unsubscribe");

    loop {

    }
}