#[doc = "Register `RCR0` reader"]
pub type R = crate::R<Rcr0Spec>;
#[doc = "Register `RCR0` writer"]
pub type W = crate::W<Rcr0Spec>;
#[doc = "Field `RXCTR` reader - Receive Counter Register"]
pub type RxctrR = crate::FieldReader<u16>;
#[doc = "Field `RXCTR` writer - Receive Counter Register"]
pub type RxctrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Counter Register"]
    #[inline(always)]
    pub fn rxctr(&self) -> RxctrR {
        RxctrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Counter Register"]
    #[inline(always)]
    #[must_use]
    pub fn rxctr(&mut self) -> RxctrW<Rcr0Spec> {
        RxctrW::new(self, 0)
    }
}
#[doc = "Receive Counter Register (pdc = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rcr0Spec;
impl crate::RegisterSpec for Rcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr0::R`](R) reader structure"]
impl crate::Readable for Rcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`rcr0::W`](W) writer structure"]
impl crate::Writable for Rcr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCR0 to value 0"]
impl crate::Resettable for Rcr0Spec {
    const RESET_VALUE: u32 = 0;
}
