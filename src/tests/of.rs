use crate::operators::of;
use crate::tests::utils::{values_sent, is_completed};

#[test]
fn create_i32() {
    let obs = of(&[1, 2, 3]);
    assert!(values_sent(&obs, &[1, 2, 3]));
    assert!(is_completed(&obs));
}

#[test]
fn create_data() {
    #[derive(PartialEq, Clone)]
    struct DataItem {
        value: i32
    };

    let data = [
        DataItem { value: 1 },
        DataItem { value: 2 },
        DataItem { value: 3 }
    ];

    let obs = of(&data);
    assert!(values_sent(&obs, &data));
    assert!(is_completed(&obs));
}