#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `RXEND` reader - End of Transfer"]
pub type RxendR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Buffer Full"]
pub type RxbuffR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Transfer"]
    #[inline(always)]
    pub fn rxend(&self) -> RxendR {
        RxendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Memory to Memory Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
