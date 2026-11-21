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
