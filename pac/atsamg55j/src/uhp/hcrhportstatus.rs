#[doc = "Register `HCRHPORTSTATUS[%s]` reader"]
pub type R = crate::R<HcrhportstatusSpec>;
#[doc = "Register `HCRHPORTSTATUS[%s]` writer"]
pub type W = crate::W<HcrhportstatusSpec>;
#[doc = "Field `CCS_CPE` reader - "]
pub type CcsCpeR = crate::BitReader;
#[doc = "Field `CCS_CPE` writer - "]
pub type CcsCpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_SPE` reader - "]
pub type PesSpeR = crate::BitReader;
#[doc = "Field `PES_SPE` writer - "]
pub type PesSpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSS_SPS` reader - "]
pub type PssSpsR = crate::BitReader;
#[doc = "Field `PSS_SPS` writer - "]
pub type PssSpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POCI_CSS` reader - "]
pub type PociCssR = crate::BitReader;
#[doc = "Field `POCI_CSS` writer - "]
pub type PociCssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS_SPR` reader - "]
pub type PrsSprR = crate::BitReader;
#[doc = "Field `PRS_SPR` writer - "]
pub type PrsSprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPS_SPP` reader - "]
pub type PpsSppR = crate::BitReader;
#[doc = "Field `PPS_SPP` writer - "]
pub type PpsSppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSDA_CPP` reader - "]
pub type LsdaCppR = crate::BitReader;
#[doc = "Field `LSDA_CPP` writer - "]
pub type LsdaCppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC` reader - Port 1 connect status change (read/write, write '1' to clear)"]
pub type CscR = crate::BitReader;
#[doc = "Field `CSC` writer - Port 1 connect status change (read/write, write '1' to clear)"]
pub type CscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PESC` reader - Port 1 enable status change (read/write, write '1' to clear)"]
pub type PescR = crate::BitReader;
#[doc = "Field `PESC` writer - Port 1 enable status change (read/write, write '1' to clear)"]
pub type PescW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSSC` reader - Port 1 suspend status change (read/write, write '1' to clear)"]
pub type PsscR = crate::BitReader;
#[doc = "Field `PSSC` writer - Port 1 suspend status change (read/write, write '1' to clear)"]
pub type PsscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIC` reader - Port 1 overcurrent indicator change (read/write)"]
pub type OcicR = crate::BitReader;
#[doc = "Field `OCIC` writer - Port 1 overcurrent indicator change (read/write)"]
pub type OcicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSC` reader - Port 1 reset status change (read/write, write '1' to clear)"]
pub type PrscR = crate::BitReader;
#[doc = "Field `PRSC` writer - Port 1 reset status change (read/write, write '1' to clear)"]
pub type PrscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ccs_cpe(&self) -> CcsCpeR {
        CcsCpeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pes_spe(&self) -> PesSpeR {
        PesSpeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pss_sps(&self) -> PssSpsR {
        PssSpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn poci_css(&self) -> PociCssR {
        PociCssR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn prs_spr(&self) -> PrsSprR {
        PrsSprR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pps_spp(&self) -> PpsSppR {
        PpsSppR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lsda_cpp(&self) -> LsdaCppR {
        LsdaCppR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Port 1 connect status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn csc(&self) -> CscR {
        CscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port 1 enable status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn pesc(&self) -> PescR {
        PescR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port 1 suspend status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn pssc(&self) -> PsscR {
        PsscR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port 1 overcurrent indicator change (read/write)"]
    #[inline(always)]
    pub fn ocic(&self) -> OcicR {
        OcicR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port 1 reset status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn prsc(&self) -> PrscR {
        PrscR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ccs_cpe(&mut self) -> CcsCpeW<HcrhportstatusSpec> {
        CcsCpeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pes_spe(&mut self) -> PesSpeW<HcrhportstatusSpec> {
        PesSpeW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pss_sps(&mut self) -> PssSpsW<HcrhportstatusSpec> {
        PssSpsW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn poci_css(&mut self) -> PociCssW<HcrhportstatusSpec> {
        PociCssW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn prs_spr(&mut self) -> PrsSprW<HcrhportstatusSpec> {
        PrsSprW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pps_spp(&mut self) -> PpsSppW<HcrhportstatusSpec> {
        PpsSppW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn lsda_cpp(&mut self) -> LsdaCppW<HcrhportstatusSpec> {
        LsdaCppW::new(self, 9)
    }
    #[doc = "Bit 16 - Port 1 connect status change (read/write, write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn csc(&mut self) -> CscW<HcrhportstatusSpec> {
        CscW::new(self, 16)
    }
    #[doc = "Bit 17 - Port 1 enable status change (read/write, write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn pesc(&mut self) -> PescW<HcrhportstatusSpec> {
        PescW::new(self, 17)
    }
    #[doc = "Bit 18 - Port 1 suspend status change (read/write, write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn pssc(&mut self) -> PsscW<HcrhportstatusSpec> {
        PsscW::new(self, 18)
    }
    #[doc = "Bit 19 - Port 1 overcurrent indicator change (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn ocic(&mut self) -> OcicW<HcrhportstatusSpec> {
        OcicW::new(self, 19)
    }
    #[doc = "Bit 20 - Port 1 reset status change (read/write, write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn prsc(&mut self) -> PrscW<HcrhportstatusSpec> {
        PrscW::new(self, 20)
    }
}
#[doc = "HC Port 1 Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhportstatus::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhportstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcrhportstatusSpec;
impl crate::RegisterSpec for HcrhportstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcrhportstatus::R`](R) reader structure"]
impl crate::Readable for HcrhportstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hcrhportstatus::W`](W) writer structure"]
impl crate::Writable for HcrhportstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
