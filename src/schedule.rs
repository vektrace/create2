use crate::{Create, Opcode, Result};
use std::io::{Read, Write};

impl<T: Write + Read> Create<T> {
    /// Sends the robot a new schedule.
    ///
    /// Available in modes: [`Passive`](crate::OperatingMode::Passive),
    /// [`Safe`](crate::OperatingMode::Safe), [`Full`](crate::OperatingMode::Full)
    ///
    /// Note that if the robot's schedule or clock button is pressed, the command will be ignored.
    pub fn schedule(&mut self, schedule: Schedule) -> Result<()> {
        let mut buf = [0u8; 16];
        buf[0] = Opcode::Schedule as u8;

        for wt in &schedule {
            let index = wt.day as usize;
            buf[1] |= wt.day.bit();
            buf[2 + index * 2] = wt.time.hour();
            buf[2 + index * 2 + 1] = wt.time.minute();
        }

        self.send(&buf)
    }

    /// Sets the robots clock.
    ///
    /// Available in modes: [`Passive`](crate::OperatingMode::Passive),
    /// [`Safe`](crate::OperatingMode::Safe), [`Full`](crate::OperatingMode::Full)
    ///
    /// Note that if the robot's schedule or clock button is pressed, the command will be ignored.
    pub fn set_daytime(&mut self, day: Day, time: Time) -> Result<()> {
        self.send(&[
            Opcode::SetDayTime as u8,
            day as u8,
            time.hour(),
            time.minute(),
        ])
    }
}

/// A weekly schedule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Schedule {
    slots: [Option<Time>; 7],
}

impl Schedule {
    /// Creates an empty schedule.
    pub const fn new() -> Self {
        Self { slots: [None; 7] }
    }

    /// Sets the scheduled time for a day of the week.
    ///
    /// If the day already has a scheduled time, it will get replaced.
    pub fn insert(&mut self, t: WeeklyTime) {
        self.slots[t.day as usize] = Some(t.time);
    }

    /// Removes the scheduled time for the given day.
    ///
    /// Does nothing if the day has no scheduled time.
    pub fn remove(&mut self, day: Day) {
        self.slots[day as usize] = None;
    }

    /// Returns the time scheduled for `day` or [`None`] if nothing has been scheduled there yet
    pub fn get(&self, day: Day) -> Option<Time> {
        self.slots[day as usize]
    }

    /// Returns an iterator over the days which have a scheduled time
    ///
    /// Iteration is in week order starting at [`Day::Sunday`].
    /// Days without a scheduled time are skipped.
    pub fn iter(&self) -> ScheduleIter<'_> {
        ScheduleIter {
            schedule: self,
            index: 0,
        }
    }

    /// Returns `true` if no day has a scheduled time
    ///
    /// An empty schedule clears all slots on the robot
    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(Option::is_none)
    }
}

impl<'a> IntoIterator for &'a Schedule {
    type Item = WeeklyTime;
    type IntoIter = ScheduleIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<WeeklyTime> for Schedule {
    fn from_iter<I: IntoIterator<Item = WeeklyTime>>(iter: I) -> Self {
        let mut schedule = Schedule::new();

        for time in iter {
            schedule.insert(time);
        }

        schedule
    }
}

/// Iterator over the days a [`Schedule`] has a time for in week order
///
/// Constructed by [`Schedule::iter`]
pub struct ScheduleIter<'a> {
    schedule: &'a Schedule,
    index: usize,
}

impl<'a> Iterator for ScheduleIter<'a> {
    type Item = WeeklyTime;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.schedule.slots.len() {
            let index = self.index;
            self.index += 1;

            if let Some(time) = self.schedule.slots[index] {
                return Some(WeeklyTime {
                    day: Day::try_from(index as u8).expect("slots is indexed by day"),
                    time,
                });
            }
        }

        None
    }
}

/// A day and the time cleaning should start on it
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WeeklyTime {
    pub day: Day,
    pub time: Time,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Day {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

impl Day {
    pub(crate) const fn bit(self) -> u8 {
        1 << (self as u8)
    }
}

impl TryFrom<u8> for Day {
    type Error = ();

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Day::Sunday),
            1 => Ok(Day::Monday),
            2 => Ok(Day::Tuesday),
            3 => Ok(Day::Wednesday),
            4 => Ok(Day::Thursday),
            5 => Ok(Day::Friday),
            6 => Ok(Day::Saturday),
            _ => Err(()),
        }
    }
}

/// Time in 24-hour format
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    hour: u8,
    minute: u8,
}

impl Time {
    /// Construct a new [`Time`] value
    ///
    /// Returns [`None`] if the time is invalid.
    pub const fn new(hour: u8, minute: u8) -> Option<Self> {
        if hour > 23 || minute > 59 {
            None
        } else {
            Some(Self { hour, minute })
        }
    }

    /// Returns the hour of this [`Time`] value in a range of 0-23
    pub const fn hour(&self) -> u8 {
        self.hour
    }

    /// Returns the minute of this [`Time`] value in a range of 0-59
    pub const fn minute(&self) -> u8 {
        self.minute
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_replaces_an_existing_slot() {
        let mut schedule = Schedule::new();

        schedule.insert(WeeklyTime {
            day: Day::Sunday,
            time: Time::new(2, 0).expect("valid time"),
        });

        assert_eq!(
            schedule.get(Day::Sunday),
            Some(Time::new(2, 0).expect("valid time"))
        );

        schedule.insert(WeeklyTime {
            day: Day::Sunday,
            time: Time::new(3, 0).expect("valid time"),
        });

        assert_eq!(
            schedule.get(Day::Sunday),
            Some(Time::new(3, 0).expect("valid time"))
        );
    }

    #[test]
    fn iter_yields_days_in_week_order() {
        let time = Time::new(2, 0).expect("valid time");

        let sunday = WeeklyTime {
            day: Day::Sunday,
            time,
        };
        let monday = WeeklyTime {
            day: Day::Monday,
            time,
        };
        let wednesday = WeeklyTime {
            day: Day::Wednesday,
            time,
        };

        let schedule = Schedule::from_iter([sunday, monday, wednesday]);
        let mut schedule_iter = schedule.into_iter();

        assert_eq!(schedule_iter.next(), Some(sunday));
        assert_eq!(schedule_iter.next(), Some(monday));
        assert_eq!(schedule_iter.next(), Some(wednesday));
    }

    #[test]
    fn invalid_times() {
        assert!(Time::new(23, 59).is_some());
        assert!(Time::new(24, 0).is_none());
        assert!(Time::new(0, 60).is_none());
    }

    #[test]
    fn day_out_of_range_values() {
        assert_eq!(Day::try_from(6), Ok(Day::Saturday));
        assert!(Day::try_from(7).is_err());
    }
}
