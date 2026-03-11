#[doc = "Register `HCPERIODCURRENTED` reader"]
pub type R = crate::R<HcperiodcurrentedSpec>;
#[doc = "Field `PCED` reader - Physical address of the current ED on the periodic ED list"]
pub type PcedR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 4:31 - Physical address of the current ED on the periodic ED list"]
    #[inline(always)]
    pub fn pced(&self) -> PcedR {
        PcedR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[doc = "HC Current Periodic Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcperiodcurrented::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcperiodcurrentedSpec;
impl crate::RegisterSpec for HcperiodcurrentedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcperiodcurrented::R`](R) reader structure"]
impl crate::Readable for HcperiodcurrentedSpec {}
#[doc = "`reset()` method sets HCPERIODCURRENTED to value 0"]
impl crate::Resettable for HcperiodcurrentedSpec {
    const RESET_VALUE: u32 = 0;
}
