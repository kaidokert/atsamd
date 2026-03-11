#[doc = "Register `FILTR` reader"]
pub type R = crate::R<FiltrSpec>;
#[doc = "Register `FILTR` writer"]
pub type W = crate::W<FiltrSpec>;
#[doc = "Field `FILT` reader - RX Digital Filter"]
pub type FiltR = crate::BitReader;
#[doc = "Field `FILT` writer - RX Digital Filter"]
pub type FiltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADFEN` reader - PAD Filter Enable"]
pub type PadfenR = crate::BitReader;
#[doc = "Field `PADFEN` writer - PAD Filter Enable"]
pub type PadfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADFCFG` reader - PAD Filter Config"]
pub type PadfcfgR = crate::BitReader;
#[doc = "Field `PADFCFG` writer - PAD Filter Config"]
pub type PadfcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES` reader - Digital Filter Threshold"]
pub type ThresR = crate::FieldReader;
#[doc = "Field `THRES` writer - Digital Filter Threshold"]
pub type ThresW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FiltR {
        FiltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    pub fn padfen(&self) -> PadfenR {
        PadfenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAD Filter Config"]
    #[inline(always)]
    pub fn padfcfg(&self) -> PadfcfgR {
        PadfcfgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    pub fn thres(&self) -> ThresR {
        ThresR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filt(&mut self) -> FiltW<FiltrSpec> {
        FiltW::new(self, 0)
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn padfen(&mut self) -> PadfenW<FiltrSpec> {
        PadfenW::new(self, 1)
    }
    #[doc = "Bit 2 - PAD Filter Config"]
    #[inline(always)]
    #[must_use]
    pub fn padfcfg(&mut self) -> PadfcfgW<FiltrSpec> {
        PadfcfgW::new(self, 2)
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thres(&mut self) -> ThresW<FiltrSpec> {
        ThresW::new(self, 8)
    }
}
#[doc = "TWI Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`filtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiltrSpec;
impl crate::RegisterSpec for FiltrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filtr::R`](R) reader structure"]
impl crate::Readable for FiltrSpec {}
#[doc = "`write(|w| ..)` method takes [`filtr::W`](W) writer structure"]
impl crate::Writable for FiltrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTR to value 0"]
impl crate::Resettable for FiltrSpec {
    const RESET_VALUE: u32 = 0;
}
