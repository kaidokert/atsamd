#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `ALMS` reader - Real-time Alarm Status (cleared on read)"]
pub type AlmsR = crate::BitReader;
#[doc = "Field `RTTINC` reader - Prescaler Roll-over Status (cleared on read)"]
pub type RttincR = crate::BitReader;
#[doc = "Field `RTTINC2` reader - Predefined Number of Prescaler Roll-over Status (cleared on read)"]
pub type Rttinc2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Real-time Alarm Status (cleared on read)"]
    #[inline(always)]
    pub fn alms(&self) -> AlmsR {
        AlmsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prescaler Roll-over Status (cleared on read)"]
    #[inline(always)]
    pub fn rttinc(&self) -> RttincR {
        RttincR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Predefined Number of Prescaler Roll-over Status (cleared on read)"]
    #[inline(always)]
    pub fn rttinc2(&self) -> Rttinc2R {
        Rttinc2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
