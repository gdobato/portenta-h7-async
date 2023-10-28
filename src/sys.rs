//! sys
//!
//! Clear up previous clock initialization done in bootloader
//! Enable external oscillator for HSE sourcing (25 MHz)
//!

#![allow(dead_code)]

use cortex_m::asm;
use embassy_stm32::pac::{self, gpio, rcc};

pub struct Unreset;
pub struct Reset;

pub struct Clk<State> {
    _state: State,
}

pub type ClkSource = pac::rcc::vals::Sw;
pub type PllSource = pac::rcc::vals::Pllsrc;

impl Clk<Unreset> {
    pub fn new() -> Clk<Unreset> {
        Clk { _state: Unreset }
    }

    pub fn get_source() -> ClkSource {
        pac::RCC.cfgr().read().sws()
    }

    pub fn get_pll_source() -> PllSource {
        pac::RCC.pllckselr().read().pllsrc()
    }

    pub fn reset(self) -> Clk<Reset> {
        let rcc = pac::RCC;

        // Enable HSI and load reset values
        rcc.cr().modify(|w| w.set_hsion(true));
        rcc.hsicfgr().modify(|w| w.set_hsitrim(0b100000));

        // Reset clock configuration and wait for clock switch
        rcc.cfgr().write_value(Default::default());
        while rcc.cfgr().read() != Default::default() {}

        // Reset CSI, HSE, HSI48 and dividers
        rcc.cr().modify(|w| {
            w.set_hsikeron(false);
            w.set_hsidiv(rcc::vals::Hsidiv::DIV1);
            w.set_hsidivf(false);
            w.set_csion(false);
            w.set_csikeron(false);
            w.set_hsi48on(false);
            w.set_hsecsson(false);
            w.set_hsebyp(false);
        });

        // Disable PLL1, PLL2, PLL3
        for n in 0..3usize {
            rcc.cr().modify(|w| w.set_pllon(n, false));
            while rcc.cr().read().pllon(n) {}
        }

        // Reset domain configurations
        rcc.d1cfgr().write_value(Default::default());
        rcc.d2cfgr().write_value(Default::default());
        rcc.d3cfgr().write_value(Default::default());

        // Reset PLLs configurations
        for n in 0..3usize {
            rcc.pllckselr()
                .write(|w| w.set_divm(n, rcc::vals::Pllm::DIV32));
            rcc.plldivr(n)
                .write(|w| w.set_pllr(rcc::vals::Plldiv::DIV2));
            rcc.pllfracr(n).write(|w| w.set_fracn(0x00));
        }
        Clk { _state: Reset }
    }
}

impl Clk<Reset> {
    pub fn enable_ext_clock(self) -> Clk<Reset> {
        let rcc = pac::RCC;
        // Enable GPIOH clock
        rcc.ahb4enr().modify(|w| w.set_gpiohen(true));

        // Enable oscilator via push pulled GPIOH_1 output
        let gpioh = pac::GPIOH;
        gpioh.bsrr().write(|w| w.set_bs(1, true));
        gpioh
            .moder()
            .modify(|w| w.set_moder(1, gpio::vals::Moder::OUTPUT));
        gpioh
            .otyper()
            .modify(|w| w.set_ot(1, gpio::vals::Ot::PUSHPULL));
        gpioh
            .ospeedr()
            .modify(|w| w.set_ospeedr(1, gpio::vals::Ospeedr::LOWSPEED));
        gpioh
            .pupdr()
            .modify(|w| w.set_pupdr(1, gpio::vals::Pupdr::PULLUP));

        asm::delay(1_000);
        Clk { _state: Reset }
    }
}
