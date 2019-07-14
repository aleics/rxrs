use rxrs::operators::of;

fn main() {
    let obs = of(&[1, 2, 3]);

    obs.subscribe_all(
        |value| { println!("first subscription: {}", value) },
        |error| { println!("{:?}", error); },
        || { println!("first subscription: completed!"); }
    );

    println!("after first subscription");

    obs.subscribe_all(
        |value| { println!("second subscription: {}", value) },
        |error| { println!("{:?}", error); },
        || { println!("second subscription: completed!"); }
    );

    println!("after second subscription");
}
