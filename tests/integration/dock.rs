use crate::common::MockPort;
use create2::{Create, Opcode};

#[test]
fn seek_dock() {
    let mut create = Create::new(MockPort::default());

    create.seek_dock().unwrap();

    assert_eq!(create.port().to_robot, &[Opcode::SeekDock as u8]);
}
