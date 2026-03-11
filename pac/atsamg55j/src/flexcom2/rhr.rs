#[doc = "Register `RHR` reader"]
pub type R = crate::R<RhrSpec>;
#[doc = "Field `RXDATA` reader - Receive Data"]
pub type RxdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "FLEXCOM Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rhr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RhrSpec;
impl crate::RegisterSpec for RhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rhr::R`](R) reader structure"]
impl crate::Readable for RhrSpec {}
#[doc = "`reset()` method sets RHR to value 0"]
impl crate::Resettable for RhrSpec {
    const RESET_VALUE: u32 = 0;
}
