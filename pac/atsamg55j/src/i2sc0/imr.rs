#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `RXRDY` reader - Receiver Ready Interrupt Disable"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `RXOR` reader - Receiver Overrun Interrupt Disable"]
pub type RxorR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of Reception Interrupt Disable"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmit Ready Interrupt Disable"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `TXUR` reader - Transmit Underflow Interrupt Disable"]
pub type TxurR = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of Transmission Interrupt Disable"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `RXFULL` reader - Receive Buffer Full Interrupt Disable"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmit Buffer Empty Interrupt Disable"]
pub type TxemptyR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Receiver Ready Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver Overrun Interrupt Disable"]
    #[inline(always)]
    pub fn rxor(&self) -> RxorR {
        RxorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Reception Interrupt Disable"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Ready Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Underflow Interrupt Disable"]
    #[inline(always)]
    pub fn txur(&self) -> TxurR {
        TxurR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of Transmission Interrupt Disable"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive Buffer Full Interrupt Disable"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmit Buffer Empty Interrupt Disable"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
