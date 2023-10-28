#![no_std]

pub mod led;
pub mod panic;
mod sys;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    peripherals,
};

use core::sync::atomic::{AtomicBool, Ordering};

pub struct Board<'a> {
    pub led_red: Output<'a, peripherals::PK5>,
    pub led_green: Output<'a, peripherals::PK6>,
    pub led_blue: Output<'a, peripherals::PK7>,
}

impl<'a> Board<'a> {
    pub fn take() -> Self {
        static TAKEN: AtomicBool = AtomicBool::new(false);
        debug_assert!(!TAKEN.swap(true, Ordering::SeqCst));
        Self::setup()
    }

    pub fn setup() -> Self {
        sys::Clk::new().reset().enable_ext_clock();
        let p = embassy_stm32::init(Default::default());
        let led_red = Output::new(p.PK5, Level::High, Speed::Low);
        let led_green = Output::new(p.PK6, Level::High, Speed::Low);
        let led_blue = Output::new(p.PK7, Level::High, Speed::Low);

        Self {
            led_red,
            led_green,
            led_blue,
        }
    }
}
