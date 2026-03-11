#[doc = "Register `PWMR` reader"]
pub type R = crate::R<PwmrSpec>;
#[doc = "Register `PWMR` writer"]
pub type W = crate::W<PwmrSpec>;
#[doc = "Low Power Value Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpowers {
    #[doc = "0: The trimming value applied to the regulator when the device is in Wait mode. This value is factory-defined."]
    Factory = 0,
    #[doc = "1: The trimming value applied to the regulator is defined by the value programmed in the LPOWERx bits."]
    User = 1,
}
impl From<Lpowers> for bool {
    #[inline(always)]
    fn from(variant: Lpowers) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPOWERS` reader - Low Power Value Selection"]
pub type LpowersR = crate::BitReader<Lpowers>;
impl LpowersR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpowers {
        match self.bits {
            false => Lpowers::Factory,
            true => Lpowers::User,
        }
    }
    #[doc = "The trimming value applied to the regulator when the device is in Wait mode. This value is factory-defined."]
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        *self == Lpowers::Factory
    }
    #[doc = "The trimming value applied to the regulator is defined by the value programmed in the LPOWERx bits."]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == Lpowers::User
    }
}
#[doc = "Field `LPOWERS` writer - Low Power Value Selection"]
pub type LpowersW<'a, REG> = crate::BitWriter<'a, REG, Lpowers>;
impl<'a, REG> LpowersW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trimming value applied to the regulator when the device is in Wait mode. This value is factory-defined."]
    #[inline(always)]
    pub fn factory(self) -> &'a mut crate::W<REG> {
        self.variant(Lpowers::Factory)
    }
    #[doc = "The trimming value applied to the regulator is defined by the value programmed in the LPOWERx bits."]
    #[inline(always)]
    pub fn user(self) -> &'a mut crate::W<REG> {
        self.variant(Lpowers::User)
    }
}
#[doc = "Field `LPOWER0` reader - Low Power Value"]
pub type Lpower0R = crate::BitReader;
#[doc = "Field `LPOWER0` writer - Low Power Value"]
pub type Lpower0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPOWER1` reader - Low Power Value"]
pub type Lpower1R = crate::BitReader;
#[doc = "Field `LPOWER1` writer - Low Power Value"]
pub type Lpower1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPOWER2` reader - Low Power Value"]
pub type Lpower2R = crate::BitReader;
#[doc = "Field `LPOWER2` writer - Low Power Value"]
pub type Lpower2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPOWER3` reader - Low Power Value"]
pub type Lpower3R = crate::BitReader;
#[doc = "Field `LPOWER3` writer - Low Power Value"]
pub type Lpower3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Start-up Time when Resuming from Wait Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stuptime {
    #[doc = "0: Fast start-up."]
    Fast = 0,
    #[doc = "1: Slow start-up."]
    Slow = 1,
}
impl From<Stuptime> for bool {
    #[inline(always)]
    fn from(variant: Stuptime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STUPTIME` reader - Start-up Time when Resuming from Wait Mode"]
pub type StuptimeR = crate::BitReader<Stuptime>;
impl StuptimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stuptime {
        match self.bits {
            false => Stuptime::Fast,
            true => Stuptime::Slow,
        }
    }
    #[doc = "Fast start-up."]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Stuptime::Fast
    }
    #[doc = "Slow start-up."]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == Stuptime::Slow
    }
}
#[doc = "Field `STUPTIME` writer - Start-up Time when Resuming from Wait Mode"]
pub type StuptimeW<'a, REG> = crate::BitWriter<'a, REG, Stuptime>;
impl<'a, REG> StuptimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast start-up."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Stuptime::Fast)
    }
    #[doc = "Slow start-up."]
    #[inline(always)]
    pub fn slow(self) -> &'a mut crate::W<REG> {
        self.variant(Stuptime::Slow)
    }
}
#[doc = "Enhanced Custom Power Value Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecpwrs {
    #[doc = "0: The trimming value applied to the regulator when the device is in Active mode. This value is factory-defined."]
    Factory = 0,
    #[doc = "1: The trimming value applied to the regulator is defined by the value programmed in ECPWRx bits."]
    User = 1,
}
impl From<Ecpwrs> for bool {
    #[inline(always)]
    fn from(variant: Ecpwrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECPWRS` reader - Enhanced Custom Power Value Selection"]
pub type EcpwrsR = crate::BitReader<Ecpwrs>;
impl EcpwrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecpwrs {
        match self.bits {
            false => Ecpwrs::Factory,
            true => Ecpwrs::User,
        }
    }
    #[doc = "The trimming value applied to the regulator when the device is in Active mode. This value is factory-defined."]
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        *self == Ecpwrs::Factory
    }
    #[doc = "The trimming value applied to the regulator is defined by the value programmed in ECPWRx bits."]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == Ecpwrs::User
    }
}
#[doc = "Field `ECPWRS` writer - Enhanced Custom Power Value Selection"]
pub type EcpwrsW<'a, REG> = crate::BitWriter<'a, REG, Ecpwrs>;
impl<'a, REG> EcpwrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trimming value applied to the regulator when the device is in Active mode. This value is factory-defined."]
    #[inline(always)]
    pub fn factory(self) -> &'a mut crate::W<REG> {
        self.variant(Ecpwrs::Factory)
    }
    #[doc = "The trimming value applied to the regulator is defined by the value programmed in ECPWRx bits."]
    #[inline(always)]
    pub fn user(self) -> &'a mut crate::W<REG> {
        self.variant(Ecpwrs::User)
    }
}
#[doc = "Field `ECPWR0` reader - Enhanced Custom Power Value"]
pub type Ecpwr0R = crate::BitReader;
#[doc = "Field `ECPWR0` writer - Enhanced Custom Power Value"]
pub type Ecpwr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECPWR1` reader - Enhanced Custom Power Value"]
pub type Ecpwr1R = crate::BitReader;
#[doc = "Field `ECPWR1` writer - Enhanced Custom Power Value"]
pub type Ecpwr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECPWR2` reader - Enhanced Custom Power Value"]
pub type Ecpwr2R = crate::BitReader;
#[doc = "Field `ECPWR2` writer - Enhanced Custom Power Value"]
pub type Ecpwr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECPWR3` reader - Enhanced Custom Power Value"]
pub type Ecpwr3R = crate::BitReader;
#[doc = "Field `ECPWR3` writer - Enhanced Custom Power Value"]
pub type Ecpwr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram0on {
    #[doc = "0: SRAMx is not powered."]
    Off = 0,
    #[doc = "1: SRAMx is powered."]
    On = 1,
}
impl From<Sram0on> for bool {
    #[inline(always)]
    fn from(variant: Sram0on) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM0ON` reader - SRAM Power Control"]
