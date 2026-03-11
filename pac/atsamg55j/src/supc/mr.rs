#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "POR Core Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodrsten {
    #[doc = "0: The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    NotEnable = 0,
    #[doc = "1: The core reset signal vddcore_nreset is asserted when a brownout detection occurs."]
    Enable = 1,
}
impl From<Bodrsten> for bool {
    #[inline(always)]
    fn from(variant: Bodrsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRSTEN` reader - POR Core Reset Enable"]
pub type BodrstenR = crate::BitReader<Bodrsten>;
impl BodrstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodrsten {
        match self.bits {
            false => Bodrsten::NotEnable,
            true => Bodrsten::Enable,
        }
    }
    #[doc = "The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Bodrsten::NotEnable
    }
    #[doc = "The core reset signal vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bodrsten::Enable
    }
}
#[doc = "Field `BODRSTEN` writer - POR Core Reset Enable"]
pub type BodrstenW<'a, REG> = crate::BitWriter<'a, REG, Bodrsten>;
impl<'a, REG> BodrstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrsten::NotEnable)
    }
    #[doc = "The core reset signal vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrsten::Enable)
    }
}
#[doc = "POR Core Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boddis {
    #[doc = "0: The core brownout detector is enabled."]
    Enable = 0,
    #[doc = "1: The core brownout detector is disabled."]
    Disable = 1,
}
impl From<Boddis> for bool {
    #[inline(always)]
    fn from(variant: Boddis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODDIS` reader - POR Core Disable"]
pub type BoddisR = crate::BitReader<Boddis>;
impl BoddisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boddis {
        match self.bits {
            false => Boddis::Enable,
            true => Boddis::Disable,
        }
    }
    #[doc = "The core brownout detector is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Boddis::Enable
    }
    #[doc = "The core brownout detector is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Boddis::Disable
    }
}
#[doc = "Field `BODDIS` writer - POR Core Disable"]
pub type BoddisW<'a, REG> = crate::BitWriter<'a, REG, Boddis>;
impl<'a, REG> BoddisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The core brownout detector is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Boddis::Enable)
    }
    #[doc = "The core brownout detector is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Boddis::Disable)
    }
}
#[doc = "Oscillator Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscbypass {
    #[doc = "0: No effect. Clock selection depends on XTALSEL value."]
    NoEffect = 0,
    #[doc = "1: The 32 kHz crystal oscillator is bypassed if XTALSEL=1. OSCBYPASS must be set prior to write XTALSEL=1."]
    Bypass = 1,
}
impl From<Oscbypass> for bool {
    #[inline(always)]
    fn from(variant: Oscbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCBYPASS` reader - Oscillator Bypass"]
pub type OscbypassR = crate::BitReader<Oscbypass>;
impl OscbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscbypass {
        match self.bits {
            false => Oscbypass::NoEffect,
            true => Oscbypass::Bypass,
        }
    }
    #[doc = "No effect. Clock selection depends on XTALSEL value."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Oscbypass::NoEffect
    }
    #[doc = "The 32 kHz crystal oscillator is bypassed if XTALSEL=1. OSCBYPASS must be set prior to write XTALSEL=1."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Oscbypass::Bypass
    }
}
#[doc = "Field `OSCBYPASS` writer - Oscillator Bypass"]
pub type OscbypassW<'a, REG> = crate::BitWriter<'a, REG, Oscbypass>;
impl<'a, REG> OscbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. Clock selection depends on XTALSEL value."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Oscbypass::NoEffect)
    }
    #[doc = "The 32 kHz crystal oscillator is bypassed if XTALSEL=1. OSCBYPASS must be set prior to write XTALSEL=1."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Oscbypass::Bypass)
    }
}
#[doc = "Cache Data SRAM Power Switch\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdpswitch {
    #[doc = "0: The cache data SRAM is not powered."]
    Off = 0,
    #[doc = "1: The cache data SRAM is powered."]
    On = 1,
}
impl From<Cdpswitch> for bool {
    #[inline(always)]
    fn from(variant: Cdpswitch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDPSWITCH` reader - Cache Data SRAM Power Switch"]
pub type CdpswitchR = crate::BitReader<Cdpswitch>;
impl CdpswitchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdpswitch {
        match self.bits {
            false => Cdpswitch::Off,
            true => Cdpswitch::On,
        }
    }
    #[doc = "The cache data SRAM is not powered."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cdpswitch::Off
    }
    #[doc = "The cache data SRAM is powered."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Cdpswitch::On
    }
}
#[doc = "Field `CDPSWITCH` writer - Cache Data SRAM Power Switch"]
pub type CdpswitchW<'a, REG> = crate::BitWriter<'a, REG, Cdpswitch>;
impl<'a, REG> CdpswitchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The cache data SRAM is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cdpswitch::Off)
    }
    #[doc = "The cache data SRAM is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Cdpswitch::On)
    }
}
#[doc = "Cache Tag SRAM Power Switch\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctpswitch {
    #[doc = "0: The cache tag SRAM is not powered."]
    Off = 0,
    #[doc = "1: The cache tag SRAM is powered."]
    On = 1,
}
impl From<Ctpswitch> for bool {
    #[inline(always)]
    fn from(variant: Ctpswitch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTPSWITCH` reader - Cache Tag SRAM Power Switch"]
pub type CtpswitchR = crate::BitReader<Ctpswitch>;
impl CtpswitchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctpswitch {
        match self.bits {
            false => Ctpswitch::Off,
            true => Ctpswitch::On,
        }
    }
    #[doc = "The cache tag SRAM is not powered."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ctpswitch::Off
    }
    #[doc = "The cache tag SRAM is powered."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ctpswitch::On
    }
}
#[doc = "Field `CTPSWITCH` writer - Cache Tag SRAM Power Switch"]
pub type CtpswitchW<'a, REG> = crate::BitWriter<'a, REG, Ctpswitch>;
impl<'a, REG> CtpswitchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The cache tag SRAM is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Ctpswitch::Off)
    }
    #[doc = "The cache tag SRAM is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Ctpswitch::On)
    }
}
#[doc = "Field `ONE` reader - This bit must always be set to 1."]
pub type OneR = crate::BitReader;
#[doc = "Field `ONE` writer - This bit must always be set to 1."]
pub type OneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Password Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "165: Writing any other value in this field aborts the write operation.Always reads as 0."]
    Passwd = 165,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` reader - Password Key"]
pub type KeyR = crate::FieldReader<Key>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Key> {
        match self.bits {
            165 => Some(Key::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Key::Passwd
    }
}
#[doc = "Field `KEY` writer - Password Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Passwd)
    }
}
impl R {
    #[doc = "Bit 12 - POR Core Reset Enable"]
    #[inline(always)]
    pub fn bodrsten(&self) -> BodrstenR {
        BodrstenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - POR Core Disable"]
    #[inline(always)]
    pub fn boddis(&self) -> BoddisR {
        BoddisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&self) -> OscbypassR {
        OscbypassR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Cache Data SRAM Power Switch"]
    #[inline(always)]
    pub fn cdpswitch(&self) -> CdpswitchR {
        CdpswitchR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Cache Tag SRAM Power Switch"]
    #[inline(always)]
    pub fn ctpswitch(&self) -> CtpswitchR {
        CtpswitchR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This bit must always be set to 1."]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - POR Core Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bodrsten(&mut self) -> BodrstenW<MrSpec> {
        BodrstenW::new(self, 12)
    }
    #[doc = "Bit 13 - POR Core Disable"]
    #[inline(always)]
    #[must_use]
    pub fn boddis(&mut self) -> BoddisW<MrSpec> {
        BoddisW::new(self, 13)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn oscbypass(&mut self) -> OscbypassW<MrSpec> {
        OscbypassW::new(self, 20)
    }
    #[doc = "Bit 21 - Cache Data SRAM Power Switch"]
    #[inline(always)]
    #[must_use]
    pub fn cdpswitch(&mut self) -> CdpswitchW<MrSpec> {
        CdpswitchW::new(self, 21)
    }
    #[doc = "Bit 22 - Cache Tag SRAM Power Switch"]
    #[inline(always)]
    #[must_use]
    pub fn ctpswitch(&mut self) -> CtpswitchW<MrSpec> {
        CtpswitchW::new(self, 22)
    }
    #[doc = "Bit 23 - This bit must always be set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn one(&mut self) -> OneW<MrSpec> {
        OneW::new(self, 23)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<MrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Supply Controller Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0x00e0_5a00"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0x00e0_5a00;
}
