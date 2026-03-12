use core::convert::Infallible;
use core::marker::PhantomData;

use crate::ehal::digital::{ErrorType, OutputPin};
use crate::pac;

pub struct Output;

pub struct Pins {
    pub pa06: Pa06<Output>,
    pub pa15: Pa15<Output>,
}

impl Pins {
    pub fn new(pmc: pac::Pmc, _pioa: pac::Pioa) -> Self {
        enable_pioa_clock(&pmc);
        configure_led_pins();

        Self {
            pa06: Pa06 { _mode: PhantomData },
            pa15: Pa15 { _mode: PhantomData },
        }
    }
}

pub struct Pa06<M = Output> {
    _mode: PhantomData<M>,
}

pub struct Pa15<M = Output> {
    _mode: PhantomData<M>,
}

impl ErrorType for Pa06<Output> {
    type Error = Infallible;
}

impl ErrorType for Pa15<Output> {
    type Error = Infallible;
}

impl OutputPin for Pa06<Output> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        // The board LED is active-low.
        unsafe {
            pioa().codr().write_with_zero(|w| w.p6().set_bit());
        }
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            pioa().sodr().write_with_zero(|w| w.p6().set_bit());
        }
        Ok(())
    }
}

impl OutputPin for Pa15<Output> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        // The board LED is active-low.
        unsafe {
            pioa().codr().write_with_zero(|w| w.p15().set_bit());
        }
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            pioa().sodr().write_with_zero(|w| w.p15().set_bit());
        }
        Ok(())
    }
}

fn enable_pioa_clock(pmc: &pac::Pmc) {
    // PIOA is peripheral ID 11 on ATSAMG55.
    unsafe {
        pmc.pmc_pcer0().write_with_zero(|w| w.pid11().set_bit());
    }
}

fn configure_led_pins() {
    unsafe {
        pioa()
            .per()
            .write_with_zero(|w| w.p6().set_bit().p15().set_bit());
        pioa()
            .oer()
            .write_with_zero(|w| w.p6().set_bit().p15().set_bit());
        pioa()
            .sodr()
            .write_with_zero(|w| w.p6().set_bit().p15().set_bit());
    }
}

fn pioa() -> &'static pac::pioa::RegisterBlock {
    unsafe { &*pac::Pioa::ptr() }
}
