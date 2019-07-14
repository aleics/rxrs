use rxrs::operators::{of, map};

fn main() {
    let obs = of(&[1, 2, 3]);
    obs.pipe(
        map(|item: &i32| item * 2)
    ).subscribe_fn(
        |item| println!("{}", item),
        |e| println!("{}", e),
        ||println!("completed")
    );
}