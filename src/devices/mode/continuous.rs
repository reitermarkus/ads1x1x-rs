//! Continuous measurement mode.

use core::{fmt::Debug, marker::PhantomData};

use crate::{
    mode,
    register::{Config, Conversion12, Conversion16},
    Ads1013, Ads1014, Ads1015, Ads1113, Ads1114, Ads1115, Channel, Error, FullScaleRange,
};

macro_rules! impl_continuous {
    ($Ads:ident, $conv:ty) => {
        impl<I2C, E> $Ads<I2C, mode::Continuous>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Changes to one-shot operating mode.
            ///
            /// On error, returns a pair of the error and the current instance.
            pub fn into_one_shot(mut self) -> Result<$Ads<I2C, mode::OneShot>, (Error<E>, Self)> {
                let config = self.config.union(Config::MODE);
                if let Err(e) = self.write_reg_u16(config) {
                    return Err((e, self));
                }
                Ok($Ads {
                    i2c: self.i2c,
                    address: self.address,
                    config,
                    mode: PhantomData,
                })
            }

            pub(crate) fn measure_raw(&mut self) -> Result<$conv, Error<E>> {
                self.read_reg_u16::<$conv>()
            }

            /// Reads the most recent measurement.
            pub fn measure(&mut self) -> Result<i16, Error<E>> {
                Ok(self.read_reg_u16::<$conv>()?.convert_measurement())
            }
        }

        impl<I2C, E> embedded_hal::adc::Voltmeter for Channel<$Ads<I2C, mode::Continuous>>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
            E: embedded_hal::i2c::Error + Debug,
        {
            fn measure_nv(&mut self) -> Result<i64, Self::Error> {
                Ok(self
                    .adc
                    .measure_raw()?
                    .nv(FullScaleRange::from(self.adc.config)))
            }

            fn measure_uv(&mut self) -> Result<i32, Self::Error> {
                Ok(self
                    .adc
                    .measure_raw()?
                    .uv(FullScaleRange::from(self.adc.config)))
            }

            fn measure_mv(&mut self) -> Result<i16, Self::Error> {
                Ok(self
                    .adc
                    .measure_raw()?
                    .mv(FullScaleRange::from(self.adc.config)))
            }
        }

        impl<I2C, E> embedded_hal::adc::Voltmeter for Channel<&mut $Ads<I2C, mode::Continuous>>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
            E: embedded_hal::i2c::Error + Debug,
        {
            fn measure_nv(&mut self) -> Result<i64, Self::Error> {
                Ok(self
                    .adc
                    .measure_raw()?
                    .nv(FullScaleRange::from(self.adc.config)))
            }

            fn measure_uv(&mut self) -> Result<i32, Self::Error> {
                Ok(self
                    .adc
                    .measure_raw()?
                    .uv(FullScaleRange::from(self.adc.config)))
            }

            fn measure_mv(&mut self) -> Result<i16, Self::Error> {
                Ok(self
                    .adc
                    .measure_raw()?
                    .mv(FullScaleRange::from(self.adc.config)))
            }
        }
    };
}

impl_continuous!(Ads1013, Conversion12);
impl_continuous!(Ads1014, Conversion12);
impl_continuous!(Ads1015, Conversion12);
impl_continuous!(Ads1113, Conversion16);
impl_continuous!(Ads1114, Conversion16);
impl_continuous!(Ads1115, Conversion16);
