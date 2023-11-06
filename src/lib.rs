#![no_std]

pub mod led;
mod sys;
use core::sync::atomic::{AtomicBool, Ordering};

use embassy_stm32::{
    bind_interrupts,
    gpio::{Level, Output, Speed},
    peripherals,
    usart::{self, Uart},
    Config,
};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    UART4 => usart::InterruptHandler<peripherals::UART4>;
});

// Naming according to breakout board
pub type Uart0 = Uart<'static, peripherals::UART4, peripherals::DMA1_CH0, peripherals::DMA1_CH1>;
pub type Uart1 = Uart<'static, peripherals::USART1, peripherals::DMA1_CH2, peripherals::DMA1_CH3>;

pub struct Board {
    pub led_red: Output<'static, peripherals::PK5>,
    pub led_green: Output<'static, peripherals::PK6>,
    pub led_blue: Output<'static, peripherals::PK7>,
    pub uart0: Uart0,
    pub uart1: Uart1,
}

impl Board {
    pub fn take() -> Self {
        static TAKEN: AtomicBool = AtomicBool::new(false);
        debug_assert!(!TAKEN.swap(true, Ordering::SeqCst));
        Self::setup()
    }

    pub fn setup() -> Self {
        sys::Clk::new().reset().enable_ext_clock();
        // TODO Configure 480 MHz (sys) and 240 MHz (per)
        let config = Config::default();
        let p = embassy_stm32::init(config);

        // User leds
        let led_red = Output::new(p.PK5, Level::High, Speed::Low);
        let led_green = Output::new(p.PK6, Level::High, Speed::Low);
        let led_blue = Output::new(p.PK7, Level::High, Speed::Low);

        // Uart0 of breakout board
        let uart0 = Uart::new_with_rtscts(
            p.UART4,
            p.PI9,
            p.PA0,
            Irqs,
            p.PA15, // TODO Set PI10
            p.PB0,  // TODO Set PI13
            p.DMA1_CH0,
            p.DMA1_CH1,
            Default::default(),
        )
        .unwrap();

        // Uart1 of breakout board
        let uart1 = Uart::new_with_rtscts(
            p.USART1,
            p.PA10,
            p.PA9,
            Irqs,
            p.PA12, // TODO Set PI14
            p.PA11, // TODO Set PI15
            p.DMA1_CH2,
            p.DMA1_CH3,
            Default::default(),
        )
        .unwrap();

        Self {
            led_red,
            led_green,
            led_blue,
            uart0,
            uart1,
        }
    }
}
