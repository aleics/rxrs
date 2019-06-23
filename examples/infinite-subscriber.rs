use rxrs::observable::Observable;

fn main() {
    let obs = Observable::of(5);
    obs.subscribe(
        |value| println!("first-sub next: {}", value),
            |error| println!("first-sub error: {}", error),
            || println!("first-sub completed!")
    );

    obs.subscribe(
        |value| println!("second-sub next: {}", value),
        |error| println!("second-sub error: {}", error),
        || println!("second-sub completed!")
    );

    loop {

    }
}