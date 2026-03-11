#[doc = "Register `SSR` writer"]
pub type W = crate::W<SsrSpec>;
#[doc = "Field `RXOR` writer - Receive Overrun Status Set"]
pub type RxorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUR` writer - Transmit Underrun Status Set"]
pub type TxurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXORCH` writer - Receive Overrun Per Channel Status Set"]
pub type RxorchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXURCH` writer - Transmit Underrun Per Channel Status Set"]
pub type TxurchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bit 2 - Receive Overrun Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxor(&mut self) -> RxorW<SsrSpec> {
        RxorW::new(self, 2)
    }
    #[doc = "Bit 6 - Transmit Underrun Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn txur(&mut self) -> TxurW<SsrSpec> {
        TxurW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Receive Overrun Per Channel Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxorch(&mut self) -> RxorchW<SsrSpec> {
        RxorchW::new(self, 8)
    }
    #[doc = "Bits 20:21 - Transmit Underrun Per Channel Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn txurch(&mut self) -> TxurchW<SsrSpec> {
        TxurchW::new(self, 20)
    }
}
#[doc = "Status Set Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsrSpec;
impl crate::RegisterSpec for SsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ssr::W`](W) writer structure"]
impl crate::Writable for SsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
