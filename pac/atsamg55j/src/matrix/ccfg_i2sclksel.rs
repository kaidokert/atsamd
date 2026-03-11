#[doc = "Register `CCFG_I2SCLKSEL` reader"]
pub type R = crate::R<CcfgI2sclkselSpec>;
#[doc = "Register `CCFG_I2SCLKSEL` writer"]
pub type W = crate::W<CcfgI2sclkselSpec>;
#[doc = "Field `CLKSEL0` reader - I2S0 Clock Source"]
pub type Clksel0R = crate::BitReader;
#[doc = "Field `CLKSEL0` writer - I2S0 Clock Source"]
pub type Clksel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL1` reader - I2S1 Clock Source"]
pub type Clksel1R = crate::BitReader;
#[doc = "Field `CLKSEL1` writer - I2S1 Clock Source"]
pub type Clksel1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2S0 Clock Source"]
    #[inline(always)]
    pub fn clksel0(&self) -> Clksel0R {
        Clksel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2S1 Clock Source"]
    #[inline(always)]
    pub fn clksel1(&self) -> Clksel1R {
        Clksel1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S0 Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn clksel0(&mut self) -> Clksel0W<CcfgI2sclkselSpec> {
        Clksel0W::new(self, 0)
    }
    #[doc = "Bit 1 - I2S1 Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn clksel1(&mut self) -> Clksel1W<CcfgI2sclkselSpec> {
        Clksel1W::new(self, 1)
    }
}
#[doc = "I2S Clock Source Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_i2sclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_i2sclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgI2sclkselSpec;
impl crate::RegisterSpec for CcfgI2sclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_i2sclksel::R`](R) reader structure"]
impl crate::Readable for CcfgI2sclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_i2sclksel::W`](W) writer structure"]
impl crate::Writable for CcfgI2sclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFG_I2SCLKSEL to value 0"]
impl crate::Resettable for CcfgI2sclkselSpec {
    const RESET_VALUE: u32 = 0;
}
