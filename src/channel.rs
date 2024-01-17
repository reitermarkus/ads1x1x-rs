//! ADC input channels.

use core::fmt::Debug;

use crate::{Ads1013, Ads1014, Ads1015, Ads1113, Ads1114, Ads1115, Config, Error};

use private::ChannelSelection;

/// Marker type for an ADC input channel.
pub trait ChannelId<T>: private::Sealed {
    /// Get the channel.
    fn channel_id() -> ChannelSelection;
}

macro_rules! impl_channels {
    ($(#[doc = $doc:expr] $CH:ident => [$($Ads:ident),+]),+ $(,)?) => {
        mod private {
            pub trait Sealed {}

            #[derive(Debug, Clone, Copy)]
            /// ADC input channel selection.
            pub enum ChannelSelection {
                $(
                    #[doc = $doc]
                    $CH,
                )+
            }
        }

        $(
            #[doc = $doc]
            pub struct $CH;

            impl private::Sealed for $CH {}

            $(
                impl<I2C, MODE> ChannelId<$Ads<I2C, MODE>> for $CH {
                    fn channel_id() -> ChannelSelection {
                        ChannelSelection::$CH
                    }
                }
            )+
        )+
    };
}

impl_channels!(
    /// Measure signal on input channel 0 differentially to signal on input channel 1.
    DifferentialA0A1 => [Ads1013, Ads1014, Ads1015, Ads1113, Ads1114,  Ads1115],
    /// Measure signal on input channel 0 differentially to signal on input channel 3.
    DifferentialA0A3 => [Ads1015, Ads1115],
    /// Measure signal on input channel 1 differentially to signal on input channel 3.
    DifferentialA1A3 => [Ads1015, Ads1115],
    /// Measure signal on input channel 3 differentially to signal on input channel 3.
    DifferentialA2A3 => [Ads1015, Ads1115],
    /// Measure single-ended signal on input channel 0.
    SingleA0 => [Ads1015, Ads1115],
    /// Measure single-ended signal on input channel 1.
    SingleA1 => [Ads1015, Ads1115],
    /// Measure single-ended signal on input channel 2.
    SingleA2 => [Ads1015, Ads1115],
    /// Measure single-ended signal on input channel 3.
    SingleA3 => [Ads1015, Ads1115]
);

impl Config {
    pub(crate) fn with_mux_bits(&self, ch: ChannelSelection) -> Self {
        match ch {
            ChannelSelection::DifferentialA0A1 => self
                .difference(Self::MUX2)
                .difference(Self::MUX1)
                .difference(Self::MUX0),
            ChannelSelection::DifferentialA0A3 => self
                .difference(Self::MUX2)
                .difference(Self::MUX1)
                .union(Self::MUX0),
            ChannelSelection::DifferentialA1A3 => self
                .difference(Self::MUX2)
                .union(Self::MUX1)
                .difference(Self::MUX0),
            ChannelSelection::DifferentialA2A3 => self
                .difference(Self::MUX2)
                .union(Self::MUX1)
                .union(Self::MUX0),
            ChannelSelection::SingleA0 => self
                .union(Self::MUX2)
                .difference(Self::MUX1)
                .difference(Self::MUX0),
            ChannelSelection::SingleA1 => self
                .union(Self::MUX2)
                .difference(Self::MUX1)
                .union(Self::MUX0),
            ChannelSelection::SingleA2 => self
                .union(Self::MUX2)
                .union(Self::MUX1)
                .difference(Self::MUX0),
            ChannelSelection::SingleA3 => {
                self.union(Self::MUX2).union(Self::MUX1).union(Self::MUX0)
            }
        }
    }
}

/// A channel used for measurement.
pub struct Channel<ADC> {
    pub(crate) adc: ADC,
}

macro_rules! impl_channel {
    ($Ads:ident, $conv:ty) => {
        impl<I2C, MODE> Channel<$Ads<I2C, MODE>> {
            /// Releases the contained ADS1x1x.
            pub fn release(self) -> $Ads<I2C, MODE> {
                self.adc
            }
        }

        impl<I2C, E, MODE> $Ads<I2C, MODE>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Creates an owned a channel to use for measurement.
            ///
            /// **Note:** When changing the channel in continuous conversion mode,
            /// the ongoing conversion will be completed. The following conversions
            /// will use the new channel configuration.
            pub fn into_channel<CH: ChannelId<Self>>(mut self) -> Result<Channel<Self>, Error<E>> {
                // Change channel if necessary.
                let config = self.config.with_mux_bits(CH::channel_id());
                if self.config != config {
                    self.write_reg_u16(config)?;
                    self.config = config;
                }

                Ok(Channel { adc: self })
            }

            /// Borrows a channel to use for measurement.
            ///
            /// **Note:** When changing the channel in continuous conversion mode,
            /// the ongoing conversion will be completed. The following conversions
            /// will use the new channel configuration.
            pub fn channel<CH: ChannelId<Self>>(&mut self) -> Result<Channel<&mut Self>, Error<E>> {
                // Change channel if necessary.
                let config = self.config.with_mux_bits(CH::channel_id());
                if self.config != config {
                    self.write_reg_u16(config)?;
                    self.config = config;
                }

                Ok(Channel { adc: self })
            }
        }

        impl<I2C, E, MODE> embedded_hal::adc::ErrorType for Channel<$Ads<I2C, MODE>>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
            E: Debug,
        {
            type Error = Error<E>;
        }

        impl<I2C, E, MODE> embedded_hal::adc::ErrorType for Channel<&mut $Ads<I2C, MODE>>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
            E: Debug,
        {
            type Error = Error<E>;
        }
    };
}

impl_channel!(Ads1013, Conversion12);
impl_channel!(Ads1014, Conversion12);
impl_channel!(Ads1015, Conversion12);
impl_channel!(Ads1113, Conversion16);
impl_channel!(Ads1114, Conversion16);
impl_channel!(Ads1115, Conversion16);
