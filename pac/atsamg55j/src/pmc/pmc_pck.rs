#[doc = "Register `PMC_PCK[%s]` reader"]
pub type R = crate::R<PmcPckSpec>;
#[doc = "Register `PMC_PCK[%s]` writer"]
pub type W = crate::W<PmcPckSpec>;
#[doc = "Master Clock Source Selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Css {
    #[doc = "0: Slow Clock is selected"]
    SlowClk = 0,
    #[doc = "1: Main Clock is selected"]
    MainClk = 1,
    #[doc = "2: PLLA Clock is selected"]
    PllaClk = 2,
    #[doc = "3: PLLB Clock is selected"]
    PllbClk = 3,
    #[doc = "4: Master Clock is selected"]
    Mck = 4,
}
impl From<Css> for u8 {
    #[inline(always)]
    fn from(variant: Css) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Css {
    type Ux = u8;
}
impl crate::IsEnum for Css {}
#[doc = "Field `CSS` reader - Master Clock Source Selection"]
pub type CssR = crate::FieldReader<Css>;
impl CssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Css> {
        match self.bits {
            0 => Some(Css::SlowClk),
            1 => Some(Css::MainClk),
            2 => Some(Css::PllaClk),
            3 => Some(Css::PllbClk),
            4 => Some(Css::Mck),
            _ => None,
        }
    }
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == Css::SlowClk
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == Css::MainClk
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == Css::PllaClk
    }
    #[doc = "PLLB Clock is selected"]
    #[inline(always)]
    pub fn is_pllb_clk(&self) -> bool {
        *self == Css::PllbClk
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Css::Mck
    }
}
#[doc = "Field `CSS` writer - Master Clock Source Selection"]
pub type CssW<'a, REG> = crate::FieldWriter<'a, REG, 3, Css>;
impl<'a, REG> CssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Css::SlowClk)
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Css::MainClk)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Css::PllaClk)
    }
    #[doc = "PLLB Clock is selected"]
    #[inline(always)]
    pub fn pllb_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Css::PllbClk)
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Css::Mck)
    }
}
#[doc = "Field `PRES` reader - Programmable Clock Prescaler"]
pub type PresR = crate::FieldReader;
#[doc = "Field `PRES` writer - Programmable Clock Prescaler"]
pub type PresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:11 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PresR {
        PresR::new(((self.bits >> 4) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CssW<PmcPckSpec> {
        CssW::new(self, 0)
    }
    #[doc = "Bits 4:11 - Programmable Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PresW<PmcPckSpec> {
        PresW::new(self, 4)
    }
}
#[doc = "Programmable Clock 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_pck::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_pck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcPckSpec;
impl crate::RegisterSpec for PmcPckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_pck::R`](R) reader structure"]
impl crate::Readable for PmcPckSpec {}
#[doc = "`write(|w| ..)` method takes [`pmc_pck::W`](W) writer structure"]
impl crate::Writable for PmcPckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
