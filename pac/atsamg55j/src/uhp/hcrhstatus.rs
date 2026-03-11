#[doc = "Register `HCRHSTATUS` reader"]
pub type R = crate::R<HcrhstatusSpec>;
#[doc = "Register `HCRHSTATUS` writer"]
pub type W = crate::W<HcrhstatusSpec>;
#[doc = "Field `LPS` reader - Local power status (read/write)"]
pub type LpsR = crate::BitReader;
#[doc = "Field `LPS` writer - Local power status (read/write)"]
pub type LpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCI` reader - Overcurrent indicator (read-only)"]
pub type OciR = crate::BitReader;
#[doc = "Field `OCI` writer - Overcurrent indicator (read-only)"]
pub type OciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRWE` reader - Device remote wake-up enable (read/write)"]
pub type DrweR = crate::BitReader;
#[doc = "Field `DRWE` writer - Device remote wake-up enable (read/write)"]
pub type DrweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPSC` reader - Local power status change (read/write)"]
pub type LpscR = crate::BitReader;
#[doc = "Field `LPSC` writer - Local power status change (read/write)"]
pub type LpscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIC` reader - Overcurrent indication change (read/write)"]
pub type OcicR = crate::BitReader;
#[doc = "Field `OCIC` writer - Overcurrent indication change (read/write)"]
pub type OcicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRWE` reader - Clear remote wake-up enable (read/write)"]
pub type CrweR = crate::BitReader;
#[doc = "Field `CRWE` writer - Clear remote wake-up enable (read/write)"]
pub type CrweW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Local power status (read/write)"]
    #[inline(always)]
    pub fn lps(&self) -> LpsR {
        LpsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overcurrent indicator (read-only)"]
    #[inline(always)]
    pub fn oci(&self) -> OciR {
        OciR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - Device remote wake-up enable (read/write)"]
    #[inline(always)]
    pub fn drwe(&self) -> DrweR {
        DrweR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Local power status change (read/write)"]
    #[inline(always)]
    pub fn lpsc(&self) -> LpscR {
        LpscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Overcurrent indication change (read/write)"]
    #[inline(always)]
    pub fn ocic(&self) -> OcicR {
        OcicR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Clear remote wake-up enable (read/write)"]
    #[inline(always)]
    pub fn crwe(&self) -> CrweR {
        CrweR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Local power status (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn lps(&mut self) -> LpsW<HcrhstatusSpec> {
        LpsW::new(self, 0)
    }
    #[doc = "Bit 1 - Overcurrent indicator (read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn oci(&mut self) -> OciW<HcrhstatusSpec> {
        OciW::new(self, 1)
    }
    #[doc = "Bit 15 - Device remote wake-up enable (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn drwe(&mut self) -> DrweW<HcrhstatusSpec> {
        DrweW::new(self, 15)
    }
    #[doc = "Bit 16 - Local power status change (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn lpsc(&mut self) -> LpscW<HcrhstatusSpec> {
        LpscW::new(self, 16)
    }
    #[doc = "Bit 17 - Overcurrent indication change (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn ocic(&mut self) -> OcicW<HcrhstatusSpec> {
        OcicW::new(self, 17)
    }
    #[doc = "Bit 31 - Clear remote wake-up enable (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn crwe(&mut self) -> CrweW<HcrhstatusSpec> {
        CrweW::new(self, 31)
    }
}
#[doc = "HC Root Hub Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcrhstatusSpec;
impl crate::RegisterSpec for HcrhstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcrhstatus::R`](R) reader structure"]
impl crate::Readable for HcrhstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hcrhstatus::W`](W) writer structure"]
impl crate::Writable for HcrhstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCRHSTATUS to value 0"]
impl crate::Resettable for HcrhstatusSpec {
    const RESET_VALUE: u32 = 0;
}
