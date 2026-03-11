#[doc = "Register `CMPR` reader"]
pub type R = crate::R<CmprSpec>;
#[doc = "Register `CMPR` writer"]
pub type W = crate::W<CmprSpec>;
#[doc = "Field `VAL1` reader - First Comparison Value for Received Character"]
pub type Val1R = crate::FieldReader<u16>;
#[doc = "Field `VAL1` writer - First Comparison Value for Received Character"]
pub type Val1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpmode {
    #[doc = "0: Any character is received and comparison function drives CMP flag."]
    FlagOnly = 0,
    #[doc = "1: Comparison condition must be met to start reception of all incoming charactersuntil REQCLR is set."]
    StartCondition = 1,
}
impl From<Cmpmode> for bool {
    #[inline(always)]
    fn from(variant: Cmpmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub type CmpmodeR = crate::BitReader<Cmpmode>;
impl CmpmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpmode {
        match self.bits {
            false => Cmpmode::FlagOnly,
            true => Cmpmode::StartCondition,
        }
    }
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn is_flag_only(&self) -> bool {
        *self == Cmpmode::FlagOnly
    }
    #[doc = "Comparison condition must be met to start reception of all incoming charactersuntil REQCLR is set."]
    #[inline(always)]
    pub fn is_start_condition(&self) -> bool {
        *self == Cmpmode::StartCondition
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub type CmpmodeW<'a, REG> = crate::BitWriter<'a, REG, Cmpmode>;
impl<'a, REG> CmpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn flag_only(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmode::FlagOnly)
    }
    #[doc = "Comparison condition must be met to start reception of all incoming charactersuntil REQCLR is set."]
    #[inline(always)]
    pub fn start_condition(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmode::StartCondition)
    }
}
#[doc = "Field `CMPPAR` reader - Compare Parity"]
pub type CmpparR = crate::BitReader;
#[doc = "Field `CMPPAR` writer - Compare Parity"]
pub type CmpparW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VAL2` reader - Second Comparison Value for Received Character"]
pub type Val2R = crate::FieldReader<u16>;
#[doc = "Field `VAL2` writer - Second Comparison Value for Received Character"]
pub type Val2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&self) -> Val1R {
        Val1R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CmpmodeR {
        CmpmodeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    pub fn cmppar(&self) -> CmpparR {
        CmpparR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&self) -> Val2R {
        Val2R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - First Comparison Value for Received Character"]
    #[inline(always)]
    #[must_use]
    pub fn val1(&mut self) -> Val1W<CmprSpec> {
        Val1W::new(self, 0)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmode(&mut self) -> CmpmodeW<CmprSpec> {
        CmpmodeW::new(self, 12)
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    #[must_use]
    pub fn cmppar(&mut self) -> CmpparW<CmprSpec> {
        CmpparW::new(self, 14)
    }
    #[doc = "Bits 16:24 - Second Comparison Value for Received Character"]
    #[inline(always)]
    #[must_use]
    pub fn val2(&mut self) -> Val2W<CmprSpec> {
        Val2W::new(self, 16)
    }
}
#[doc = "USART Comparison Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
