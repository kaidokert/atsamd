#[doc = "Register `HCLSTHRESHOLD` reader"]
pub type R = crate::R<HclsthresholdSpec>;
#[doc = "Register `HCLSTHRESHOLD` writer"]
pub type W = crate::W<HclsthresholdSpec>;
#[doc = "Field `LST` reader - Low-speed threshold"]
pub type LstR = crate::FieldReader<u16>;
#[doc = "Field `LST` writer - Low-speed threshold"]
pub type LstW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Low-speed threshold"]
    #[inline(always)]
    pub fn lst(&self) -> LstR {
        LstR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Low-speed threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lst(&mut self) -> LstW<HclsthresholdSpec> {
        LstW::new(self, 0)
    }
}
#[doc = "HC Low-Speed Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hclsthreshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hclsthreshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HclsthresholdSpec;
impl crate::RegisterSpec for HclsthresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hclsthreshold::R`](R) reader structure"]
impl crate::Readable for HclsthresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`hclsthreshold::W`](W) writer structure"]
impl crate::Writable for HclsthresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCLSTHRESHOLD to value 0x0628"]
impl crate::Resettable for HclsthresholdSpec {
    const RESET_VALUE: u32 = 0x0628;
}
