use std::thread::sleep;
use std::time::Duration;

use crate::operators::interval;
use crate::tests::utils::{is_completed, values_sent};

#[test]
fn create() {
    let obs = interval(1);

    sleep(Duration::from_millis(2));
    assert!(values_sent(&obs, &[0, 1, 2]));
    assert_eq!(is_completed(&obs), false);
}