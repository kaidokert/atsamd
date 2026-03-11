#[doc = "Register `HCCONTROLCURRENTED` reader"]
pub type R = crate::R<HccontrolcurrentedSpec>;
#[doc = "Register `HCCONTROLCURRENTED` writer"]
pub type W = crate::W<HccontrolcurrentedSpec>;
#[doc = "Field `CCED` reader - Physical address of the current ED on the control ED list"]
pub type CcedR = crate::FieldReader<u32>;
#[doc = "Field `CCED` writer - Physical address of the current ED on the control ED list"]
pub type CcedW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - Physical address of the current ED on the control ED list"]
    #[inline(always)]
    pub fn cced(&self) -> CcedR {
        CcedR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - Physical address of the current ED on the control ED list"]
    #[inline(always)]
    #[must_use]
    pub fn cced(&mut self) -> CcedW<HccontrolcurrentedSpec> {
        CcedW::new(self, 4)
    }
}
#[doc = "HC Current Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hccontrolcurrented::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccontrolcurrented::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccontrolcurrentedSpec;
impl crate::RegisterSpec for HccontrolcurrentedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hccontrolcurrented::R`](R) reader structure"]
impl crate::Readable for HccontrolcurrentedSpec {}
#[doc = "`write(|w| ..)` method takes [`hccontrolcurrented::W`](W) writer structure"]
impl crate::Writable for HccontrolcurrentedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCCONTROLCURRENTED to value 0"]
impl crate::Resettable for HccontrolcurrentedSpec {
    const RESET_VALUE: u32 = 0;
}
