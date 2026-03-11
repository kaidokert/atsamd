#[doc = "Register `PTCR` writer"]
pub type W = crate::W<PtcrSpec>;
#[doc = "Field `RXTEN` writer - Receiver Transfer Enable"]
pub type RxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTDIS` writer - Receiver Transfer Disable"]
pub type RxtdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTEN` writer - Transmitter Transfer Enable"]
pub type TxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTDIS` writer - Transmitter Transfer Disable"]
pub type TxtdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCBEN` writer - Receiver Circular Buffer Enable"]
pub type RxcbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCBDIS` writer - Receiver Circular Buffer Disable"]
pub type RxcbdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCBEN` writer - Transmitter Circular Buffer Enable"]
pub type TxcbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCBDIS` writer - Transmitter Circular Buffer Disable"]
pub type TxcbdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRCLR` writer - Transfer Bus Error Clear"]
pub type ErrclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receiver Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxten(&mut self) -> RxtenW<PtcrSpec> {
        RxtenW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxtdis(&mut self) -> RxtdisW<PtcrSpec> {
        RxtdisW::new(self, 1)
    }
    #[doc = "Bit 8 - Transmitter Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txten(&mut self) -> TxtenW<PtcrSpec> {
        TxtenW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmitter Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txtdis(&mut self) -> TxtdisW<PtcrSpec> {
        TxtdisW::new(self, 9)
    }
    #[doc = "Bit 16 - Receiver Circular Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcben(&mut self) -> RxcbenW<PtcrSpec> {
        RxcbenW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Circular Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcbdis(&mut self) -> RxcbdisW<PtcrSpec> {
        RxcbdisW::new(self, 17)
    }
    #[doc = "Bit 18 - Transmitter Circular Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcben(&mut self) -> TxcbenW<PtcrSpec> {
        TxcbenW::new(self, 18)
    }
    #[doc = "Bit 19 - Transmitter Circular Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txcbdis(&mut self) -> TxcbdisW<PtcrSpec> {
        TxcbdisW::new(self, 19)
    }
    #[doc = "Bit 24 - Transfer Bus Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn errclr(&mut self) -> ErrclrW<PtcrSpec> {
        ErrclrW::new(self, 24)
    }
}
#[doc = "Transfer Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtcrSpec;
impl crate::RegisterSpec for PtcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ptcr::W`](W) writer structure"]
impl crate::Writable for PtcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
