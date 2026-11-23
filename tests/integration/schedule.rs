use crate::common::MockPort;
use create2::{Create, Day, Opcode, Schedule, Time, WeeklyTime};

#[test]
fn schedule() {
    let schedule = Schedule::from_iter([
        WeeklyTime {
            day: Day::Monday,
            time: Time::new(8, 30).unwrap(),
        },
        WeeklyTime {
            day: Day::Saturday,
            time: Time::new(9, 0).unwrap(),
        },
    ]);

    let mut create = Create::new(MockPort::default());
    create.schedule(schedule).unwrap();

    assert_eq!(
        create.port().to_robot,
        &[
            Opcode::Schedule as u8,
            0b0100_0010,
            0,
            0, // Sunday
            8,
            30, // Monday
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0, // Tue - Fri
            9,
            0, // Saturday
        ]
    );
}

#[test]
fn empty_schedule() {
    let mut create = Create::new(MockPort::default());
    create.schedule(Schedule::new()).unwrap();

    let mut expected = [0u8; 16];
    expected[0] = Opcode::Schedule as u8;

    assert_eq!(create.port().to_robot, expected);
}

#[test]
fn set_daytime() {
    let mut create = Create::new(MockPort::default());
    create
        .set_daytime(Day::Monday, Time::new(20, 30).expect("valid time"))
        .unwrap();

    assert_eq!(
        create.port().to_robot,
        &[Opcode::SetDayTime as u8, Day::Monday as u8, 20, 30]
    );
}
