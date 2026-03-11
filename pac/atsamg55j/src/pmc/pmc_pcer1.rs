#[doc = "Register `PMC_PCER1` writer"]
pub type W = crate::W<PmcPcer1Spec>;
#[doc = "Field `PID47` writer - Peripheral Clock 47 Enable"]
pub type Pid47W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID48` writer - Peripheral Clock 48 Enable"]
pub type Pid48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID49` writer - Peripheral Clock 49 Enable"]
pub type Pid49W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 15 - Peripheral Clock 47 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid47(&mut self) -> Pid47W<PmcPcer1Spec> {
        Pid47W::new(self, 15)
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid48(&mut self) -> Pid48W<PmcPcer1Spec> {
        Pid48W::new(self, 16)
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid49(&mut self) -> Pid49W<PmcPcer1Spec> {
        Pid49W::new(self, 17)
    }
}
#[doc = "Peripheral Clock Enable Register 1\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_pcer1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcPcer1Spec;
impl crate::RegisterSpec for PmcPcer1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_pcer1::W`](W) writer structure"]
impl crate::Writable for PmcPcer1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
