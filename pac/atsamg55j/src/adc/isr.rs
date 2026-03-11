#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `EOC0` reader - End of Conversion 0 (automatically set / cleared)"]
pub type Eoc0R = crate::BitReader;
#[doc = "Field `EOC1` reader - End of Conversion 1 (automatically set / cleared)"]
pub type Eoc1R = crate::BitReader;
#[doc = "Field `EOC2` reader - End of Conversion 2 (automatically set / cleared)"]
pub type Eoc2R = crate::BitReader;
#[doc = "Field `EOC3` reader - End of Conversion 3 (automatically set / cleared)"]
pub type Eoc3R = crate::BitReader;
#[doc = "Field `EOC4` reader - End of Conversion 4 (automatically set / cleared)"]
pub type Eoc4R = crate::BitReader;
#[doc = "Field `EOC5` reader - End of Conversion 5 (automatically set / cleared)"]
pub type Eoc5R = crate::BitReader;
#[doc = "Field `EOC6` reader - End of Conversion 6 (automatically set / cleared)"]
pub type Eoc6R = crate::BitReader;
#[doc = "Field `EOC7` reader - End of Conversion 7 (automatically set / cleared)"]
pub type Eoc7R = crate::BitReader;
#[doc = "Field `LCCHG` reader - Last Channel Change (cleared on read)"]
pub type LcchgR = crate::BitReader;
#[doc = "Field `DRDY` reader - Data Ready (automatically set / cleared)"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `GOVRE` reader - General Overrun Error (cleared on read)"]
pub type GovreR = crate::BitReader;
#[doc = "Field `COMPE` reader - Comparison Event (cleared on read)"]
pub type CompeR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of Receive Transfer (cleared by writing ADC_RCR or ADC_RNCR)"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Receive Buffer Full (cleared by writing ADC_RCR or ADC_RNCR)"]
pub type RxbuffR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Conversion 0 (automatically set / cleared)"]
    #[inline(always)]
    pub fn eoc0(&self) -> Eoc0R {
        Eoc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Conversion 1 (automatically set / cleared)"]
    #[inline(always)]
    pub fn eoc1(&self) -> Eoc1R {
        Eoc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Conversion 2 (automatically set / cleared)"]
    #[inline(always)]
    pub fn eoc2(&self) -> Eoc2R {
        Eoc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Conversion 3 (automatically set / cleared)"]
    #[inline(always)]
    pub fn eoc3(&self) -> Eoc3R {
        Eoc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Conversion 4 (automatically set / cleared)"]
    #[inline(always)]
    pub fn eoc4(&self) -> Eoc4R {
        Eoc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion 5 (automatically set / cleared)"]
    #[inline(always)]
    pub fn eoc5(&self) -> Eoc5R {
        Eoc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Conversion 6 (automatically set / cleared)"]
    #[inline(always)]
    pub fn eoc6(&self) -> Eoc6R {
        Eoc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of Conversion 7 (automatically set / cleared)"]
    #[inline(always)]
    pub fn eoc7(&self) -> Eoc7R {
        Eoc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 19 - Last Channel Change (cleared on read)"]
    #[inline(always)]
    pub fn lcchg(&self) -> LcchgR {
        LcchgR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Data Ready (automatically set / cleared)"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - General Overrun Error (cleared on read)"]
    #[inline(always)]
    pub fn govre(&self) -> GovreR {
        GovreR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Comparison Event (cleared on read)"]
    #[inline(always)]
    pub fn compe(&self) -> CompeR {
        CompeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End of Receive Transfer (cleared by writing ADC_RCR or ADC_RNCR)"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Receive Buffer Full (cleared by writing ADC_RCR or ADC_RNCR)"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
