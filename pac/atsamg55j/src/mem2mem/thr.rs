#[doc = "Register `THR` reader"]
pub type R = crate::R<ThrSpec>;
#[doc = "Register `THR` writer"]
pub type W = crate::W<ThrSpec>;
#[doc = "Field `THDATA` reader - Transfer Holding Data"]
pub type ThdataR = crate::FieldReader<u32>;
#[doc = "Field `THDATA` writer - Transfer Holding Data"]
pub type ThdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transfer Holding Data"]
    #[inline(always)]
    pub fn thdata(&self) -> ThdataR {
        ThdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transfer Holding Data"]
    #[inline(always)]
    #[must_use]
    pub fn thdata(&mut self) -> ThdataW<ThrSpec> {
        ThdataW::new(self, 0)
    }
}
#[doc = "Memory to Memory Transfer Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThrSpec;
impl crate::RegisterSpec for ThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thr::R`](R) reader structure"]
impl crate::Readable for ThrSpec {}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for ThrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for ThrSpec {
    const RESET_VALUE: u32 = 0;
}
