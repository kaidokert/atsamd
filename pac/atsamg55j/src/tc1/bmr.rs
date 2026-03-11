#[doc = "Register `BMR` reader"]
pub type R = crate::R<BmrSpec>;
#[doc = "Register `BMR` writer"]
pub type W = crate::W<BmrSpec>;
#[doc = "External Clock Signal 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tc0xc0s {
    #[doc = "0: Signal connected to XC0: TCLK0"]
    Tclk0 = 0,
    #[doc = "2: Signal connected to XC0: TIOA1"]
    Tioa1 = 2,
    #[doc = "3: Signal connected to XC0: TIOA2"]
    Tioa2 = 3,
}
impl From<Tc0xc0s> for u8 {
    #[inline(always)]
    fn from(variant: Tc0xc0s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tc0xc0s {
    type Ux = u8;
}
impl crate::IsEnum for Tc0xc0s {}
#[doc = "Field `TC0XC0S` reader - External Clock Signal 0 Selection"]
pub type Tc0xc0sR = crate::FieldReader<Tc0xc0s>;
impl Tc0xc0sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tc0xc0s> {
        match self.bits {
            0 => Some(Tc0xc0s::Tclk0),
            2 => Some(Tc0xc0s::Tioa1),
            3 => Some(Tc0xc0s::Tioa2),
            _ => None,
        }
    }
    #[doc = "Signal connected to XC0: TCLK0"]
    #[inline(always)]
    pub fn is_tclk0(&self) -> bool {
        *self == Tc0xc0s::Tclk0
    }
    #[doc = "Signal connected to XC0: TIOA1"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == Tc0xc0s::Tioa1
    }
    #[doc = "Signal connected to XC0: TIOA2"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == Tc0xc0s::Tioa2
    }
}
#[doc = "Field `TC0XC0S` writer - External Clock Signal 0 Selection"]
pub type Tc0xc0sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tc0xc0s>;
impl<'a, REG> Tc0xc0sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal connected to XC0: TCLK0"]
    #[inline(always)]
    pub fn tclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Tc0xc0s::Tclk0)
    }
    #[doc = "Signal connected to XC0: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut crate::W<REG> {
        self.variant(Tc0xc0s::Tioa1)
    }
    #[doc = "Signal connected to XC0: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut crate::W<REG> {
        self.variant(Tc0xc0s::Tioa2)
    }
}
#[doc = "External Clock Signal 1 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tc1xc1s {
    #[doc = "0: Signal connected to XC1: TCLK1"]
    Tclk1 = 0,
    #[doc = "2: Signal connected to XC1: TIOA0"]
    Tioa0 = 2,
    #[doc = "3: Signal connected to XC1: TIOA2"]
    Tioa2 = 3,
}
impl From<Tc1xc1s> for u8 {
    #[inline(always)]
    fn from(variant: Tc1xc1s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tc1xc1s {
    type Ux = u8;
}
impl crate::IsEnum for Tc1xc1s {}
#[doc = "Field `TC1XC1S` reader - External Clock Signal 1 Selection"]
pub type Tc1xc1sR = crate::FieldReader<Tc1xc1s>;
impl Tc1xc1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tc1xc1s> {
        match self.bits {
            0 => Some(Tc1xc1s::Tclk1),
            2 => Some(Tc1xc1s::Tioa0),
            3 => Some(Tc1xc1s::Tioa2),
            _ => None,
        }
    }
    #[doc = "Signal connected to XC1: TCLK1"]
    #[inline(always)]
    pub fn is_tclk1(&self) -> bool {
        *self == Tc1xc1s::Tclk1
    }
    #[doc = "Signal connected to XC1: TIOA0"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == Tc1xc1s::Tioa0
    }
    #[doc = "Signal connected to XC1: TIOA2"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == Tc1xc1s::Tioa2
    }
}
#[doc = "Field `TC1XC1S` writer - External Clock Signal 1 Selection"]
pub type Tc1xc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tc1xc1s>;
impl<'a, REG> Tc1xc1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal connected to XC1: TCLK1"]
    #[inline(always)]
    pub fn tclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Tc1xc1s::Tclk1)
    }
    #[doc = "Signal connected to XC1: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut crate::W<REG> {
        self.variant(Tc1xc1s::Tioa0)
    }
    #[doc = "Signal connected to XC1: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut crate::W<REG> {
        self.variant(Tc1xc1s::Tioa2)
    }
}
#[doc = "External Clock Signal 2 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tc2xc2s {
    #[doc = "0: Signal connected to XC2: TCLK2"]
    Tclk2 = 0,
    #[doc = "2: Signal connected to XC2: TIOA0"]
    Tioa0 = 2,
    #[doc = "3: Signal connected to XC2: TIOA1"]
    Tioa1 = 3,
}
impl From<Tc2xc2s> for u8 {
    #[inline(always)]
    fn from(variant: Tc2xc2s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tc2xc2s {
    type Ux = u8;
}
impl crate::IsEnum for Tc2xc2s {}
#[doc = "Field `TC2XC2S` reader - External Clock Signal 2 Selection"]
pub type Tc2xc2sR = crate::FieldReader<Tc2xc2s>;
impl Tc2xc2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tc2xc2s> {
        match self.bits {
            0 => Some(Tc2xc2s::Tclk2),
            2 => Some(Tc2xc2s::Tioa0),
            3 => Some(Tc2xc2s::Tioa1),
            _ => None,
        }
    }
    #[doc = "Signal connected to XC2: TCLK2"]
    #[inline(always)]
    pub fn is_tclk2(&self) -> bool {
        *self == Tc2xc2s::Tclk2
    }
    #[doc = "Signal connected to XC2: TIOA0"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == Tc2xc2s::Tioa0
    }
    #[doc = "Signal connected to XC2: TIOA1"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == Tc2xc2s::Tioa1
    }
}
#[doc = "Field `TC2XC2S` writer - External Clock Signal 2 Selection"]
pub type Tc2xc2sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tc2xc2s>;
impl<'a, REG> Tc2xc2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal connected to XC2: TCLK2"]
    #[inline(always)]
    pub fn tclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Tc2xc2s::Tclk2)
    }
    #[doc = "Signal connected to XC2: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut crate::W<REG> {
        self.variant(Tc2xc2s::Tioa0)
    }
    #[doc = "Signal connected to XC2: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut crate::W<REG> {
        self.variant(Tc2xc2s::Tioa1)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&self) -> Tc0xc0sR {
        Tc0xc0sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&self) -> Tc1xc1sR {
        Tc1xc1sR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&self) -> Tc2xc2sR {
        Tc2xc2sR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc0xc0s(&mut self) -> Tc0xc0sW<BmrSpec> {
        Tc0xc0sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc1xc1s(&mut self) -> Tc1xc1sW<BmrSpec> {
        Tc1xc1sW::new(self, 2)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc2xc2s(&mut self) -> Tc2xc2sW<BmrSpec> {
        Tc2xc2sW::new(self, 4)
    }
}
#[doc = "Block Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmrSpec;
impl crate::RegisterSpec for BmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmr::R`](R) reader structure"]
impl crate::Readable for BmrSpec {}
#[doc = "`write(|w| ..)` method takes [`bmr::W`](W) writer structure"]
impl crate::Writable for BmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BMR to value 0"]
impl crate::Resettable for BmrSpec {
    const RESET_VALUE: u32 = 0;
}
