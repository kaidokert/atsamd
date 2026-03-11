#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `GCLKDIS` reader - Disable Clock Gating"]
pub type GclkdisR = crate::BitReader;
#[doc = "Field `GCLKDIS` writer - Disable Clock Gating"]
pub type GclkdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICDIS` reader - "]
pub type IcdisR = crate::BitReader;
#[doc = "Field `ICDIS` writer - "]
pub type IcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDIS` reader - "]
pub type DcdisR = crate::BitReader;
#[doc = "Field `DCDIS` writer - "]
pub type DcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRGCSIZE` reader - "]
pub type PrgcsizeR = crate::FieldReader;
#[doc = "Field `PRGCSIZE` writer - "]
pub type PrgcsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Disable Clock Gating"]
    #[inline(always)]
    pub fn gclkdis(&self) -> GclkdisR {
        GclkdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icdis(&self) -> IcdisR {
        IcdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dcdis(&self) -> DcdisR {
        DcdisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn prgcsize(&self) -> PrgcsizeR {
        PrgcsizeR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Clock Gating"]
    #[inline(always)]
    #[must_use]
    pub fn gclkdis(&mut self) -> GclkdisW<CfgSpec> {
        GclkdisW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn icdis(&mut self) -> IcdisW<CfgSpec> {
        IcdisW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dcdis(&mut self) -> DcdisW<CfgSpec> {
        DcdisW::new(self, 2)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn prgcsize(&mut self) -> PrgcsizeW<CfgSpec> {
        PrgcsizeW::new(self, 4)
    }
}
#[doc = "Cache Controller Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0x20"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x20;
}
