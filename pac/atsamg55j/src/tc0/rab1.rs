#[doc = "Register `RAB1` reader"]
pub type R = crate::R<Rab1Spec>;
#[doc = "Field `RAB` reader - Register A or Register B"]
pub type RabR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register A or Register B"]
    #[inline(always)]
    pub fn rab(&self) -> RabR {
        RabR::new(self.bits)
    }
}
#[doc = "Register AB (channel = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`rab1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rab1Spec;
impl crate::RegisterSpec for Rab1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rab1::R`](R) reader structure"]
impl crate::Readable for Rab1Spec {}
#[doc = "`reset()` method sets RAB1 to value 0"]
impl crate::Resettable for Rab1Spec {
    const RESET_VALUE: u32 = 0;
}
