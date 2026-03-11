#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `RXEN` reader - Receiver Enabled"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receive Ready"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `RXOR` reader - Receive Overrun"]
pub type RxorR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of Receiver Transfer"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `TXEN` reader - Transmitter Enabled"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmit Ready"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `TXUR` reader - Transmit Underrun"]
pub type TxurR = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of Transmitter Transfer"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `RXORCH` reader - Receive Overrun Channel"]
pub type RxorchR = crate::FieldReader;
#[doc = "Field `RXBUFF` reader - Receive Buffer Full"]
pub type RxbuffR = crate::BitReader;
#[doc = "Field `TXURCH` reader - Transmit Underrun Channel"]
pub type TxurchR = crate::FieldReader;
#[doc = "Field `TXBUFE` reader - Transmit Buffer Empty"]
pub type TxbufeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Enabled"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    pub fn rxor(&self) -> RxorR {
        RxorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Receiver Transfer"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter Enabled"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn txur(&self) -> TxurR {
        TxurR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of Transmitter Transfer"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Receive Overrun Channel"]
    #[inline(always)]
    pub fn rxorch(&self) -> RxorchR {
        RxorchR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 19 - Receive Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Transmit Underrun Channel"]
    #[inline(always)]
    pub fn txurch(&self) -> TxurchR {
        TxurchR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 31 - Transmit Buffer Empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
