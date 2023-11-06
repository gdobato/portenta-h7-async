#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::{main, task, Spawner};
use embassy_time::Timer;
use panic_probe as _;
use portenta_h7_async::led;

#[main]
async fn main(spawner: Spawner) {
    info!("Init");
    let portenta_h7_async::Board {
        led_red,
        led_blue,
        led_green,
        ..
    } = portenta_h7_async::Board::take();

    spawner.spawn(blink_led_red(led_red)).unwrap();
    spawner.spawn(blink_led_green(led_green)).unwrap();
    spawner.spawn(blink_led_blue(led_blue)).unwrap();

    loop {
        Timer::after_millis(100).await;
    }
}

#[task]
async fn blink_led_red(mut led: led::user::Red) {
    loop {
        led.toggle();
        Timer::after_millis(250).await;
    }
}

#[task]
async fn blink_led_green(mut led: led::user::Green) {
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}

#[task]
async fn blink_led_blue(mut led: led::user::Blue) {
    loop {
        led.toggle();
        Timer::after_millis(1_000).await;
    }
}
