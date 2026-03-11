#[doc = "Register `LCTMR` reader"]
pub type R = crate::R<LctmrSpec>;
#[doc = "Register `LCTMR` writer"]
pub type W = crate::W<LctmrSpec>;
#[doc = "Field `DUALTRIG` reader - Dual Trigger ON"]
pub type DualtrigR = crate::BitReader;
#[doc = "Field `DUALTRIG` writer - Dual Trigger ON"]
pub type DualtrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Last Channel Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpmod {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    Low = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    High = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    In = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    Out = 3,
}
impl From<Cmpmod> for u8 {
    #[inline(always)]
    fn from(variant: Cmpmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpmod {
    type Ux = u8;
}
impl crate::IsEnum for Cmpmod {}
#[doc = "Field `CMPMOD` reader - Last Channel Comparison Mode"]
pub type CmpmodR = crate::FieldReader<Cmpmod>;
impl CmpmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpmod {
        match self.bits {
            0 => Cmpmod::Low,
            1 => Cmpmod::High,
            2 => Cmpmod::In,
            3 => Cmpmod::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cmpmod::Low
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cmpmod::High
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Cmpmod::In
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Cmpmod::Out
    }
}
#[doc = "Field `CMPMOD` writer - Last Channel Comparison Mode"]
pub type CmpmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmpmod, crate::Safe>;
impl<'a, REG> CmpmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmod::Low)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmod::High)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmod::In)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmod::Out)
    }
}
impl R {
    #[doc = "Bit 0 - Dual Trigger ON"]
    #[inline(always)]
    pub fn dualtrig(&self) -> DualtrigR {
        DualtrigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Last Channel Comparison Mode"]
    #[inline(always)]
    pub fn cmpmod(&self) -> CmpmodR {
        CmpmodR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dual Trigger ON"]
    #[inline(always)]
    #[must_use]
    pub fn dualtrig(&mut self) -> DualtrigW<LctmrSpec> {
        DualtrigW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Last Channel Comparison Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmod(&mut self) -> CmpmodW<LctmrSpec> {
        CmpmodW::new(self, 4)
    }
}
#[doc = "Last Channel Trigger Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lctmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LctmrSpec;
impl crate::RegisterSpec for LctmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lctmr::R`](R) reader structure"]
impl crate::Readable for LctmrSpec {}
#[doc = "`write(|w| ..)` method takes [`lctmr::W`](W) writer structure"]
impl crate::Writable for LctmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCTMR to value 0"]
impl crate::Resettable for LctmrSpec {
    const RESET_VALUE: u32 = 0;
}
