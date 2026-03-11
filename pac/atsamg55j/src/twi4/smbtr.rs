#[doc = "Register `SMBTR` reader"]
pub type R = crate::R<SmbtrSpec>;
#[doc = "Register `SMBTR` writer"]
pub type W = crate::W<SmbtrSpec>;
#[doc = "Field `PRESC` reader - SMBus Clock Prescaler"]
pub type PrescR = crate::FieldReader;
#[doc = "Field `PRESC` writer - SMBus Clock Prescaler"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TLOWS` reader - Slave Clock Stretch Maximum Cycles"]
pub type TlowsR = crate::FieldReader;
#[doc = "Field `TLOWS` writer - Slave Clock Stretch Maximum Cycles"]
pub type TlowsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TLOWM` reader - Master Clock Stretch Maximum Cycles"]
pub type TlowmR = crate::FieldReader;
#[doc = "Field `TLOWM` writer - Master Clock Stretch Maximum Cycles"]
pub type TlowmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THMAX` reader - Clock High Maximum Cycles"]
pub type ThmaxR = crate::FieldReader;
#[doc = "Field `THMAX` writer - Clock High Maximum Cycles"]
pub type ThmaxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - SMBus Clock Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Slave Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlows(&self) -> TlowsR {
        TlowsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Master Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlowm(&self) -> TlowmR {
        TlowmR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock High Maximum Cycles"]
    #[inline(always)]
    pub fn thmax(&self) -> ThmaxR {
        ThmaxR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SMBus Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<SmbtrSpec> {
        PrescW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Slave Clock Stretch Maximum Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn tlows(&mut self) -> TlowsW<SmbtrSpec> {
        TlowsW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Master Clock Stretch Maximum Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn tlowm(&mut self) -> TlowmW<SmbtrSpec> {
        TlowmW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clock High Maximum Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn thmax(&mut self) -> ThmaxW<SmbtrSpec> {
        ThmaxW::new(self, 24)
    }
}
#[doc = "TWI SMBus Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smbtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbtrSpec;
impl crate::RegisterSpec for SmbtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smbtr::R`](R) reader structure"]
impl crate::Readable for SmbtrSpec {}
#[doc = "`write(|w| ..)` method takes [`smbtr::W`](W) writer structure"]
impl crate::Writable for SmbtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMBTR to value 0"]
impl crate::Resettable for SmbtrSpec {
    const RESET_VALUE: u32 = 0;
}
