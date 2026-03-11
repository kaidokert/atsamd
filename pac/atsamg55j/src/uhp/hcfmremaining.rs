#[doc = "Register `HCFMREMAINING` reader"]
pub type R = crate::R<HcfmremainingSpec>;
#[doc = "Field `FR` reader - Frame remaining"]
pub type FrR = crate::FieldReader<u16>;
#[doc = "Field `FRT` reader - Frame remaining toggle"]
pub type FrtR = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - Frame remaining"]
    #[inline(always)]
    pub fn fr(&self) -> FrR {
        FrR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Frame remaining toggle"]
    #[inline(always)]
    pub fn frt(&self) -> FrtR {
        FrtR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "HC Frame Remaining Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfmremaining::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcfmremainingSpec;
impl crate::RegisterSpec for HcfmremainingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfmremaining::R`](R) reader structure"]
impl crate::Readable for HcfmremainingSpec {}
#[doc = "`reset()` method sets HCFMREMAINING to value 0"]
impl crate::Resettable for HcfmremainingSpec {
    const RESET_VALUE: u32 = 0;
}
