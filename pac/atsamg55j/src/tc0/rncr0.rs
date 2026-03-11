#[doc = "Register `RNCR0` reader"]
pub type R = crate::R<Rncr0Spec>;
#[doc = "Register `RNCR0` writer"]
pub type W = crate::W<Rncr0Spec>;
#[doc = "Field `RXNCTR` reader - Receive Next Counter"]
pub type RxnctrR = crate::FieldReader<u16>;
#[doc = "Field `RXNCTR` writer - Receive Next Counter"]
pub type RxnctrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Next Counter"]
    #[inline(always)]
    pub fn rxnctr(&self) -> RxnctrR {
        RxnctrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Next Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rxnctr(&mut self) -> RxnctrW<Rncr0Spec> {
        RxnctrW::new(self, 0)
    }
}
#[doc = "Receive Next Counter Register (pdc = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rncr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rncr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rncr0Spec;
impl crate::RegisterSpec for Rncr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rncr0::R`](R) reader structure"]
impl crate::Readable for Rncr0Spec {}
#[doc = "`write(|w| ..)` method takes [`rncr0::W`](W) writer structure"]
impl crate::Writable for Rncr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNCR0 to value 0"]
impl crate::Resettable for Rncr0Spec {
    const RESET_VALUE: u32 = 0;
}
