//! Common functions

use crate::{register::Config, Ads1013, Ads1014, Ads1015, Ads1113, Ads1114, Ads1115, Error};

macro_rules! impl_common_features {
    ($Ads:ident) => {
        impl<I2C, E, MODE> $Ads<I2C, MODE>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Read whether a measurement is currently in progress.
            pub fn is_measurement_in_progress(&mut self) -> Result<bool, Error<E>> {
                let config = self.read_reg_u16::<Config>()?;
                Ok(!config.contains(Config::OS))
            }

            /// Resets the internal state of this driver to the default values.
            ///
            /// *Note:* This does not alter the state or configuration of the device.
            ///
            /// This resets the cached configuration register value in this driver to
            /// the power-up (reset) configuration of the device.
            ///
            /// This needs to be called after performing a reset of the device, for
            /// example through an I²C general call reset which was not done
            /// through this driver to ensure that the configurations in the device
            /// and in the driver match.
            pub fn reset_internal_driver_state(&mut self) {
                self.config = Config::default();
            }
        }
    };
}

impl_common_features!(Ads1013);
impl_common_features!(Ads1113);
impl_common_features!(Ads1014);
impl_common_features!(Ads1114);
impl_common_features!(Ads1015);
impl_common_features!(Ads1115);
