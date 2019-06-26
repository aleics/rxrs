use rxrs::observable::Observable;

fn main() {
    let sync_observable = Observable::of(1);

    sync_observable.subscribe(
        |value| { println!("first subscription: {}", value) },
        |error| { println!("{:?}", error); },
        || { println!("first subscription: completed!"); }
    );

    println!("after first subscription");

    sync_observable.subscribe(
        |value| { println!("second subscription: {}", value) },
        |error| { println!("{:?}", error); },
        || { println!("second subscription: completed!"); }
    );

    println!("after second subscription");

    loop {

    }
}