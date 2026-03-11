#[doc = "Register `RAB0` reader"]
pub type R = crate::R<Rab0Spec>;
#[doc = "Field `RAB` reader - Register A or Register B"]
pub type RabR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register A or Register B"]
    #[inline(always)]
    pub fn rab(&self) -> RabR {
        RabR::new(self.bits)
    }
}
#[doc = "Register AB (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rab0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rab0Spec;
impl crate::RegisterSpec for Rab0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rab0::R`](R) reader structure"]
impl crate::Readable for Rab0Spec {}
#[doc = "`reset()` method sets RAB0 to value 0"]
impl crate::Resettable for Rab0Spec {
    const RESET_VALUE: u32 = 0;
}
