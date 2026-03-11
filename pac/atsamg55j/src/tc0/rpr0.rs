#[doc = "Register `RPR0` reader"]
pub type R = crate::R<Rpr0Spec>;
#[doc = "Register `RPR0` writer"]
pub type W = crate::W<Rpr0Spec>;
#[doc = "Field `RXPTR` reader - Receive Pointer Register"]
pub type RxptrR = crate::FieldReader<u32>;
#[doc = "Field `RXPTR` writer - Receive Pointer Register"]
pub type RxptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Pointer Register"]
    #[inline(always)]
    pub fn rxptr(&self) -> RxptrR {
        RxptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Pointer Register"]
    #[inline(always)]
    #[must_use]
    pub fn rxptr(&mut self) -> RxptrW<Rpr0Spec> {
        RxptrW::new(self, 0)
    }
}
#[doc = "Receive Pointer Register (pdc = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rpr0Spec;
impl crate::RegisterSpec for Rpr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr0::R`](R) reader structure"]
impl crate::Readable for Rpr0Spec {}
#[doc = "`write(|w| ..)` method takes [`rpr0::W`](W) writer structure"]
impl crate::Writable for Rpr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RPR0 to value 0"]
impl crate::Resettable for Rpr0Spec {
    const RESET_VALUE: u32 = 0;
}
