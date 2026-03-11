#[doc = "Register `RAB2` reader"]
pub type R = crate::R<Rab2Spec>;
#[doc = "Field `RAB` reader - Register A or Register B"]
pub type RabR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register A or Register B"]
    #[inline(always)]
    pub fn rab(&self) -> RabR {
        RabR::new(self.bits)
    }
}
#[doc = "Register AB (channel = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`rab2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rab2Spec;
impl crate::RegisterSpec for Rab2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rab2::R`](R) reader structure"]
impl crate::Readable for Rab2Spec {}
#[doc = "`reset()` method sets RAB2 to value 0"]
impl crate::Resettable for Rab2Spec {
    const RESET_VALUE: u32 = 0;
}
