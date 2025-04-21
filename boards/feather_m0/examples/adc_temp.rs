#![no_std]
#![no_main]

use atsamd_hal::pac::adc::inputctrl::Muxposselect;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use cortex_m_semihosting::hprintln;

use bsp::hal;
use bsp::pac;
use feather_m0 as bsp;

use bsp::entry;
use hal::adc::Adc;
use hal::clock::GenericClockController;
use hal::prelude::*;
use pac::{CorePeripherals, Peripherals};

use embedded_hal_02 as ehal_02;
use ehal_02::adc::Channel;

struct TempChannel;
struct ScaledIOVccChannel;
struct ScaledCoreVccChannel;

impl Channel<pac::Adc> for TempChannel {
    type ID = u8;
    fn channel() -> u8 {
        Muxposselect::Temp.into()
    }
}
impl Channel<pac::Adc> for ScaledIOVccChannel {
    type ID = u8;
    fn channel() -> u8 {
        Muxposselect::Scalediovcc.into()
    }
}
impl Channel<pac::Adc> for ScaledCoreVccChannel {
    type ID = u8;
    fn channel() -> u8 {
        Muxposselect::Scaledcorevcc.into()
    }
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let mut adc = Adc::adc(peripherals.adc, &mut peripherals.pm, &mut clocks);
    let mut channel = TempChannel;
    loop {
        let data: u16 = adc.read(&mut channel).unwrap();
        hprintln!("{}", data).ok();
        delay.delay_ms(1000u16);
    }
}
