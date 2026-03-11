#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Field `DATAL` reader - Data Length"]
pub type DatalR = crate::FieldReader;
#[doc = "Field `DATAL` writer - Data Length"]
pub type DatalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DIR` reader - Transfer Direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Transfer Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEC` reader - PEC Request (SMBus Mode only)"]
pub type PecR = crate::BitReader;
#[doc = "Field `PEC` writer - PEC Request (SMBus Mode only)"]
pub type PecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDATAL` reader - Next Data Length"]
pub type NdatalR = crate::FieldReader;
#[doc = "Field `NDATAL` writer - Next Data Length"]
pub type NdatalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NDIR` reader - Next Transfer Direction"]
pub type NdirR = crate::BitReader;
#[doc = "Field `NDIR` writer - Next Transfer Direction"]
pub type NdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPEC` reader - Next PEC Request (SMBus Mode only)"]
pub type NpecR = crate::BitReader;
#[doc = "Field `NPEC` writer - Next PEC Request (SMBus Mode only)"]
pub type NpecW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn datal(&self) -> DatalR {
        DatalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Transfer Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PEC Request (SMBus Mode only)"]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Next Data Length"]
    #[inline(always)]
    pub fn ndatal(&self) -> NdatalR {
        NdatalR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Next Transfer Direction"]
    #[inline(always)]
    pub fn ndir(&self) -> NdirR {
        NdirR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Next PEC Request (SMBus Mode only)"]
    #[inline(always)]
    pub fn npec(&self) -> NpecR {
        NpecR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn datal(&mut self) -> DatalW<AcrSpec> {
        DatalW::new(self, 0)
    }
    #[doc = "Bit 8 - Transfer Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<AcrSpec> {
        DirW::new(self, 8)
    }
    #[doc = "Bit 9 - PEC Request (SMBus Mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn pec(&mut self) -> PecW<AcrSpec> {
        PecW::new(self, 9)
    }
    #[doc = "Bits 16:23 - Next Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn ndatal(&mut self) -> NdatalW<AcrSpec> {
        NdatalW::new(self, 16)
    }
    #[doc = "Bit 24 - Next Transfer Direction"]
    #[inline(always)]
    #[must_use]
    pub fn ndir(&mut self) -> NdirW<AcrSpec> {
        NdirW::new(self, 24)
    }
    #[doc = "Bit 25 - Next PEC Request (SMBus Mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn npec(&mut self) -> NpecW<AcrSpec> {
        NpecW::new(self, 25)
    }
}
#[doc = "TWI Alternative Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
