use crate::operators::of;
use crate::tests::utils::{values_sent, is_completed};

#[test]
fn create() {
    let obs = of(&[1, 2, 3]);
    assert!(values_sent(&obs, &[1, 2, 3]));
    assert!(is_completed(&obs));
}