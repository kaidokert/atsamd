#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOR` writer - Receiver Overrun Interrupt Enable"]
pub type RxorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Reception Interrupt Enable"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUR` writer - Transmit Underflow Interrupt Enable"]
pub type TxurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmission Interrupt Enable"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Receive Buffer Full Interrupt Enable"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Receiver Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RxrdyW<IerSpec> {
        RxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxor(&mut self) -> RxorW<IerSpec> {
        RxorW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Reception Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IerSpec> {
        EndrxW::new(self, 3)
    }
    #[doc = "Bit 5 - Transmit Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<IerSpec> {
        TxrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txur(&mut self) -> TxurW<IerSpec> {
        TxurW::new(self, 6)
    }
    #[doc = "Bit 7 - End of Transmission Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<IerSpec> {
        EndtxW::new(self, 7)
    }
    #[doc = "Bit 19 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RxfullW<IerSpec> {
        RxfullW::new(self, 19)
    }
    #[doc = "Bit 31 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<IerSpec> {
        TxemptyW::new(self, 31)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
