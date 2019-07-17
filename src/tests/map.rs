use crate::operators::{of, map};
use crate::tests::utils::{values_sent, is_completed};

#[test]
fn map_i32() {
    let obs = of(&[1, 2, 3]).pipe(
        map(|item| item * 2)
    );

    assert!(values_sent(&obs, &[2, 4, 6]));
    assert!(is_completed(&obs));
}

#[test]
fn map_str() {
    let data = ["Hello"];
    let obs = of(&data).pipe(
        map(|_| "Bye")
    );

    assert!(values_sent(&obs, &["Bye"]));
    assert!(is_completed(&obs));
}

#[test]
fn map_string() {
    let data = [String::from("Hello")];
    let obs = of(&data).pipe(
        map(|_| String::from("Bye"))
    );

    assert!(values_sent(&obs, &[String::from("Bye")]));
    assert!(is_completed(&obs));
}

#[test]
fn map_data() {
    #[derive(PartialEq, Clone)]
    struct DataItem {
        value: i32
    };

    let data = [
        DataItem { value: 1 },
        DataItem { value: 2 },
        DataItem { value: 3 }
    ];

    let obs = of(&data).pipe(
        map(|item: &DataItem| DataItem { value: item.value * 2 })
    );
    assert!(values_sent(&obs, &[
        DataItem { value: 2 },
        DataItem { value: 4 },
        DataItem { value: 6 }
    ]));
    assert!(is_completed(&obs));
}

#[test]
fn multiple_subscription() {
    let data = [1, 2, 3];

    let even = of(&data).pipe(
        map(|item| item % 2 == 0)
    );
    assert!(values_sent(&even, &[false, true, false]));
    assert!(is_completed(&even));

    let odd = of(&data).pipe(
        map(|item| item % 2 == 1)
    );
    assert!(values_sent(&odd, &[true, false, true]));
    assert!(is_completed(&odd));
}


#[test]
fn successive_pipes() {
    let data = [1, 2, 3];

    let multiplied = of(&data).pipe(
        map(|item| item * 2)
    );

    let result = multiplied.pipe(
        map(|item| item / 2)
    );
    assert!(values_sent(&result, &data));
    assert!(is_completed(&result));
}