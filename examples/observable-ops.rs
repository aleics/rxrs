use rxrs::operators::{of, map, filter};

fn main() {
    let obs_map = of(&[1, 2, 3]);
    obs_map.pipe(
        map(|item| item * 2)
    ).subscribe_next(|item| println!("map: {}", item));

    let obs_filter = of(&[1, 2, 3]);
    obs_filter.pipe(
        filter(|item: &i32| item.clone() > 1)
    ).subscribe_next(|item| println!("filter: {}", item));
}