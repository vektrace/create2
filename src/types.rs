/// iRobot Create 2 Opcodes according to the [Create 2 Open Interface Spec](https://cdn-shop.adafruit.com/datasheets/create_2_Open_Interface_Spec.pdf)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Start = 128,
    Reset = 7,
    Stop = 173,
    Baud = 129,
    Safe = 131,
    Full = 132,
    Clean = 135,
    Max = 136,
    Spot = 134,
    SeekDock = 143,
    Power = 133,
    Schedule = 167,
    SetDayTime = 168,
    Drive = 137,
    DriveDirect = 145,
    DrivePWM = 146,
    Motors = 138,
    PWMMotors = 144,
    LEDs = 139,
    SchedulingLEDs = 162,
    DigitLEDsRaw = 163,
    Buttons = 165,
    DigitLEDsASCII = 164,
    Song = 140,
    Play = 141,
    Sensors = 142,
    QueryList = 149,
    Stream = 148,
    PauseResumeStream = 150,
}

/// OI baud rates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BaudRate {
    B300 = 0,
    B600 = 1,
    B1200 = 2,
    B2400 = 3,
    B4800 = 4,
    B9600 = 5,
    B14400 = 6,
    B19200 = 7,
    B28800 = 8,
    B38400 = 9,
    B57600 = 10,
    /// Default
    B115200 = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingMode {
    /// After a battery change or when power is first turned on, the OI is in Off mode.
    Off,
    /// The OI enters Passive mode when the [Start command](crate::Create::start()) or any of the cleaning mode commands has
    /// been sent.
    ///
    /// When in Passive mode, sensor data can be requested and received, but actuators (motors,
    /// speaker, lights, low side drivers, digital outputs) cannot be controlled. To control the
    /// actuators the robot must be either in [Full](OperatingMode::Full) or in
    /// [Safe](OperatingMode::Safe) mode.
    Passive,
    /// Safe mode gives full control of the robot, with some safety-related exceptions:
    /// - Detection of a cliff while moving forward (or moving backward with a small turning radius, less than one robot radius).
    /// - Detection of a wheel drop (on any wheel).
    /// - Charger plugged in and powered.
    ///
    /// If any of these conditions occur while in Safe mode, the robot stops all motors and reverts
    /// to [Passive](OperatingMode::Passive) mode.
    ///
    /// Charging terminates when you enter Safe Mode, and the robot will not power save.
    Safe,
    /// Full mode provides full control over the robot.
    ///
    /// The safety features are disabled in Full mode.
    ///
    /// Charging terminates when you enter Full Mode, and Roomba will not power save.
    Full,
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

    pub fn get(&self, day: Day) -> Option<Time> {
        self.slots[day as usize]
    }

    pub fn iter(&self) -> ScheduleIter<'_> {
        ScheduleIter {
            schedule: self,
            index: 0,
        }
    }

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
                    day: (index as u8).try_into().ok()?,
                    time,
                });
            }
        }

        None
    }
}

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
    pub const fn bit(self) -> u8 {
        1 << (self as u8)
    }
}

impl TryFrom<u8> for Day {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    hour: u8,
    minute: u8,
}

impl Time {
    /// Construct a new [`Time`] value.
    ///
    /// Returns [`None`] if the time is invalid.
    pub const fn new(hour: u8, minute: u8) -> Option<Self> {
        if hour > 23 || minute > 59 {
            None
        } else {
            Some(Self { hour, minute })
        }
    }

    pub const fn hour(&self) -> u8 {
        self.hour
    }

    pub const fn minute(&self) -> u8 {
        self.minute
    }
}
