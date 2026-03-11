#[doc = "Register `PMC_IDR` writer"]
pub type W = crate::W<PmcIdrSpec>;
#[doc = "Field `MOSCXTS` writer - Main Crystal Oscillator Status Interrupt Disable"]
pub type MoscxtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKA` writer - PLLA Lock Interrupt Disable"]
pub type LockaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKB` writer - PLLB Lock Interrupt Disable"]
pub type LockbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKRDY` writer - Master Clock Ready Interrupt Disable"]
pub type MckrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY0` writer - Programmable Clock Ready 0 Interrupt Disable"]
pub type Pckrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY1` writer - Programmable Clock Ready 1 Interrupt Disable"]
pub type Pckrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY2` writer - Programmable Clock Ready 2 Interrupt Disable"]
pub type Pckrdy2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY3` writer - Programmable Clock Ready 3 Interrupt Disable"]
pub type Pckrdy3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY4` writer - Programmable Clock Ready 4 Interrupt Disable"]
pub type Pckrdy4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY5` writer - Programmable Clock Ready 5 Interrupt Disable"]
pub type Pckrdy5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY6` writer - Programmable Clock Ready 6 Interrupt Disable"]
pub type Pckrdy6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY7` writer - Programmable Clock Ready 7 Interrupt Disable"]
pub type Pckrdy7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCSELS` writer - Main Oscillator Selection Status Interrupt Disable"]
pub type MoscselsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCRCS` writer - Main On-Chip RC Status Interrupt Disable"]
pub type MoscrcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEV` writer - Clock Failure Detector Event Interrupt Disable"]
pub type CfdevW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn moscxts(&mut self) -> MoscxtsW<PmcIdrSpec> {
        MoscxtsW::new(self, 0)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn locka(&mut self) -> LockaW<PmcIdrSpec> {
        LockaW::new(self, 1)
    }
    #[doc = "Bit 2 - PLLB Lock Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lockb(&mut self) -> LockbW<PmcIdrSpec> {
        LockbW::new(self, 2)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mckrdy(&mut self) -> MckrdyW<PmcIdrSpec> {
        MckrdyW::new(self, 3)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy0(&mut self) -> Pckrdy0W<PmcIdrSpec> {
        Pckrdy0W::new(self, 8)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy1(&mut self) -> Pckrdy1W<PmcIdrSpec> {
        Pckrdy1W::new(self, 9)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy2(&mut self) -> Pckrdy2W<PmcIdrSpec> {
        Pckrdy2W::new(self, 10)
    }
    #[doc = "Bit 11 - Programmable Clock Ready 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy3(&mut self) -> Pckrdy3W<PmcIdrSpec> {
        Pckrdy3W::new(self, 11)
    }
    #[doc = "Bit 12 - Programmable Clock Ready 4 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy4(&mut self) -> Pckrdy4W<PmcIdrSpec> {
        Pckrdy4W::new(self, 12)
    }
    #[doc = "Bit 13 - Programmable Clock Ready 5 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy5(&mut self) -> Pckrdy5W<PmcIdrSpec> {
        Pckrdy5W::new(self, 13)
    }
    #[doc = "Bit 14 - Programmable Clock Ready 6 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy6(&mut self) -> Pckrdy6W<PmcIdrSpec> {
        Pckrdy6W::new(self, 14)
    }
    #[doc = "Bit 15 - Programmable Clock Ready 7 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy7(&mut self) -> Pckrdy7W<PmcIdrSpec> {
        Pckrdy7W::new(self, 15)
    }
    #[doc = "Bit 16 - Main Oscillator Selection Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn moscsels(&mut self) -> MoscselsW<PmcIdrSpec> {
        MoscselsW::new(self, 16)
    }
    #[doc = "Bit 17 - Main On-Chip RC Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcs(&mut self) -> MoscrcsW<PmcIdrSpec> {
        MoscrcsW::new(self, 17)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cfdev(&mut self) -> CfdevW<PmcIdrSpec> {
        CfdevW::new(self, 18)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcIdrSpec;
impl crate::RegisterSpec for PmcIdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_idr::W`](W) writer structure"]
impl crate::Writable for PmcIdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
