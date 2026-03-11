#[doc = "Register `HCRHDESCRIPTORA` reader"]
pub type R = crate::R<HcrhdescriptoraSpec>;
#[doc = "Register `HCRHDESCRIPTORA` writer"]
pub type W = crate::W<HcrhdescriptoraSpec>;
#[doc = "Field `NDP` reader - Number of downstream ports (read-only)"]
pub type NdpR = crate::FieldReader;
#[doc = "Field `NDP` writer - Number of downstream ports (read-only)"]
pub type NdpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PSM` reader - Power switching mode (read/write)"]
pub type PsmR = crate::BitReader;
#[doc = "Field `PSM` writer - Power switching mode (read/write)"]
pub type PsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPS` reader - No power switching (read/write)"]
pub type NpsR = crate::BitReader;
#[doc = "Field `NPS` writer - No power switching (read/write)"]
pub type NpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT` reader - Device type (read-only)"]
pub type DtR = crate::BitReader;
#[doc = "Field `DT` writer - Device type (read-only)"]
pub type DtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPM` reader - Overcurrent protection mode (read/write)"]
pub type OcpmR = crate::BitReader;
#[doc = "Field `OCPM` writer - Overcurrent protection mode (read/write)"]
pub type OcpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOCP` reader - No overcurrent protection (read/write)"]
pub type NocpR = crate::BitReader;
#[doc = "Field `NOCP` writer - No overcurrent protection (read/write)"]
pub type NocpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POTPG` reader - Power-on to power-good time (read/write)"]
pub type PotpgR = crate::FieldReader;
#[doc = "Field `POTPG` writer - Power-on to power-good time (read/write)"]
pub type PotpgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Number of downstream ports (read-only)"]
    #[inline(always)]
    pub fn ndp(&self) -> NdpR {
        NdpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Power switching mode (read/write)"]
    #[inline(always)]
    pub fn psm(&self) -> PsmR {
        PsmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - No power switching (read/write)"]
    #[inline(always)]
    pub fn nps(&self) -> NpsR {
        NpsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Device type (read-only)"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overcurrent protection mode (read/write)"]
    #[inline(always)]
    pub fn ocpm(&self) -> OcpmR {
        OcpmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - No overcurrent protection (read/write)"]
    #[inline(always)]
    pub fn nocp(&self) -> NocpR {
        NocpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Power-on to power-good time (read/write)"]
    #[inline(always)]
    pub fn potpg(&self) -> PotpgR {
        PotpgR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of downstream ports (read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn ndp(&mut self) -> NdpW<HcrhdescriptoraSpec> {
        NdpW::new(self, 0)
    }
    #[doc = "Bit 8 - Power switching mode (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn psm(&mut self) -> PsmW<HcrhdescriptoraSpec> {
        PsmW::new(self, 8)
    }
    #[doc = "Bit 9 - No power switching (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn nps(&mut self) -> NpsW<HcrhdescriptoraSpec> {
        NpsW::new(self, 9)
    }
    #[doc = "Bit 10 - Device type (read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DtW<HcrhdescriptoraSpec> {
        DtW::new(self, 10)
    }
    #[doc = "Bit 11 - Overcurrent protection mode (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn ocpm(&mut self) -> OcpmW<HcrhdescriptoraSpec> {
        OcpmW::new(self, 11)
    }
    #[doc = "Bit 12 - No overcurrent protection (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn nocp(&mut self) -> NocpW<HcrhdescriptoraSpec> {
        NocpW::new(self, 12)
    }
    #[doc = "Bits 24:31 - Power-on to power-good time (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn potpg(&mut self) -> PotpgW<HcrhdescriptoraSpec> {
        PotpgW::new(self, 24)
    }
}
#[doc = "HC Root Hub A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhdescriptora::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhdescriptora::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcrhdescriptoraSpec;
impl crate::RegisterSpec for HcrhdescriptoraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcrhdescriptora::R`](R) reader structure"]
impl crate::Readable for HcrhdescriptoraSpec {}
#[doc = "`write(|w| ..)` method takes [`hcrhdescriptora::W`](W) writer structure"]
impl crate::Writable for HcrhdescriptoraSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCRHDESCRIPTORA to value 0x0a00_1203"]
impl crate::Resettable for HcrhdescriptoraSpec {
    const RESET_VALUE: u32 = 0x0a00_1203;
}
