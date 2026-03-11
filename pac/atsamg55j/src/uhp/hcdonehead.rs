#[doc = "Register `HCDONEHEAD` reader"]
pub type R = crate::R<HcdoneheadSpec>;
#[doc = "Field `DH` reader - Physical address of the last TD that has added to the done queue"]
pub type DhR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 4:31 - Physical address of the last TD that has added to the done queue"]
    #[inline(always)]
    pub fn dh(&self) -> DhR {
        DhR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[doc = "HC Head Done Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdonehead::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcdoneheadSpec;
impl crate::RegisterSpec for HcdoneheadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdonehead::R`](R) reader structure"]
impl crate::Readable for HcdoneheadSpec {}
#[doc = "`reset()` method sets HCDONEHEAD to value 0"]
impl crate::Resettable for HcdoneheadSpec {
    const RESET_VALUE: u32 = 0;
}
