use crate::common::MockPort;
use create2::{Create, Error};
use std::io::ErrorKind;

#[test]
fn write_failure() {
    let mut create = Create::new(MockPort {
        fail: Some(ErrorKind::Other),
        ..Default::default()
    });

    let err = create.start().unwrap_err();
    assert!(matches!(err, Error::Io(_)));
}
