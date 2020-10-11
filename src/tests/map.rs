use crate::operators::of;
use crate::tests::utils::{is_completed, values_sent};

#[test]
fn map_i32() {
    let obs = of(&[1, 2, 3])
        .map(|item| item * 2);

    assert!(values_sent(&obs, &[2, 4, 6]));
    assert!(is_completed(&obs));
}

#[test]
fn map_i32_to_string() {
    let obs = of(&[1, 2, 3])
        .map(|item| item.to_string());

    assert!(values_sent(&obs, &[String::from("1"), String::from("2"), String::from("3")]));
    assert!(is_completed(&obs));
}

#[test]
fn map_str() {
    let data = ["Hello"];
    let obs = of(&data)
        .map(|_| "Bye");

    assert!(values_sent(&obs, &["Bye"]));
    assert!(is_completed(&obs));
}

#[test]
fn map_string() {
    let data = [String::from("Hello")];
    let obs = of(&data)
        .map(|_| String::from("Bye"));

    assert!(values_sent(&obs, &[String::from("Bye")]));
    assert!(is_completed(&obs));
}

#[test]
fn map_data() {
    #[derive(PartialEq, Clone)]
    struct DataItem {
        value: i32
    }
    ;

    let data = [
        DataItem { value: 1 },
        DataItem { value: 2 },
        DataItem { value: 3 }
    ];

    let obs = of(&data)
        .map(|item: &DataItem| DataItem { value: item.value * 2 });

    assert!(values_sent(&obs, &[
        DataItem { value: 2 },
        DataItem { value: 4 },
        DataItem { value: 6 }
    ]));
    assert!(is_completed(&obs));
}

#[test]
fn multiple_subscriptions() {
    let data = [1, 2, 3];

    let even = of(&data)
        .map(|item| item % 2 == 0);

    assert!(values_sent(&even, &[false, true, false]));
    assert!(is_completed(&even));

    let odd = of(&data)
        .map(|item| item % 2 == 1);

    assert!(values_sent(&odd, &[true, false, true]));
    assert!(is_completed(&odd));
}


#[test]
fn successive_maps() {
    let data = [1, 2, 3];

    let result = of(&data)
        .map(|item| item * 2)
        .map(|item| item / 2);

    assert!(values_sent(&result, &data));
    assert!(is_completed(&result));
}

#[test]
fn successive_operators() {
    let data = [1, 2, 3];

    let result = of(&data)
        .map(|item: &i32| item * 2)
        .filter(|item: &i32| item > &2);

    assert!(values_sent(&result, &[4, 6]));
    assert!(is_completed(&result));
}