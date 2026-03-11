#[doc = "Register `PMC_OCR` reader"]
pub type R = crate::R<PmcOcrSpec>;
#[doc = "Register `PMC_OCR` writer"]
pub type W = crate::W<PmcOcrSpec>;
#[doc = "Field `CAL8` reader - RC Oscillator Calibration bits for 8 MHz"]
pub type Cal8R = crate::FieldReader;
#[doc = "Field `CAL8` writer - RC Oscillator Calibration bits for 8 MHz"]
pub type Cal8W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL8` reader - Selection of RC Oscillator Calibration bits for 8 MHz"]
pub type Sel8R = crate::BitReader;
#[doc = "Field `SEL8` writer - Selection of RC Oscillator Calibration bits for 8 MHz"]
pub type Sel8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL16` reader - RC Oscillator Calibration bits for 16 MHz"]
pub type Cal16R = crate::FieldReader;
#[doc = "Field `CAL16` writer - RC Oscillator Calibration bits for 16 MHz"]
pub type Cal16W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL16` reader - Selection of RC Oscillator Calibration bits for 16 MHz"]
pub type Sel16R = crate::BitReader;
#[doc = "Field `SEL16` writer - Selection of RC Oscillator Calibration bits for 16 MHz"]
pub type Sel16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL24` reader - RC Oscillator Calibration bits for 24 MHz"]
pub type Cal24R = crate::FieldReader;
#[doc = "Field `CAL24` writer - RC Oscillator Calibration bits for 24 MHz"]
pub type Cal24W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL24` reader - Selection of RC Oscillator Calibration bits for 24 MHz"]
pub type Sel24R = crate::BitReader;
#[doc = "Field `SEL24` writer - Selection of RC Oscillator Calibration bits for 24 MHz"]
pub type Sel24W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&self) -> Cal8R {
        Cal8R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Selection of RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&self) -> Sel8R {
        Sel8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - RC Oscillator Calibration bits for 16 MHz"]
    #[inline(always)]
    pub fn cal16(&self) -> Cal16R {
        Cal16R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Selection of RC Oscillator Calibration bits for 16 MHz"]
    #[inline(always)]
    pub fn sel16(&self) -> Sel16R {
        Sel16R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - RC Oscillator Calibration bits for 24 MHz"]
    #[inline(always)]
    pub fn cal24(&self) -> Cal24R {
        Cal24R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Selection of RC Oscillator Calibration bits for 24 MHz"]
    #[inline(always)]
    pub fn sel24(&self) -> Sel24R {
        Sel24R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal8(&mut self) -> Cal8W<PmcOcrSpec> {
        Cal8W::new(self, 0)
    }
    #[doc = "Bit 7 - Selection of RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel8(&mut self) -> Sel8W<PmcOcrSpec> {
        Sel8W::new(self, 7)
    }
    #[doc = "Bits 8:14 - RC Oscillator Calibration bits for 16 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal16(&mut self) -> Cal16W<PmcOcrSpec> {
        Cal16W::new(self, 8)
    }
    #[doc = "Bit 15 - Selection of RC Oscillator Calibration bits for 16 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel16(&mut self) -> Sel16W<PmcOcrSpec> {
        Sel16W::new(self, 15)
    }
    #[doc = "Bits 16:22 - RC Oscillator Calibration bits for 24 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal24(&mut self) -> Cal24W<PmcOcrSpec> {
        Cal24W::new(self, 16)
    }
    #[doc = "Bit 23 - Selection of RC Oscillator Calibration bits for 24 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel24(&mut self) -> Sel24W<PmcOcrSpec> {
        Sel24W::new(self, 23)
    }
}
#[doc = "Oscillator Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_ocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_ocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcOcrSpec;
impl crate::RegisterSpec for PmcOcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_ocr::R`](R) reader structure"]
impl crate::Readable for PmcOcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmc_ocr::W`](W) writer structure"]
impl crate::Writable for PmcOcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC_OCR to value 0x0040_4040"]
impl crate::Resettable for PmcOcrSpec {
    const RESET_VALUE: u32 = 0x0040_4040;
}
