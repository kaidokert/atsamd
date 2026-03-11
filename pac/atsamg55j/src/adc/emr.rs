#[doc = "Register `EMR` reader"]
pub type R = crate::R<EmrSpec>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EmrSpec>;
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpmode {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    Low = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    High = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    In = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    Out = 3,
}
impl From<Cmpmode> for u8 {
    #[inline(always)]
    fn from(variant: Cmpmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpmode {
    type Ux = u8;
}
impl crate::IsEnum for Cmpmode {}
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub type CmpmodeR = crate::FieldReader<Cmpmode>;
impl CmpmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpmode {
        match self.bits {
            0 => Cmpmode::Low,
            1 => Cmpmode::High,
            2 => Cmpmode::In,
            3 => Cmpmode::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cmpmode::Low
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cmpmode::High
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Cmpmode::In
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Cmpmode::Out
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub type CmpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmpmode, crate::Safe>;
impl<'a, REG> CmpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmode::Low)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmode::High)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmode::In)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmode::Out)
    }
}
#[doc = "Comparison Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmptype {
    #[doc = "0: Any conversion is performed and comparison function drives the COMPE flag."]
    FlagOnly = 0,
    #[doc = "1: Comparison conditions must be met to start the storage of all conversions until the CMPRST bit is set."]
    StartCondition = 1,
}
impl From<Cmptype> for bool {
    #[inline(always)]
    fn from(variant: Cmptype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPTYPE` reader - Comparison Type"]
pub type CmptypeR = crate::BitReader<Cmptype>;
impl CmptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmptype {
        match self.bits {
            false => Cmptype::FlagOnly,
            true => Cmptype::StartCondition,
        }
    }
    #[doc = "Any conversion is performed and comparison function drives the COMPE flag."]
    #[inline(always)]
    pub fn is_flag_only(&self) -> bool {
        *self == Cmptype::FlagOnly
    }
    #[doc = "Comparison conditions must be met to start the storage of all conversions until the CMPRST bit is set."]
    #[inline(always)]
    pub fn is_start_condition(&self) -> bool {
        *self == Cmptype::StartCondition
    }
}
#[doc = "Field `CMPTYPE` writer - Comparison Type"]
pub type CmptypeW<'a, REG> = crate::BitWriter<'a, REG, Cmptype>;
impl<'a, REG> CmptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any conversion is performed and comparison function drives the COMPE flag."]
    #[inline(always)]
    pub fn flag_only(self) -> &'a mut crate::W<REG> {
        self.variant(Cmptype::FlagOnly)
    }
    #[doc = "Comparison conditions must be met to start the storage of all conversions until the CMPRST bit is set."]
    #[inline(always)]
    pub fn start_condition(self) -> &'a mut crate::W<REG> {
        self.variant(Cmptype::StartCondition)
    }
}
#[doc = "Field `CMPSEL` reader - Comparison Selected Channel"]
pub type CmpselR = crate::FieldReader;
#[doc = "Field `CMPSEL` writer - Comparison Selected Channel"]
pub type CmpselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMPALL` reader - Compare All Channels"]
pub type CmpallR = crate::BitReader;
#[doc = "Field `CMPALL` writer - Compare All Channels"]
pub type CmpallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPFILTER` reader - Compare Event Filtering"]
pub type CmpfilterR = crate::FieldReader;
#[doc = "Field `CMPFILTER` writer - Compare Event Filtering"]
pub type CmpfilterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Over Sampling Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Osr {
    #[doc = "0: No averaging. ADC sample rate is maximum."]
    NoAverage = 0,
    #[doc = "1: 1-bit enhanced resolution by averaging. ADC sample rate divided by 4."]
    Osr4 = 1,
    #[doc = "2: 2-bit enhanced resolution by averaging. ADC sample rate divided by 16."]
    Osr16 = 2,
    #[doc = "3: 3-bit enhanced resolution by averaging. ADC sample rate divided by 64."]
    Osr64 = 3,
    #[doc = "4: 4-bit enhanced resolution by averaging. ADC sample rate divided by 256."]
    Osr256 = 4,
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
#[doc = "Field `OSR` reader - Over Sampling Rate"]
pub type OsrR = crate::FieldReader<Osr>;
impl OsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Osr> {
        match self.bits {
            0 => Some(Osr::NoAverage),
            1 => Some(Osr::Osr4),
            2 => Some(Osr::Osr16),
            3 => Some(Osr::Osr64),
            4 => Some(Osr::Osr256),
            _ => None,
        }
    }
    #[doc = "No averaging. ADC sample rate is maximum."]
    #[inline(always)]
    pub fn is_no_average(&self) -> bool {
        *self == Osr::NoAverage
    }
    #[doc = "1-bit enhanced resolution by averaging. ADC sample rate divided by 4."]
    #[inline(always)]
    pub fn is_osr4(&self) -> bool {
        *self == Osr::Osr4
    }
    #[doc = "2-bit enhanced resolution by averaging. ADC sample rate divided by 16."]
    #[inline(always)]
    pub fn is_osr16(&self) -> bool {
        *self == Osr::Osr16
    }
    #[doc = "3-bit enhanced resolution by averaging. ADC sample rate divided by 64."]
    #[inline(always)]
    pub fn is_osr64(&self) -> bool {
        *self == Osr::Osr64
    }
    #[doc = "4-bit enhanced resolution by averaging. ADC sample rate divided by 256."]
    #[inline(always)]
    pub fn is_osr256(&self) -> bool {
        *self == Osr::Osr256
    }
}
#[doc = "Field `OSR` writer - Over Sampling Rate"]
pub type OsrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Osr>;
impl<'a, REG> OsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No averaging. ADC sample rate is maximum."]
    #[inline(always)]
    pub fn no_average(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::NoAverage)
    }
    #[doc = "1-bit enhanced resolution by averaging. ADC sample rate divided by 4."]
    #[inline(always)]
    pub fn osr4(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::Osr4)
    }
    #[doc = "2-bit enhanced resolution by averaging. ADC sample rate divided by 16."]
    #[inline(always)]
    pub fn osr16(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::Osr16)
    }
    #[doc = "3-bit enhanced resolution by averaging. ADC sample rate divided by 64."]
    #[inline(always)]
    pub fn osr64(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::Osr64)
    }
    #[doc = "4-bit enhanced resolution by averaging. ADC sample rate divided by 256."]
    #[inline(always)]
    pub fn osr256(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::Osr256)
    }
}
#[doc = "Averaging on Single Trigger Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aste {
    #[doc = "0: The average requests several trigger events."]
    MultiTrigAverage = 0,
    #[doc = "1: The average requests only one trigger event."]
    SingleTrigAverage = 1,
}
impl From<Aste> for bool {
    #[inline(always)]
    fn from(variant: Aste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASTE` reader - Averaging on Single Trigger Event"]
pub type AsteR = crate::BitReader<Aste>;
impl AsteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aste {
        match self.bits {
            false => Aste::MultiTrigAverage,
            true => Aste::SingleTrigAverage,
        }
    }
    #[doc = "The average requests several trigger events."]
    #[inline(always)]
    pub fn is_multi_trig_average(&self) -> bool {
        *self == Aste::MultiTrigAverage
    }
    #[doc = "The average requests only one trigger event."]
    #[inline(always)]
    pub fn is_single_trig_average(&self) -> bool {
        *self == Aste::SingleTrigAverage
    }
}
#[doc = "Field `ASTE` writer - Averaging on Single Trigger Event"]
pub type AsteW<'a, REG> = crate::BitWriter<'a, REG, Aste>;
impl<'a, REG> AsteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The average requests several trigger events."]
    #[inline(always)]
    pub fn multi_trig_average(self) -> &'a mut crate::W<REG> {
        self.variant(Aste::MultiTrigAverage)
    }
    #[doc = "The average requests only one trigger event."]
    #[inline(always)]
    pub fn single_trig_average(self) -> &'a mut crate::W<REG> {
        self.variant(Aste::SingleTrigAverage)
    }
}
#[doc = "External Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcclk {
    #[doc = "0: The peripheral clock is the source for the ADC prescaler."]
    PeriphClk = 0,
    #[doc = "1: PMC PCKx is the source clock for the ADC prescaler, thus the ADC clock can be independent of the core/peripheral clock."]
    PmcPck = 1,
}
impl From<Srcclk> for bool {
    #[inline(always)]
    fn from(variant: Srcclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCCLK` reader - External Clock Selection"]
pub type SrcclkR = crate::BitReader<Srcclk>;
impl SrcclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcclk {
        match self.bits {
            false => Srcclk::PeriphClk,
            true => Srcclk::PmcPck,
        }
    }
    #[doc = "The peripheral clock is the source for the ADC prescaler."]
    #[inline(always)]
    pub fn is_periph_clk(&self) -> bool {
        *self == Srcclk::PeriphClk
    }
    #[doc = "PMC PCKx is the source clock for the ADC prescaler, thus the ADC clock can be independent of the core/peripheral clock."]
    #[inline(always)]
    pub fn is_pmc_pck(&self) -> bool {
        *self == Srcclk::PmcPck
    }
}
#[doc = "Field `SRCCLK` writer - External Clock Selection"]
pub type SrcclkW<'a, REG> = crate::BitWriter<'a, REG, Srcclk>;
impl<'a, REG> SrcclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The peripheral clock is the source for the ADC prescaler."]
    #[inline(always)]
    pub fn periph_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Srcclk::PeriphClk)
    }
    #[doc = "PMC PCKx is the source clock for the ADC prescaler, thus the ADC clock can be independent of the core/peripheral clock."]
    #[inline(always)]
    pub fn pmc_pck(self) -> &'a mut crate::W<REG> {
        self.variant(Srcclk::PmcPck)
    }
}
#[doc = "Field `TAG` reader - Tag of the ADC_LCDR"]
pub type TagR = crate::BitReader;
#[doc = "Field `TAG` writer - Tag of the ADC_LCDR"]
pub type TagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CmpmodeR {
        CmpmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Comparison Type"]
    #[inline(always)]
    pub fn cmptype(&self) -> CmptypeR {
        CmptypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CmpselR {
        CmpselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&self) -> CmpallR {
        CmpallR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&self) -> CmpfilterR {
        CmpfilterR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Over Sampling Rate"]
    #[inline(always)]
    pub fn osr(&self) -> OsrR {
        OsrR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Averaging on Single Trigger Event"]
    #[inline(always)]
    pub fn aste(&self) -> AsteR {
        AsteR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - External Clock Selection"]
    #[inline(always)]
    pub fn srcclk(&self) -> SrcclkR {
        SrcclkR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Tag of the ADC_LCDR"]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmode(&mut self) -> CmpmodeW<EmrSpec> {
        CmpmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - Comparison Type"]
    #[inline(always)]
    #[must_use]
    pub fn cmptype(&mut self) -> CmptypeW<EmrSpec> {
        CmptypeW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel(&mut self) -> CmpselW<EmrSpec> {
        CmpselW::new(self, 4)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    #[must_use]
    pub fn cmpall(&mut self) -> CmpallW<EmrSpec> {
        CmpallW::new(self, 9)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn cmpfilter(&mut self) -> CmpfilterW<EmrSpec> {
        CmpfilterW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Over Sampling Rate"]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OsrW<EmrSpec> {
        OsrW::new(self, 16)
    }
    #[doc = "Bit 20 - Averaging on Single Trigger Event"]
    #[inline(always)]
    #[must_use]
    pub fn aste(&mut self) -> AsteW<EmrSpec> {
        AsteW::new(self, 20)
    }
    #[doc = "Bit 21 - External Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn srcclk(&mut self) -> SrcclkW<EmrSpec> {
        SrcclkW::new(self, 21)
    }
    #[doc = "Bit 24 - Tag of the ADC_LCDR"]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TagW<EmrSpec> {
        TagW::new(self, 24)
    }
}
#[doc = "Extended Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmrSpec;
impl crate::RegisterSpec for EmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EmrSpec {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EmrSpec {
    const RESET_VALUE: u32 = 0;
}
