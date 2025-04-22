#![no_std]
#![no_main]

use atsamd_hal::adc::Reference;
use atsamd_hal::adc::Resolution;
use atsamd_hal::adc::SampleRate;
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

use ehal_02::adc::Channel;
use embedded_hal_02 as ehal_02;

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
    adc.resolution(Resolution::_10bit);
    adc.reference(Reference::Int1v);
    adc.samples(SampleRate::_1); // Take 1 sample and filter in software

    let mut temp_channel = TempChannel;
    let mut scaled_iovcc_channel = ScaledIOVccChannel;
    let mut scaled_corevcc_channel = ScaledCoreVccChannel;
    loop {
        // Median filter as it's so noisy
        let mut temp_readings = [0u16; 17];
        for i in temp_readings.iter_mut() {
            *i = adc.read(&mut temp_channel).unwrap();
            delay.delay_us(100u16); // 1 ms delay between readings
        }
        temp_readings.sort_unstable();
        let temp = temp_readings[temp_readings.len() / 2 + 1];
        let iovcc: u16 = adc.read(&mut scaled_iovcc_channel).unwrap();
        let corevcc: u16 = adc.read(&mut scaled_corevcc_channel).unwrap();
        hprintln!("temp: {} iovcc:{} corevcc:{}", temp, iovcc, corevcc).ok();
        delay.delay_ms(100u16);
    }
}
