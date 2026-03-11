#[doc = "Register `RNPR0` reader"]
pub type R = crate::R<Rnpr0Spec>;
#[doc = "Register `RNPR0` writer"]
pub type W = crate::W<Rnpr0Spec>;
#[doc = "Field `RXNPTR` reader - Receive Next Pointer"]
pub type RxnptrR = crate::FieldReader<u32>;
#[doc = "Field `RXNPTR` writer - Receive Next Pointer"]
pub type RxnptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    pub fn rxnptr(&self) -> RxnptrR {
        RxnptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rxnptr(&mut self) -> RxnptrW<Rnpr0Spec> {
        RxnptrW::new(self, 0)
    }
}
#[doc = "Receive Next Pointer Register (pdc = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rnpr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnpr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rnpr0Spec;
impl crate::RegisterSpec for Rnpr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnpr0::R`](R) reader structure"]
impl crate::Readable for Rnpr0Spec {}
#[doc = "`write(|w| ..)` method takes [`rnpr0::W`](W) writer structure"]
impl crate::Writable for Rnpr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNPR0 to value 0"]
impl crate::Resettable for Rnpr0Spec {
    const RESET_VALUE: u32 = 0;
}
