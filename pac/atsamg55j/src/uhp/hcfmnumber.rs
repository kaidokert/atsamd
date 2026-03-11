#[doc = "Register `HCFMNUMBER` reader"]
pub type R = crate::R<HcfmnumberSpec>;
#[doc = "Field `FN` reader - Frame number"]
pub type FnR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame number"]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "HC Frame Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfmnumber::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcfmnumberSpec;
impl crate::RegisterSpec for HcfmnumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfmnumber::R`](R) reader structure"]
impl crate::Readable for HcfmnumberSpec {}
#[doc = "`reset()` method sets HCFMNUMBER to value 0"]
impl crate::Resettable for HcfmnumberSpec {
    const RESET_VALUE: u32 = 0;
}
