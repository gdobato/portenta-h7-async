//! led

use embassy_stm32::{
    gpio::{Output, Pin},
    peripherals,
};

pub trait Led {
    fn on(&mut self);
    fn off(&mut self);
    fn toggle(&mut self);
}

pub mod user {
    use super::*;

    pub type Red = Output<'static, peripherals::PK5>;
    pub type Green = Output<'static, peripherals::PK6>;
    pub type Blue = Output<'static, peripherals::PK7>;

    // Marker trait
    trait BoardLed {}
    impl BoardLed for Red {}
    impl BoardLed for Green {}
    impl BoardLed for Blue {}

    impl<T> Led for Output<'static, T>
    where
        T: Pin,
        Output<'static, T>: BoardLed,
    {
        #[inline]
        fn on(&mut self) {
            self.set_low();
        }

        #[inline]
        fn off(&mut self) {
            self.set_high();
        }

        #[inline]
        fn toggle(&mut self) {
            self.toggle();
        }
    }
}
