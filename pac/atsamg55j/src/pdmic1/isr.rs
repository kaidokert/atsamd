#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `FIFOCNT` reader - FIFO Count"]
pub type FifocntR = crate::FieldReader;
#[doc = "Field `DRDY` reader - Data Ready"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error"]
pub type OvreR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of RX Buffer"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - RX Buffer Full"]
pub type RxbuffR = crate::BitReader;
impl R {
    #[doc = "Bits 16:23 - FIFO Count"]
    #[inline(always)]
    pub fn fifocnt(&self) -> FifocntR {
        FifocntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - End of RX Buffer"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RX Buffer Full"]
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
