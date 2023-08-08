#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_time::{Duration, Timer};

static LED_RED: Mutex<RefCell<Option<Output<'static, embassy_stm32::peripherals::PK5>>>> =
    Mutex::new(RefCell::new(None));
static LED_GREEN: Mutex<RefCell<Option<Output<'static, embassy_stm32::peripherals::PK6>>>> =
    Mutex::new(RefCell::new(None));
static LED_BLUE: Mutex<RefCell<Option<Output<'static, embassy_stm32::peripherals::PK7>>>> =
    Mutex::new(RefCell::new(None));

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let portenta_h7_async::Board {
        led_red,
        led_green,
        led_blue,
    } = portenta_h7_async::Board::take();

    cortex_m::interrupt::free(|cs| {
        LED_RED.borrow(cs).replace(Some(led_red));
        LED_GREEN.borrow(cs).replace(Some(led_green));
        LED_BLUE.borrow(cs).replace(Some(led_blue));
    });

    spawner.spawn(blink_led_red()).unwrap();
    spawner.spawn(blink_led_green()).unwrap();
    spawner.spawn(blink_led_blue()).unwrap();

    loop {
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::task]
async fn blink_led_red() {
    loop {
        cortex_m::interrupt::free(|cs| {
            if let Some(led_red) = LED_RED.borrow(cs).borrow_mut().as_mut() {
                led_red.toggle();
            }
        });
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn blink_led_green() {
    loop {
        cortex_m::interrupt::free(|cs| {
            if let Some(led_green) = LED_GREEN.borrow(cs).borrow_mut().as_mut() {
                led_green.toggle();
            }
        });
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[embassy_executor::task]
async fn blink_led_blue() {
    loop {
        cortex_m::interrupt::free(|cs| {
            if let Some(led_blue) = LED_BLUE.borrow(cs).borrow_mut().as_mut() {
                led_blue.toggle();
            }
        });
        Timer::after(Duration::from_millis(2_000)).await;
    }
}
