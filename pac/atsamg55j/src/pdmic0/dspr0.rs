#[doc = "Register `DSPR0` reader"]
pub type R = crate::R<Dspr0Spec>;
#[doc = "Register `DSPR0` writer"]
pub type W = crate::W<Dspr0Spec>;
#[doc = "Field `HPFBYP` reader - High-Pass Filter Bypass"]
pub type HpfbypR = crate::BitReader;
#[doc = "Field `HPFBYP` writer - High-Pass Filter Bypass"]
pub type HpfbypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINBYP` reader - SINCC Filter Bypass"]
pub type SinbypR = crate::BitReader;
#[doc = "Field `SINBYP` writer - SINCC Filter Bypass"]
pub type SinbypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZE` reader - Data Size"]
pub type SizeR = crate::BitReader;
#[doc = "Field `SIZE` writer - Data Size"]
pub type SizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Oversampling Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Osr {
    #[doc = "0: Oversampling ratio is 128"]
    _128 = 0,
    #[doc = "1: Oversampling ratio is 64"]
    _64 = 1,
}
impl From<Osr> for u8 {
    #[inline(always)]
    fn from(variant: Osr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Osr {
    type Ux = u8;
}
impl crate::IsEnum for Osr {}
#[doc = "Field `OSR` reader - Oversampling Ratio"]
pub type OsrR = crate::FieldReader<Osr>;
impl OsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Osr> {
        match self.bits {
            0 => Some(Osr::_128),
            1 => Some(Osr::_64),
            _ => None,
        }
    }
    #[doc = "Oversampling ratio is 128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Osr::_128
    }
    #[doc = "Oversampling ratio is 64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Osr::_64
    }
}
#[doc = "Field `OSR` writer - Oversampling Ratio"]
pub type OsrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Osr>;
impl<'a, REG> OsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Oversampling ratio is 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::_128)
    }
    #[doc = "Oversampling ratio is 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::_64)
    }
}
#[doc = "Field `SCALE` reader - Data Scale"]
pub type ScaleR = crate::FieldReader;
#[doc = "Field `SCALE` writer - Data Scale"]
pub type ScaleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SHIFT` reader - Data Shift"]
pub type ShiftR = crate::FieldReader;
#[doc = "Field `SHIFT` writer - Data Shift"]
pub type ShiftW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 1 - High-Pass Filter Bypass"]
    #[inline(always)]
    pub fn hpfbyp(&self) -> HpfbypR {
        HpfbypR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SINCC Filter Bypass"]
    #[inline(always)]
    pub fn sinbyp(&self) -> SinbypR {
        SinbypR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Oversampling Ratio"]
    #[inline(always)]
    pub fn osr(&self) -> OsrR {
        OsrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Data Scale"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Data Shift"]
    #[inline(always)]
    pub fn shift(&self) -> ShiftR {
        ShiftR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - High-Pass Filter Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hpfbyp(&mut self) -> HpfbypW<Dspr0Spec> {
        HpfbypW::new(self, 1)
    }
    #[doc = "Bit 2 - SINCC Filter Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn sinbyp(&mut self) -> SinbypW<Dspr0Spec> {
        SinbypW::new(self, 2)
    }
    #[doc = "Bit 3 - Data Size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<Dspr0Spec> {
        SizeW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Oversampling Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OsrW<Dspr0Spec> {
        OsrW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Data Scale"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> ScaleW<Dspr0Spec> {
        ScaleW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Data Shift"]
    #[inline(always)]
    #[must_use]
    pub fn shift(&mut self) -> ShiftW<Dspr0Spec> {
        ShiftW::new(self, 12)
    }
}
#[doc = "DSP Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dspr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dspr0Spec;
impl crate::RegisterSpec for Dspr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dspr0::R`](R) reader structure"]
impl crate::Readable for Dspr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dspr0::W`](W) writer structure"]
impl crate::Writable for Dspr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSPR0 to value 0"]
impl crate::Resettable for Dspr0Spec {
    const RESET_VALUE: u32 = 0;
}
