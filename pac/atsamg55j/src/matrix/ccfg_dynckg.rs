#[doc = "Register `CCFG_DYNCKG` reader"]
pub type R = crate::R<CcfgDynckgSpec>;
#[doc = "Register `CCFG_DYNCKG` writer"]
pub type W = crate::W<CcfgDynckgSpec>;
#[doc = "Field `MATCKG` reader - MATRIX Dynamic Clock Gating"]
pub type MatckgR = crate::BitReader;
#[doc = "Field `MATCKG` writer - MATRIX Dynamic Clock Gating"]
pub type MatckgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRIDCKG` reader - Bridge Dynamic Clock Gating Enable"]
pub type BridckgR = crate::BitReader;
#[doc = "Field `BRIDCKG` writer - Bridge Dynamic Clock Gating Enable"]
pub type BridckgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFCCKG` reader - EFC Dynamic Clock Gating Enable"]
pub type EfcckgR = crate::BitReader;
#[doc = "Field `EFCCKG` writer - EFC Dynamic Clock Gating Enable"]
pub type EfcckgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MATRIX Dynamic Clock Gating"]
    #[inline(always)]
    pub fn matckg(&self) -> MatckgR {
        MatckgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bridge Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn bridckg(&self) -> BridckgR {
        BridckgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EFC Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn efcckg(&self) -> EfcckgR {
        EfcckgR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MATRIX Dynamic Clock Gating"]
    #[inline(always)]
    #[must_use]
    pub fn matckg(&mut self) -> MatckgW<CcfgDynckgSpec> {
        MatckgW::new(self, 0)
    }
    #[doc = "Bit 1 - Bridge Dynamic Clock Gating Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bridckg(&mut self) -> BridckgW<CcfgDynckgSpec> {
        BridckgW::new(self, 1)
    }
    #[doc = "Bit 2 - EFC Dynamic Clock Gating Enable"]
    #[inline(always)]
    #[must_use]
    pub fn efcckg(&mut self) -> EfcckgW<CcfgDynckgSpec> {
        EfcckgW::new(self, 2)
    }
}
#[doc = "Dynamic Clock Gating Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_dynckg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_dynckg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgDynckgSpec;
impl crate::RegisterSpec for CcfgDynckgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_dynckg::R`](R) reader structure"]
impl crate::Readable for CcfgDynckgSpec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_dynckg::W`](W) writer structure"]
impl crate::Writable for CcfgDynckgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFG_DYNCKG to value 0x07"]
impl crate::Resettable for CcfgDynckgSpec {
    const RESET_VALUE: u32 = 0x07;
}
