use rxrs::observable::interval;

fn main() {
    let observable = interval(1000);
    observable.subscribe(
        |value| println!("{}", value),
        |error| println!("{}", error),
        || println!("completed")
    );

    println!("after subscribe");

    loop {

    }
}