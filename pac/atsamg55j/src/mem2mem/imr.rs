#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `RXEND` reader - End of Transfer Interrupt Mask"]
pub type RxendR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Buffer Full Interrupt Mask"]
pub type RxbuffR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Transfer Interrupt Mask"]
    #[inline(always)]
    pub fn rxend(&self) -> RxendR {
        RxendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Memory to Memory Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
