#[doc = "Register `CMPR` reader"]
pub type R = crate::R<CmprSpec>;
#[doc = "Register `CMPR` writer"]
pub type W = crate::W<CmprSpec>;
#[doc = "Field `VAL1` reader - First Comparison Value for Received Character"]
pub type Val1R = crate::FieldReader<u16>;
#[doc = "Field `VAL1` writer - First Comparison Value for Received Character"]
pub type Val1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VAL2` reader - Second Comparison Value for Received Character"]
pub type Val2R = crate::FieldReader<u16>;
#[doc = "Field `VAL2` writer - Second Comparison Value for Received Character"]
pub type Val2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&self) -> Val1R {
        Val1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&self) -> Val2R {
        Val2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - First Comparison Value for Received Character"]
    #[inline(always)]
    #[must_use]
    pub fn val1(&mut self) -> Val1W<CmprSpec> {
        Val1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Second Comparison Value for Received Character"]
    #[inline(always)]
    #[must_use]
    pub fn val2(&mut self) -> Val2W<CmprSpec> {
        Val2W::new(self, 16)
    }
}
#[doc = "SPI Comparison Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmprSpec;
impl crate::RegisterSpec for CmprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpr::R`](R) reader structure"]
impl crate::Readable for CmprSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpr::W`](W) writer structure"]
impl crate::Writable for CmprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPR to value 0"]
impl crate::Resettable for CmprSpec {
    const RESET_VALUE: u32 = 0;
}
