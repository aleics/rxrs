use rxrs::operators::{of, map};

#[derive(Debug)]
struct DataItem {
    value: i32
}

fn main() {
    let obs_static = of(&[1, 2, 3]);

    obs_static.subscribe_next(|value| println!("first: {}", value));
    println!("after first");

    obs_static.subscribe_next(|value| println!("second: {}", value));
    println!("after second");

    let data = [
        DataItem { value: 1 },
        DataItem { value: 2 },
        DataItem { value: 3 }
    ];
    let obs_data = of(&data);
    obs_data.pipe(
        map(|item: &DataItem| DataItem { value: item.value + 1})
    ).subscribe_next(|item| println!("{:?}", item));
}
