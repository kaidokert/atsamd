#[doc = "Register `MODR` reader"]
pub type R = crate::R<ModrSpec>;
#[doc = "Register `MODR` writer"]
pub type W = crate::W<ModrSpec>;
#[doc = "Selection of the 32-bit Counter Modulo to generate RTTINC2 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selinc2 {
    #[doc = "0: The RTTINC2 flag never rises"]
    NoRttinc2 = 0,
    #[doc = "1: The RTTINC2 flag is set when CRTV modulo 64 equals 0"]
    Mod64 = 1,
    #[doc = "2: The RTTINC2 flag is set when CRTV modulo 128 equals 0"]
    Mod128 = 2,
    #[doc = "3: The RTTINC2 flag is set when CRTV modulo 256 equals 0"]
    Mod256 = 3,
    #[doc = "4: The RTTINC2 flag is set when CRTV modulo 512 equals 0"]
    Mod512 = 4,
    #[doc = "5: The RTTINC2 flag is set when CRTV modulo 1024 equals 0.Example: If RTPRES=32 then RTTINC2 flag rises once per second if the slow clock is 32.768 kHz."]
    Mod1024 = 5,
    #[doc = "6: The RTTINC2 flag is set when CRTV modulo 2048 equals 0"]
    Mod2048 = 6,
    #[doc = "7: The RTTINC2 flag is set when CRTV modulo 4096 equals 0"]
    Mod4096 = 7,
}
impl From<Selinc2> for u8 {
    #[inline(always)]
    fn from(variant: Selinc2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selinc2 {
    type Ux = u8;
}
impl crate::IsEnum for Selinc2 {}
#[doc = "Field `SELINC2` reader - Selection of the 32-bit Counter Modulo to generate RTTINC2 flag"]
pub type Selinc2R = crate::FieldReader<Selinc2>;
impl Selinc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selinc2 {
        match self.bits {
            0 => Selinc2::NoRttinc2,
            1 => Selinc2::Mod64,
            2 => Selinc2::Mod128,
            3 => Selinc2::Mod256,
            4 => Selinc2::Mod512,
            5 => Selinc2::Mod1024,
            6 => Selinc2::Mod2048,
            7 => Selinc2::Mod4096,
            _ => unreachable!(),
        }
    }
    #[doc = "The RTTINC2 flag never rises"]
    #[inline(always)]
    pub fn is_no_rttinc2(&self) -> bool {
        *self == Selinc2::NoRttinc2
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 64 equals 0"]
    #[inline(always)]
    pub fn is_mod64(&self) -> bool {
        *self == Selinc2::Mod64
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 128 equals 0"]
    #[inline(always)]
    pub fn is_mod128(&self) -> bool {
        *self == Selinc2::Mod128
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 256 equals 0"]
    #[inline(always)]
    pub fn is_mod256(&self) -> bool {
        *self == Selinc2::Mod256
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 512 equals 0"]
    #[inline(always)]
    pub fn is_mod512(&self) -> bool {
        *self == Selinc2::Mod512
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 1024 equals 0.Example: If RTPRES=32 then RTTINC2 flag rises once per second if the slow clock is 32.768 kHz."]
    #[inline(always)]
    pub fn is_mod1024(&self) -> bool {
        *self == Selinc2::Mod1024
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 2048 equals 0"]
    #[inline(always)]
    pub fn is_mod2048(&self) -> bool {
        *self == Selinc2::Mod2048
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 4096 equals 0"]
    #[inline(always)]
    pub fn is_mod4096(&self) -> bool {
        *self == Selinc2::Mod4096
    }
}
#[doc = "Field `SELINC2` writer - Selection of the 32-bit Counter Modulo to generate RTTINC2 flag"]
pub type Selinc2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Selinc2, crate::Safe>;
impl<'a, REG> Selinc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The RTTINC2 flag never rises"]
    #[inline(always)]
    pub fn no_rttinc2(self) -> &'a mut crate::W<REG> {
        self.variant(Selinc2::NoRttinc2)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 64 equals 0"]
    #[inline(always)]
    pub fn mod64(self) -> &'a mut crate::W<REG> {
        self.variant(Selinc2::Mod64)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 128 equals 0"]
    #[inline(always)]
    pub fn mod128(self) -> &'a mut crate::W<REG> {
        self.variant(Selinc2::Mod128)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 256 equals 0"]
    #[inline(always)]
    pub fn mod256(self) -> &'a mut crate::W<REG> {
        self.variant(Selinc2::Mod256)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 512 equals 0"]
    #[inline(always)]
    pub fn mod512(self) -> &'a mut crate::W<REG> {
        self.variant(Selinc2::Mod512)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 1024 equals 0.Example: If RTPRES=32 then RTTINC2 flag rises once per second if the slow clock is 32.768 kHz."]
    #[inline(always)]
    pub fn mod1024(self) -> &'a mut crate::W<REG> {
        self.variant(Selinc2::Mod1024)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 2048 equals 0"]
    #[inline(always)]
    pub fn mod2048(self) -> &'a mut crate::W<REG> {
        self.variant(Selinc2::Mod2048)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 4096 equals 0"]
    #[inline(always)]
    pub fn mod4096(self) -> &'a mut crate::W<REG> {
        self.variant(Selinc2::Mod4096)
    }
}
#[doc = "Selection of the 32-bit Counter Modulo to generate the trigger event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Seltrgev {
    #[doc = "0: No event generated"]
    NoEvent = 0,
    #[doc = "1: Event occurs when CRTV modulo 2 equals 0"]
    Mod2 = 1,
    #[doc = "2: Event occurs when CRTV modulo 4 equals 0"]
    Mod4 = 2,
    #[doc = "3: Event occurs when CRTV modulo 8 equals 0"]
    Mod8 = 3,
    #[doc = "4: Event occurs when CRTV modulo 16 equals 0"]
    Mod16 = 4,
    #[doc = "5: Event occurs when CRTV modulo 32 equals 0"]
    Mod32 = 5,
    #[doc = "6: Event occurs when CRTV modulo 64 equals 0"]
    Mod64 = 6,
    #[doc = "7: Event occurs when CRTV modulo 128 equals 0"]
    Mod128 = 7,
}
impl From<Seltrgev> for u8 {
    #[inline(always)]
    fn from(variant: Seltrgev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Seltrgev {
    type Ux = u8;
}
impl crate::IsEnum for Seltrgev {}
#[doc = "Field `SELTRGEV` reader - Selection of the 32-bit Counter Modulo to generate the trigger event"]
pub type SeltrgevR = crate::FieldReader<Seltrgev>;
impl SeltrgevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seltrgev {
        match self.bits {
            0 => Seltrgev::NoEvent,
            1 => Seltrgev::Mod2,
            2 => Seltrgev::Mod4,
            3 => Seltrgev::Mod8,
            4 => Seltrgev::Mod16,
            5 => Seltrgev::Mod32,
            6 => Seltrgev::Mod64,
            7 => Seltrgev::Mod128,
            _ => unreachable!(),
        }
    }
    #[doc = "No event generated"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == Seltrgev::NoEvent
    }
    #[doc = "Event occurs when CRTV modulo 2 equals 0"]
    #[inline(always)]
    pub fn is_mod2(&self) -> bool {
        *self == Seltrgev::Mod2
    }
    #[doc = "Event occurs when CRTV modulo 4 equals 0"]
    #[inline(always)]
    pub fn is_mod4(&self) -> bool {
        *self == Seltrgev::Mod4
    }
    #[doc = "Event occurs when CRTV modulo 8 equals 0"]
    #[inline(always)]
    pub fn is_mod8(&self) -> bool {
        *self == Seltrgev::Mod8
    }
    #[doc = "Event occurs when CRTV modulo 16 equals 0"]
    #[inline(always)]
    pub fn is_mod16(&self) -> bool {
        *self == Seltrgev::Mod16
    }
    #[doc = "Event occurs when CRTV modulo 32 equals 0"]
    #[inline(always)]
    pub fn is_mod32(&self) -> bool {
        *self == Seltrgev::Mod32
    }
    #[doc = "Event occurs when CRTV modulo 64 equals 0"]
    #[inline(always)]
    pub fn is_mod64(&self) -> bool {
        *self == Seltrgev::Mod64
    }
    #[doc = "Event occurs when CRTV modulo 128 equals 0"]
    #[inline(always)]
    pub fn is_mod128(&self) -> bool {
        *self == Seltrgev::Mod128
    }
}
#[doc = "Field `SELTRGEV` writer - Selection of the 32-bit Counter Modulo to generate the trigger event"]
pub type SeltrgevW<'a, REG> = crate::FieldWriter<'a, REG, 3, Seltrgev, crate::Safe>;
impl<'a, REG> SeltrgevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event generated"]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(Seltrgev::NoEvent)
    }
    #[doc = "Event occurs when CRTV modulo 2 equals 0"]
    #[inline(always)]
    pub fn mod2(self) -> &'a mut crate::W<REG> {
        self.variant(Seltrgev::Mod2)
    }
    #[doc = "Event occurs when CRTV modulo 4 equals 0"]
    #[inline(always)]
    pub fn mod4(self) -> &'a mut crate::W<REG> {
        self.variant(Seltrgev::Mod4)
    }
    #[doc = "Event occurs when CRTV modulo 8 equals 0"]
    #[inline(always)]
    pub fn mod8(self) -> &'a mut crate::W<REG> {
        self.variant(Seltrgev::Mod8)
    }
    #[doc = "Event occurs when CRTV modulo 16 equals 0"]
    #[inline(always)]
    pub fn mod16(self) -> &'a mut crate::W<REG> {
        self.variant(Seltrgev::Mod16)
    }
    #[doc = "Event occurs when CRTV modulo 32 equals 0"]
    #[inline(always)]
    pub fn mod32(self) -> &'a mut crate::W<REG> {
        self.variant(Seltrgev::Mod32)
    }
    #[doc = "Event occurs when CRTV modulo 64 equals 0"]
    #[inline(always)]
    pub fn mod64(self) -> &'a mut crate::W<REG> {
        self.variant(Seltrgev::Mod64)
    }
    #[doc = "Event occurs when CRTV modulo 128 equals 0"]
    #[inline(always)]
    pub fn mod128(self) -> &'a mut crate::W<REG> {
        self.variant(Seltrgev::Mod128)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selection of the 32-bit Counter Modulo to generate RTTINC2 flag"]
    #[inline(always)]
    pub fn selinc2(&self) -> Selinc2R {
        Selinc2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Selection of the 32-bit Counter Modulo to generate the trigger event"]
    #[inline(always)]
    pub fn seltrgev(&self) -> SeltrgevR {
        SeltrgevR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection of the 32-bit Counter Modulo to generate RTTINC2 flag"]
    #[inline(always)]
    #[must_use]
    pub fn selinc2(&mut self) -> Selinc2W<ModrSpec> {
        Selinc2W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Selection of the 32-bit Counter Modulo to generate the trigger event"]
    #[inline(always)]
    #[must_use]
    pub fn seltrgev(&mut self) -> SeltrgevW<ModrSpec> {
        SeltrgevW::new(self, 8)
    }
}
#[doc = "Modulo Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`modr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModrSpec;
impl crate::RegisterSpec for ModrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modr::R`](R) reader structure"]
impl crate::Readable for ModrSpec {}
#[doc = "`write(|w| ..)` method takes [`modr::W`](W) writer structure"]
impl crate::Writable for ModrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODR to value 0"]
impl crate::Resettable for ModrSpec {
    const RESET_VALUE: u32 = 0;
}