pub type Sram0onR = crate::BitReader<Sram0on>;
impl Sram0onR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram0on {
        match self.bits {
            false => Sram0on::Off,
            true => Sram0on::On,
        }
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Sram0on::Off
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Sram0on::On
    }
}
#[doc = "Field `SRAM0ON` writer - SRAM Power Control"]
pub type Sram0onW<'a, REG> = crate::BitWriter<'a, REG, Sram0on>;
impl<'a, REG> Sram0onW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Sram0on::Off)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sram0on::On)
    }
}
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram1on {
    #[doc = "0: SRAMx is not powered."]
    Off = 0,
    #[doc = "1: SRAMx is powered."]
    On = 1,
}
impl From<Sram1on> for bool {
    #[inline(always)]
    fn from(variant: Sram1on) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1ON` reader - SRAM Power Control"]
pub type Sram1onR = crate::BitReader<Sram1on>;
impl Sram1onR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram1on {
        match self.bits {
            false => Sram1on::Off,
            true => Sram1on::On,
        }
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Sram1on::Off
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Sram1on::On
    }
}
#[doc = "Field `SRAM1ON` writer - SRAM Power Control"]
pub type Sram1onW<'a, REG> = crate::BitWriter<'a, REG, Sram1on>;
impl<'a, REG> Sram1onW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Sram1on::Off)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sram1on::On)
    }
}
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram2on {
    #[doc = "0: SRAMx is not powered."]
    Off = 0,
    #[doc = "1: SRAMx is powered."]
    On = 1,
}
impl From<Sram2on> for bool {
    #[inline(always)]
    fn from(variant: Sram2on) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2ON` reader - SRAM Power Control"]
pub type Sram2onR = crate::BitReader<Sram2on>;
impl Sram2onR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram2on {
        match self.bits {
            false => Sram2on::Off,
            true => Sram2on::On,
        }
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Sram2on::Off
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Sram2on::On
    }
}
#[doc = "Field `SRAM2ON` writer - SRAM Power Control"]
pub type Sram2onW<'a, REG> = crate::BitWriter<'a, REG, Sram2on>;
impl<'a, REG> Sram2onW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Sram2on::Off)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sram2on::On)
    }
}
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram3on {
    #[doc = "0: SRAMx is not powered."]
    Off = 0,
    #[doc = "1: SRAMx is powered."]
    On = 1,
}
impl From<Sram3on> for bool {
    #[inline(always)]
    fn from(variant: Sram3on) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM3ON` reader - SRAM Power Control"]
pub type Sram3onR = crate::BitReader<Sram3on>;
impl Sram3onR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram3on {
        match self.bits {
            false => Sram3on::Off,
            true => Sram3on::On,
        }
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Sram3on::Off
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Sram3on::On
    }
}
#[doc = "Field `SRAM3ON` writer - SRAM Power Control"]
pub type Sram3onW<'a, REG> = crate::BitWriter<'a, REG, Sram3on>;
impl<'a, REG> Sram3onW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Sram3on::Off)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sram3on::On)
    }
}
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram4on {
    #[doc = "0: SRAMx is not powered."]
    Off = 0,
    #[doc = "1: SRAMx is powered."]
    On = 1,
}
impl From<Sram4on> for bool {
    #[inline(always)]
    fn from(variant: Sram4on) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM4ON` reader - SRAM Power Control"]
pub type Sram4onR = crate::BitReader<Sram4on>;
impl Sram4onR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram4on {
        match self.bits {
            false => Sram4on::Off,
            true => Sram4on::On,
        }
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Sram4on::Off
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Sram4on::On
    }
}
#[doc = "Field `SRAM4ON` writer - SRAM Power Control"]
pub type Sram4onW<'a, REG> = crate::BitWriter<'a, REG, Sram4on>;
impl<'a, REG> Sram4onW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Sram4on::Off)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sram4on::On)
    }
}
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram5on {
    #[doc = "0: SRAMx is not powered."]
    Off = 0,
    #[doc = "1: SRAMx is powered."]
    On = 1,
}
impl From<Sram5on> for bool {
    #[inline(always)]
    fn from(variant: Sram5on) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM5ON` reader - SRAM Power Control"]
pub type Sram5onR = crate::BitReader<Sram5on>;
impl Sram5onR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram5on {
        match self.bits {
            false => Sram5on::Off,
            true => Sram5on::On,
        }
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Sram5on::Off
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Sram5on::On
    }
}
#[doc = "Field `SRAM5ON` writer - SRAM Power Control"]
pub type Sram5onW<'a, REG> = crate::BitWriter<'a, REG, Sram5on>;
impl<'a, REG> Sram5onW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Sram5on::Off)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sram5on::On)
    }
}
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram6on {
    #[doc = "0: SRAMx is not powered."]
    Off = 0,
    #[doc = "1: SRAMx is powered."]
    On = 1,
}
impl From<Sram6on> for bool {
    #[inline(always)]
    fn from(variant: Sram6on) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM6ON` reader - SRAM Power Control"]
pub type Sram6onR = crate::BitReader<Sram6on>;
impl Sram6onR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram6on {
        match self.bits {
            false => Sram6on::Off,
            true => Sram6on::On,
        }
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Sram6on::Off
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Sram6on::On
    }
}
#[doc = "Field `SRAM6ON` writer - SRAM Power Control"]
pub type Sram6onW<'a, REG> = crate::BitWriter<'a, REG, Sram6on>;
impl<'a, REG> Sram6onW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Sram6on::Off)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sram6on::On)
    }
}
#[doc = "Dual-port RAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpramon {
    #[doc = "0: USB dual-port RAM is not powered."]
    Off = 0,
    #[doc = "1: USB dual-port RAM is powered."]
    On = 1,
}
impl From<Dpramon> for bool {
    #[inline(always)]
    fn from(variant: Dpramon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPRAMON` reader - Dual-port RAM Power Control"]
pub type DpramonR = crate::BitReader<Dpramon>;
impl DpramonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpramon {
        match self.bits {
            false => Dpramon::Off,
            true => Dpramon::On,
        }
    }
    #[doc = "USB dual-port RAM is not powered."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Dpramon::Off
    }
    #[doc = "USB dual-port RAM is powered."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Dpramon::On
    }
}
#[doc = "Field `DPRAMON` writer - Dual-port RAM Power Control"]
pub type DpramonW<'a, REG> = crate::BitWriter<'a, REG, Dpramon>;
impl<'a, REG> DpramonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB dual-port RAM is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Dpramon::Off)
    }
    #[doc = "USB dual-port RAM is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Dpramon::On)
    }
}
#[doc = "Password Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "90: Writing any other value in this field aborts the write operation.Always reads as 0."]
    Passwd = 90,
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
            90 => Some(Key::Passwd),
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
    #[doc = "Bit 0 - Low Power Value Selection"]
    #[inline(always)]
    pub fn lpowers(&self) -> LpowersR {
        LpowersR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Power Value"]
    #[inline(always)]
    pub fn lpower0(&self) -> Lpower0R {
        Lpower0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Power Value"]
    #[inline(always)]
    pub fn lpower1(&self) -> Lpower1R {
        Lpower1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low Power Value"]
    #[inline(always)]
    pub fn lpower2(&self) -> Lpower2R {
        Lpower2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Power Value"]
    #[inline(always)]
    pub fn lpower3(&self) -> Lpower3R {
        Lpower3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Start-up Time when Resuming from Wait Mode"]
    #[inline(always)]
    pub fn stuptime(&self) -> StuptimeR {
        StuptimeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enhanced Custom Power Value Selection"]
    #[inline(always)]
    pub fn ecpwrs(&self) -> EcpwrsR {
        EcpwrsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr0(&self) -> Ecpwr0R {
        Ecpwr0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr1(&self) -> Ecpwr1R {
        Ecpwr1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr2(&self) -> Ecpwr2R {
        Ecpwr2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr3(&self) -> Ecpwr3R {
        Ecpwr3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram0on(&self) -> Sram0onR {
        Sram0onR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram1on(&self) -> Sram1onR {
        Sram1onR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram2on(&self) -> Sram2onR {
        Sram2onR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram3on(&self) -> Sram3onR {
        Sram3onR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram4on(&self) -> Sram4onR {
        Sram4onR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram5on(&self) -> Sram5onR {
        Sram5onR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram6on(&self) -> Sram6onR {
        Sram6onR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Dual-port RAM Power Control"]
    #[inline(always)]
    pub fn dpramon(&self) -> DpramonR {
        DpramonR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Value Selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpowers(&mut self) -> LpowersW<PwmrSpec> {
        LpowersW::new(self, 0)
    }
    #[doc = "Bit 1 - Low Power Value"]
    #[inline(always)]
    #[must_use]
    pub fn lpower0(&mut self) -> Lpower0W<PwmrSpec> {
        Lpower0W::new(self, 1)
    }
    #[doc = "Bit 2 - Low Power Value"]
    #[inline(always)]
    #[must_use]
    pub fn lpower1(&mut self) -> Lpower1W<PwmrSpec> {
        Lpower1W::new(self, 2)
    }
    #[doc = "Bit 3 - Low Power Value"]
    #[inline(always)]
    #[must_use]
    pub fn lpower2(&mut self) -> Lpower2W<PwmrSpec> {
        Lpower2W::new(self, 3)
    }
    #[doc = "Bit 4 - Low Power Value"]
    #[inline(always)]
    #[must_use]
    pub fn lpower3(&mut self) -> Lpower3W<PwmrSpec> {
        Lpower3W::new(self, 4)
    }
    #[doc = "Bit 7 - Start-up Time when Resuming from Wait Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stuptime(&mut self) -> StuptimeW<PwmrSpec> {
        StuptimeW::new(self, 7)
    }
    #[doc = "Bit 8 - Enhanced Custom Power Value Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ecpwrs(&mut self) -> EcpwrsW<PwmrSpec> {
        EcpwrsW::new(self, 8)
    }
    #[doc = "Bit 9 - Enhanced Custom Power Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecpwr0(&mut self) -> Ecpwr0W<PwmrSpec> {
        Ecpwr0W::new(self, 9)
    }
    #[doc = "Bit 10 - Enhanced Custom Power Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecpwr1(&mut self) -> Ecpwr1W<PwmrSpec> {
        Ecpwr1W::new(self, 10)
    }
    #[doc = "Bit 11 - Enhanced Custom Power Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecpwr2(&mut self) -> Ecpwr2W<PwmrSpec> {
        Ecpwr2W::new(self, 11)
    }
    #[doc = "Bit 12 - Enhanced Custom Power Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecpwr3(&mut self) -> Ecpwr3W<PwmrSpec> {
        Ecpwr3W::new(self, 12)
    }
    #[doc = "Bit 16 - SRAM Power Control"]
    #[inline(always)]
    #[must_use]
    pub fn sram0on(&mut self) -> Sram0onW<PwmrSpec> {
        Sram0onW::new(self, 16)
    }
    #[doc = "Bit 17 - SRAM Power Control"]
    #[inline(always)]
    #[must_use]
    pub fn sram1on(&mut self) -> Sram1onW<PwmrSpec> {
        Sram1onW::new(self, 17)
    }
    #[doc = "Bit 18 - SRAM Power Control"]
    #[inline(always)]
    #[must_use]
    pub fn sram2on(&mut self) -> Sram2onW<PwmrSpec> {
        Sram2onW::new(self, 18)
    }
    #[doc = "Bit 19 - SRAM Power Control"]
    #[inline(always)]
    #[must_use]
    pub fn sram3on(&mut self) -> Sram3onW<PwmrSpec> {
        Sram3onW::new(self, 19)
    }
    #[doc = "Bit 20 - SRAM Power Control"]
    #[inline(always)]
    #[must_use]
    pub fn sram4on(&mut self) -> Sram4onW<PwmrSpec> {
        Sram4onW::new(self, 20)
    }
    #[doc = "Bit 21 - SRAM Power Control"]
    #[inline(always)]
    #[must_use]
    pub fn sram5on(&mut self) -> Sram5onW<PwmrSpec> {
        Sram5onW::new(self, 21)
    }
    #[doc = "Bit 22 - SRAM Power Control"]
    #[inline(always)]
    #[must_use]
    pub fn sram6on(&mut self) -> Sram6onW<PwmrSpec> {
        Sram6onW::new(self, 22)
    }
    #[doc = "Bit 23 - Dual-port RAM Power Control"]
    #[inline(always)]
    #[must_use]
    pub fn dpramon(&mut self) -> DpramonW<PwmrSpec> {
        DpramonW::new(self, 23)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<PwmrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Supply Controller Power Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmrSpec;
impl crate::RegisterSpec for PwmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmr::R`](R) reader structure"]
impl crate::Readable for PwmrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmr::W`](W) writer structure"]
impl crate::Writable for PwmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMR to value 0"]
impl crate::Resettable for PwmrSpec {
    const RESET_VALUE: u32 = 0;
}
