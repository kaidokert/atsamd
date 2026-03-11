#[doc = "Register `DSPR1` reader"]
pub type R = crate::R<Dspr1Spec>;
#[doc = "Register `DSPR1` writer"]
pub type W = crate::W<Dspr1Spec>;
#[doc = "Field `DGAIN` reader - Gain Correction"]
pub type DgainR = crate::FieldReader<u16>;
#[doc = "Field `DGAIN` writer - Gain Correction"]
pub type DgainW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `OFFSET` reader - Offset Correction"]
pub type OffsetR = crate::FieldReader<u16>;
#[doc = "Field `OFFSET` writer - Offset Correction"]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:14 - Gain Correction"]
    #[inline(always)]
    pub fn dgain(&self) -> DgainR {
        DgainR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:31 - Offset Correction"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Gain Correction"]
    #[inline(always)]
    #[must_use]
    pub fn dgain(&mut self) -> DgainW<Dspr1Spec> {
        DgainW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Offset Correction"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OffsetW<Dspr1Spec> {
        OffsetW::new(self, 16)
    }
}
#[doc = "DSP Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dspr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dspr1Spec;
impl crate::RegisterSpec for Dspr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dspr1::R`](R) reader structure"]
impl crate::Readable for Dspr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dspr1::W`](W) writer structure"]
impl crate::Writable for Dspr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSPR1 to value 0x01"]
impl crate::Resettable for Dspr1Spec {
    const RESET_VALUE: u32 = 0x01;
}
