#[doc = "Register `CKGR_PLLBR` reader"]
pub type R = crate::R<CkgrPllbrSpec>;
#[doc = "Register `CKGR_PLLBR` writer"]
pub type W = crate::W<CkgrPllbrSpec>;
#[doc = "Field `PLLBEN` reader - PLLB Control"]
pub type PllbenR = crate::FieldReader;
#[doc = "Field `PLLBEN` writer - PLLB Control"]
pub type PllbenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PLLBCOUNT` reader - PLLB Counter"]
pub type PllbcountR = crate::FieldReader;
#[doc = "Field `PLLBCOUNT` writer - PLLB Counter"]
pub type PllbcountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MULB` reader - PLLB Multiplier"]
pub type MulbR = crate::FieldReader<u16>;
#[doc = "Field `MULB` writer - PLLB Multiplier"]
pub type MulbW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ZERO` reader - Must Be Written to 0"]
pub type ZeroR = crate::BitReader;
#[doc = "Field `ZERO` writer - Must Be Written to 0"]
pub type ZeroW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PLLB Control"]
    #[inline(always)]
    pub fn pllben(&self) -> PllbenR {
        PllbenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    pub fn pllbcount(&self) -> PllbcountR {
        PllbcountR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    pub fn mulb(&self) -> MulbR {
        MulbR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Must Be Written to 0"]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLLB Control"]
    #[inline(always)]
    #[must_use]
    pub fn pllben(&mut self) -> PllbenW<CkgrPllbrSpec> {
        PllbenW::new(self, 0)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pllbcount(&mut self) -> PllbcountW<CkgrPllbrSpec> {
        PllbcountW::new(self, 8)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn mulb(&mut self) -> MulbW<CkgrPllbrSpec> {
        MulbW::new(self, 16)
    }
    #[doc = "Bit 29 - Must Be Written to 0"]
    #[inline(always)]
    #[must_use]
    pub fn zero(&mut self) -> ZeroW<CkgrPllbrSpec> {
        ZeroW::new(self, 29)
    }
}
#[doc = "PLLB Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_pllbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_pllbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkgrPllbrSpec;
impl crate::RegisterSpec for CkgrPllbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_pllbr::R`](R) reader structure"]
impl crate::Readable for CkgrPllbrSpec {}
#[doc = "`write(|w| ..)` method takes [`ckgr_pllbr::W`](W) writer structure"]
impl crate::Writable for CkgrPllbrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKGR_PLLBR to value 0x3f00"]
impl crate::Resettable for CkgrPllbrSpec {
    const RESET_VALUE: u32 = 0x3f00;
}
