use rxrs::operators::of;

fn main() {
    let obs = of(&[1, 2, 3]);

    obs.subscribe_next(|value| println!("first: {}", value));
    println!("after first");

    obs.subscribe_next(|value| println!("second: {}", value));
    println!("after second");
}
