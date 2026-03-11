#[doc = "Register `HCCOMMANDSTATUS` reader"]
pub type R = crate::R<HccommandstatusSpec>;
#[doc = "Register `HCCOMMANDSTATUS` writer"]
pub type W = crate::W<HccommandstatusSpec>;
#[doc = "Field `HCR` reader - Host controller reset (read/write)"]
pub type HcrR = crate::BitReader;
#[doc = "Field `HCR` writer - Host controller reset (read/write)"]
pub type HcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLF` reader - Control list filled (read/write)"]
pub type ClfR = crate::BitReader;
#[doc = "Field `CLF` writer - Control list filled (read/write)"]
pub type ClfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLF` reader - Bulk list filled (read/write)"]
pub type BlfR = crate::BitReader;
#[doc = "Field `BLF` writer - Bulk list filled (read/write)"]
pub type BlfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCR` reader - Ownership change request (read/write)"]
pub type OcrR = crate::BitReader;
#[doc = "Field `OCR` writer - Ownership change request (read/write)"]
pub type OcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC` reader - Scheduling overrun count (read-only)"]
pub type SocR = crate::FieldReader;
#[doc = "Field `SOC` writer - Scheduling overrun count (read-only)"]
pub type SocW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Host controller reset (read/write)"]
    #[inline(always)]
    pub fn hcr(&self) -> HcrR {
        HcrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control list filled (read/write)"]
    #[inline(always)]
    pub fn clf(&self) -> ClfR {
        ClfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bulk list filled (read/write)"]
    #[inline(always)]
    pub fn blf(&self) -> BlfR {
        BlfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ownership change request (read/write)"]
    #[inline(always)]
    pub fn ocr(&self) -> OcrR {
        OcrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Scheduling overrun count (read-only)"]
    #[inline(always)]
    pub fn soc(&self) -> SocR {
        SocR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Host controller reset (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn hcr(&mut self) -> HcrW<HccommandstatusSpec> {
        HcrW::new(self, 0)
    }
    #[doc = "Bit 1 - Control list filled (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn clf(&mut self) -> ClfW<HccommandstatusSpec> {
        ClfW::new(self, 1)
    }
    #[doc = "Bit 2 - Bulk list filled (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn blf(&mut self) -> BlfW<HccommandstatusSpec> {
        BlfW::new(self, 2)
    }
    #[doc = "Bit 3 - Ownership change request (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn ocr(&mut self) -> OcrW<HccommandstatusSpec> {
        OcrW::new(self, 3)
    }
    #[doc = "Bits 16:17 - Scheduling overrun count (read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn soc(&mut self) -> SocW<HccommandstatusSpec> {
        SocW::new(self, 16)
    }
}
#[doc = "HC Command and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hccommandstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccommandstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccommandstatusSpec;
impl crate::RegisterSpec for HccommandstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hccommandstatus::R`](R) reader structure"]
impl crate::Readable for HccommandstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hccommandstatus::W`](W) writer structure"]
impl crate::Writable for HccommandstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCCOMMANDSTATUS to value 0"]
impl crate::Resettable for HccommandstatusSpec {
    const RESET_VALUE: u32 = 0;
}
