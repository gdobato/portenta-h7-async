#![no_std]
#![feature(type_alias_impl_trait)]

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

#[cfg(feature="usb")]
use embassy_stm32::usb_otg::{self, Driver};

#[cfg(not(feature="usb"))]
bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    UART4 => usart::InterruptHandler<peripherals::UART4>;
});

#[cfg(feature="usb")]
bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    UART4 => usart::InterruptHandler<peripherals::UART4>;
    OTG_HS => usb_otg::InterruptHandler<peripherals::USB_OTG_HS>;
});

// Naming according to breakout board
pub type Uart0 = Uart<'static, peripherals::UART4, peripherals::DMA1_CH0, peripherals::DMA1_CH1>;
pub type Uart1 = Uart<'static, peripherals::USART1, peripherals::DMA1_CH2, peripherals::DMA1_CH3>;
#[cfg(feature="usb")]
pub type Usb = Driver<'static, peripherals::USB_OTG_HS>;


pub struct Board {
    pub led_red: Output<'static, peripherals::PK5>,
    pub led_green: Output<'static, peripherals::PK6>,
    pub led_blue: Output<'static, peripherals::PK7>,
    #[cfg(not(feature="usb"))]
    pub uart0: Uart0,
    pub uart1: Uart1,
    #[cfg(feature="usb")]
    pub usb: Usb,
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
        let mut config = Config::default();

        {
            use embassy_stm32::rcc::*;
            config.rcc.hsi48 = Some(Hsi48Config {
                sync_from_usb: true,
            });
        }

        let p = embassy_stm32::init(config);

        // User leds
        let led_red = Output::new(p.PK5, Level::High, Speed::Low);
        let led_green = Output::new(p.PK6, Level::High, Speed::Low);
        let led_blue = Output::new(p.PK7, Level::High, Speed::Low);

        #[cfg(not(feature="usb"))]
        let uart0 = Uart::new_with_rtscts(
            p.UART4,
            p.PI9,
            p.PA0,
            Irqs,
            p.PA15,
            p.PB0,
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
            p.PA12,
            p.PA11,
            p.DMA1_CH2,
            p.DMA1_CH3,
            Default::default(),
        )
        .unwrap();

        #[cfg(feature="usb")]
        let usb;

        #[cfg(feature="usb")] 
        {
            use static_cell::make_static;
            
            // Create the USB driver, from the HAL.
            let mut config = embassy_stm32::usb_otg::Config::default();
            config.vbus_detection = true;
            usb = Driver::new_hs_ulpi(
                p.USB_OTG_HS,
                Irqs,
                p.PA5,
                p.PI11,
                p.PH4,
                p.PC0,
                p.PA3,
                p.PB0,
                p.PB1,
                p.PB10,
                p.PB11,
                p.PB12,
                p.PB13,
                p.PB5,
                &mut make_static!([0; 1024])[..],
                config,
            );
        }

        Self {
            led_red,
            led_green,
            led_blue,
            #[cfg(not(feature="usb"))]
            uart0,
            uart1,
            #[cfg(feature="usb")]
            usb
        }
    }
}
