use rxrs::observable::{of, ObservableLike};

fn main() {
    let mut obs = of(&[1, 2, 3]);

    obs.subscribe(
        |value| { println!("first subscription: {}", value) },
        |error| { println!("{:?}", error); },
        || { println!("first subscription: completed!"); }
    );

    println!("after first subscription");

    obs.subscribe(
        |value| { println!("second subscription: {}", value) },
        |error| { println!("{:?}", error); },
        || { println!("second subscription: completed!"); }
    );

    println!("after second subscription");
}
