#[doc = "Register `PTSR` reader"]
pub type R = crate::R<PtsrSpec>;
#[doc = "Field `RXTEN` reader - Receiver Transfer Enable"]
pub type RxtenR = crate::BitReader;
#[doc = "Field `TXTEN` reader - Transmitter Transfer Enable"]
pub type TxtenR = crate::BitReader;
#[doc = "Field `RXCBEN` reader - Receiver Circular Buffer Enable"]
pub type RxcbenR = crate::BitReader;
#[doc = "Field `TXCBEN` reader - Transmitter Circular Buffer Enable"]
pub type TxcbenR = crate::BitReader;
#[doc = "Field `ERR` reader - Transfer Bus Error"]
pub type ErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Transfer Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RxtenR {
        RxtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitter Transfer Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TxtenR {
        TxtenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Receiver Circular Buffer Enable"]
    #[inline(always)]
    pub fn rxcben(&self) -> RxcbenR {
        RxcbenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmitter Circular Buffer Enable"]
    #[inline(always)]
    pub fn txcben(&self) -> TxcbenR {
        TxcbenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Transfer Bus Error"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Transfer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtsrSpec;
impl crate::RegisterSpec for PtsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptsr::R`](R) reader structure"]
impl crate::Readable for PtsrSpec {}
#[doc = "`reset()` method sets PTSR to value 0"]
impl crate::Resettable for PtsrSpec {
    const RESET_VALUE: u32 = 0;
}
