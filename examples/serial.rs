#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::{main, task, Spawner};
use embassy_time::Timer;
use panic_probe as _;
use portenta_h7_async::{Uart0, Uart1};

const DATA: &[u8] = b"Hello world!";

#[main]
async fn main(spawner: Spawner) {
    info!("Init");
    let portenta_h7_async::Board { uart0, uart1, .. } = portenta_h7_async::Board::take();

    spawner.spawn(uart_write(uart0)).unwrap();
    spawner.spawn(uart_read(uart1)).unwrap();

    loop {
        Timer::after_millis(100).await;
    }
}

#[task]
async fn uart_write(mut uart: Uart0) {
    loop {
        uart.blocking_write(DATA).unwrap(); // TODO Use async version, it panics, fix it
        Timer::after_millis(1_000).await;
    }
}

#[task]
async fn uart_read(mut uart: Uart1) {
    let rx_buf: &mut [u8] = &mut [0; DATA.len()];
    loop {
        Timer::after_millis(1_000).await;
        uart.blocking_read(rx_buf).unwrap(); // TODO Use async version, it panics, fix it

        info!("{}", core::str::from_utf8(&rx_buf).unwrap());
    }
}
