use core::fmt::{self, Debug, Display};

/// An ADC error.
#[derive(Debug, Clone)]
pub enum Error<BUS> {
    /// A bus error.
    Bus(BUS),
}

impl<BUS: Debug> embedded_hal::adc::Error for Error<BUS> {
    fn kind(&self) -> embedded_hal::adc::ErrorKind {
        match self {
            Self::Bus(_) => embedded_hal::adc::ErrorKind::Other,
        }
    }
}

// impl<BUS: Debug> embedded_hal::i2c::Error for Error<BUS>
// where
//     BUS: embedded_hal::i2c::Error,
// {
//     fn kind(&self) -> embedded_hal::i2c::ErrorKind {
//         match self {
//             Self::Bus(err) => err.kind(),
//         }
//     }
// }

impl<BUS: Display> Display for Error<BUS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bus(err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl<BUS: Display + Debug> std::error::Error for Error<BUS> {}
