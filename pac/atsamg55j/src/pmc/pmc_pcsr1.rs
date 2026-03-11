#[doc = "Register `PMC_PCSR1` reader"]
pub type R = crate::R<PmcPcsr1Spec>;
#[doc = "Field `PID47` reader - Peripheral Clock 47 Status"]
pub type Pid47R = crate::BitReader;
#[doc = "Field `PID48` reader - Peripheral Clock 48 Status"]
pub type Pid48R = crate::BitReader;
#[doc = "Field `PID49` reader - Peripheral Clock 49 Status"]
pub type Pid49R = crate::BitReader;
impl R {
    #[doc = "Bit 15 - Peripheral Clock 47 Status"]
    #[inline(always)]
    pub fn pid47(&self) -> Pid47R {
        Pid47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Status"]
    #[inline(always)]
    pub fn pid48(&self) -> Pid48R {
        Pid48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Status"]
    #[inline(always)]
    pub fn pid49(&self) -> Pid49R {
        Pid49R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Peripheral Clock Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_pcsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcPcsr1Spec;
impl crate::RegisterSpec for PmcPcsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_pcsr1::R`](R) reader structure"]
impl crate::Readable for PmcPcsr1Spec {}
#[doc = "`reset()` method sets PMC_PCSR1 to value 0"]
impl crate::Resettable for PmcPcsr1Spec {
    const RESET_VALUE: u32 = 0;
}
