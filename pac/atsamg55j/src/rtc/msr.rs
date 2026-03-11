#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Field `MS` reader - Number of 1/1024 seconds elapsed within 1 second"]
pub type MsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Number of 1/1024 seconds elapsed within 1 second"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Milliseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MsrSpec {
    const RESET_VALUE: u32 = 0;
}
