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
