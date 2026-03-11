#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "ADC Auto-test modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Autotest {
    #[doc = "0: No auto test, normal mode of operation"]
    NoAutotest = 0,
    #[doc = "1: Offset Error test (refer to ADC cell datasheet)"]
    OffsetError = 1,
    #[doc = "2: Gain Error (high code) test (refer to ADC cell datasheet)"]
    GainErrorHigh = 2,
    #[doc = "3: Gain Error (low code) test (refer to ADC cell datasheet)"]
    GainErrorLow = 3,
}
impl From<Autotest> for u8 {
    #[inline(always)]
    fn from(variant: Autotest) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Autotest {
    type Ux = u8;
}
impl crate::IsEnum for Autotest {}
#[doc = "Field `AUTOTEST` reader - ADC Auto-test modes"]
pub type AutotestR = crate::FieldReader<Autotest>;
impl AutotestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autotest {
        match self.bits {
            0 => Autotest::NoAutotest,
            1 => Autotest::OffsetError,
            2 => Autotest::GainErrorHigh,
            3 => Autotest::GainErrorLow,
            _ => unreachable!(),
        }
    }
    #[doc = "No auto test, normal mode of operation"]
    #[inline(always)]
    pub fn is_no_autotest(&self) -> bool {
        *self == Autotest::NoAutotest
    }
    #[doc = "Offset Error test (refer to ADC cell datasheet)"]
    #[inline(always)]
    pub fn is_offset_error(&self) -> bool {
        *self == Autotest::OffsetError
    }
    #[doc = "Gain Error (high code) test (refer to ADC cell datasheet)"]
    #[inline(always)]
    pub fn is_gain_error_high(&self) -> bool {
        *self == Autotest::GainErrorHigh
    }
    #[doc = "Gain Error (low code) test (refer to ADC cell datasheet)"]
    #[inline(always)]
    pub fn is_gain_error_low(&self) -> bool {
        *self == Autotest::GainErrorLow
    }
}
#[doc = "Field `AUTOTEST` writer - ADC Auto-test modes"]
pub type AutotestW<'a, REG> = crate::FieldWriter<'a, REG, 2, Autotest, crate::Safe>;
impl<'a, REG> AutotestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No auto test, normal mode of operation"]
    #[inline(always)]
    pub fn no_autotest(self) -> &'a mut crate::W<REG> {
        self.variant(Autotest::NoAutotest)
    }
    #[doc = "Offset Error test (refer to ADC cell datasheet)"]
    #[inline(always)]
    pub fn offset_error(self) -> &'a mut crate::W<REG> {
        self.variant(Autotest::OffsetError)
    }
    #[doc = "Gain Error (high code) test (refer to ADC cell datasheet)"]
    #[inline(always)]
    pub fn gain_error_high(self) -> &'a mut crate::W<REG> {
        self.variant(Autotest::GainErrorHigh)
    }
    #[doc = "Gain Error (low code) test (refer to ADC cell datasheet)"]
    #[inline(always)]
    pub fn gain_error_low(self) -> &'a mut crate::W<REG> {
        self.variant(Autotest::GainErrorLow)
    }
}
impl R {
    #[doc = "Bits 30:31 - ADC Auto-test modes"]
    #[inline(always)]
    pub fn autotest(&self) -> AutotestR {
        AutotestR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - ADC Auto-test modes"]
    #[inline(always)]
    #[must_use]
    pub fn autotest(&mut self) -> AutotestW<AcrSpec> {
        AutotestW::new(self, 30)
    }
}
#[doc = "Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for AcrSpec {
    const RESET_VALUE: u32 = 0;
}
