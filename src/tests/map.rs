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