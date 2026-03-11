#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading US_RHR)"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmitter Ready (cleared by writing US_THR)"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `RXBRK` reader - Break Received/End of Break (cleared by writing a one to the bit US_CR.RSTSTA)"]
pub type RxbrkR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of RX Buffer (cleared by writing US_RCR or US_RNCR)"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of TX Buffer (cleared by writing US_TCR or US_TNCR)"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error (cleared by writing a one to the bit US_CR.RSTSTA)"]
pub type OvreR = crate::BitReader;
#[doc = "Field `FRAME` reader - Framing Error (cleared by writing a one to the bit US_CR.RSTSTA)"]
pub type FrameR = crate::BitReader;
#[doc = "Field `PARE` reader - Parity Error (cleared by writing a one to the US_CR.RSTSTA)"]
pub type PareR = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Receiver Time-out (cleared by writing a one to the bit US_CR.STTTO)"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty (cleared by writing US_THR)"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `ITER` reader - Max Number of Repetitions Reached (cleared by writing a one to the bit US_CR.RSTIT)"]
pub type IterR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - TX Buffer Empty (cleared by writing US_TCR or US_TNCR)"]
pub type TxbufeR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - RX Buffer Full (cleared by writing US_RCR or US_RNCR)"]
pub type RxbuffR = crate::BitReader;
#[doc = "Field `NACK` reader - Non Acknowledge Interrupt (cleared by writing a one to the bit US_CR.RSTNACK)"]
pub type NackR = crate::BitReader;
#[doc = "Field `CTSIC` reader - Clear to Send Input Change Flag (cleared on read)"]
pub type CtsicR = crate::BitReader;
#[doc = "Field `CMP` reader - Comparison Status (cleared by writing a one to the bit US_CR.RSTSTA command)"]
pub type CmpR = crate::BitReader;
#[doc = "Field `CTS` reader - Image of CTS Input"]
pub type CtsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Ready (cleared by reading US_RHR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break Received/End of Break (cleared by writing a one to the bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RxbrkR {
        RxbrkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of RX Buffer (cleared by writing US_RCR or US_RNCR)"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of TX Buffer (cleared by writing US_TCR or US_TNCR)"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error (cleared by writing a one to the bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error (cleared by writing a one to the bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn frame(&self) -> FrameR {
        FrameR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error (cleared by writing a one to the US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn pare(&self) -> PareR {
        PareR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receiver Time-out (cleared by writing a one to the bit US_CR.STTTO)"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Max Number of Repetitions Reached (cleared by writing a one to the bit US_CR.RSTIT)"]
    #[inline(always)]
    pub fn iter(&self) -> IterR {
        IterR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TX Buffer Empty (cleared by writing US_TCR or US_TNCR)"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX Buffer Full (cleared by writing US_RCR or US_RNCR)"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt (cleared by writing a one to the bit US_CR.RSTNACK)"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn ctsic(&self) -> CtsicR {
        CtsicR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Comparison Status (cleared by writing a one to the bit US_CR.RSTSTA command)"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Image of CTS Input"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "USART Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
