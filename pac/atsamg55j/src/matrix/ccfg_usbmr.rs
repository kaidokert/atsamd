#[doc = "Register `CCFG_USBMR` reader"]
pub type R = crate::R<CcfgUsbmrSpec>;
#[doc = "Register `CCFG_USBMR` writer"]
pub type W = crate::W<CcfgUsbmrSpec>;
#[doc = "Field `USBMODE` reader - USB Mode Selection"]
pub type UsbmodeR = crate::BitReader;
#[doc = "Field `USBMODE` writer - USB Mode Selection"]
pub type UsbmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBHTSSC` reader - USB Transceiver Suspend Software Control"]
pub type UsbhtsscR = crate::BitReader;
#[doc = "Field `USBHTSSC` writer - USB Transceiver Suspend Software Control"]
pub type UsbhtsscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBHTSC` reader - USB Host Transceiver Suspend Control"]
pub type UsbhtscR = crate::BitReader;
#[doc = "Field `USBHTSC` writer - USB Host Transceiver Suspend Control"]
pub type UsbhtscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB Mode Selection"]
    #[inline(always)]
    pub fn usbmode(&self) -> UsbmodeR {
        UsbmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Transceiver Suspend Software Control"]
    #[inline(always)]
    pub fn usbhtssc(&self) -> UsbhtsscR {
        UsbhtsscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB Host Transceiver Suspend Control"]
    #[inline(always)]
    pub fn usbhtsc(&self) -> UsbhtscR {
        UsbhtscR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbmode(&mut self) -> UsbmodeW<CcfgUsbmrSpec> {
        UsbmodeW::new(self, 0)
    }
    #[doc = "Bit 1 - USB Transceiver Suspend Software Control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhtssc(&mut self) -> UsbhtsscW<CcfgUsbmrSpec> {
        UsbhtsscW::new(self, 1)
    }
    #[doc = "Bit 2 - USB Host Transceiver Suspend Control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhtsc(&mut self) -> UsbhtscW<CcfgUsbmrSpec> {
        UsbhtscW::new(self, 2)
    }
}
#[doc = "USB Management Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_usbmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_usbmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgUsbmrSpec;
impl crate::RegisterSpec for CcfgUsbmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_usbmr::R`](R) reader structure"]
impl crate::Readable for CcfgUsbmrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_usbmr::W`](W) writer structure"]
impl crate::Writable for CcfgUsbmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFG_USBMR to value 0"]
impl crate::Resettable for CcfgUsbmrSpec {
    const RESET_VALUE: u32 = 0;
}
