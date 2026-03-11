#[doc = "Register `HCCONTROL` reader"]
pub type R = crate::R<HccontrolSpec>;
#[doc = "Register `HCCONTROL` writer"]
pub type W = crate::W<HccontrolSpec>;
#[doc = "Field `CBSR` reader - Control/bulk service ratio"]
pub type CbsrR = crate::FieldReader;
#[doc = "Field `CBSR` writer - Control/bulk service ratio"]
pub type CbsrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLE` reader - Periodic list enable"]
pub type PleR = crate::BitReader;
#[doc = "Field `PLE` writer - Periodic list enable"]
pub type PleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Isochronous enable"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - Isochronous enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLE` reader - Control list enable"]
pub type CleR = crate::BitReader;
#[doc = "Field `CLE` writer - Control list enable"]
pub type CleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLE` reader - Bulk list enable"]
pub type BleR = crate::BitReader;
#[doc = "Field `BLE` writer - Bulk list enable"]
pub type BleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCFS` reader - Host controller functional state"]
pub type HcfsR = crate::FieldReader;
#[doc = "Field `HCFS` writer - Host controller functional state"]
pub type HcfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IR` reader - Interrupt routing"]
pub type IrR = crate::BitReader;
#[doc = "Field `IR` writer - Interrupt routing"]
pub type IrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWC` reader - Remote wake-up connected"]
pub type RwcR = crate::BitReader;
#[doc = "Field `RWC` writer - Remote wake-up connected"]
pub type RwcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWE` reader - Remote wake-up enable"]
pub type RweR = crate::BitReader;
#[doc = "Field `RWE` writer - Remote wake-up enable"]
pub type RweW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Control/bulk service ratio"]
    #[inline(always)]
    pub fn cbsr(&self) -> CbsrR {
        CbsrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Periodic list enable"]
    #[inline(always)]
    pub fn ple(&self) -> PleR {
        PleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Isochronous enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control list enable"]
    #[inline(always)]
    pub fn cle(&self) -> CleR {
        CleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bulk list enable"]
    #[inline(always)]
    pub fn ble(&self) -> BleR {
        BleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Host controller functional state"]
    #[inline(always)]
    pub fn hcfs(&self) -> HcfsR {
        HcfsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Interrupt routing"]
    #[inline(always)]
    pub fn ir(&self) -> IrR {
        IrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remote wake-up connected"]
    #[inline(always)]
    pub fn rwc(&self) -> RwcR {
        RwcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Remote wake-up enable"]
    #[inline(always)]
    pub fn rwe(&self) -> RweR {
        RweR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control/bulk service ratio"]
    #[inline(always)]
    #[must_use]
    pub fn cbsr(&mut self) -> CbsrW<HccontrolSpec> {
        CbsrW::new(self, 0)
    }
    #[doc = "Bit 2 - Periodic list enable"]
    #[inline(always)]
    #[must_use]
    pub fn ple(&mut self) -> PleW<HccontrolSpec> {
        PleW::new(self, 2)
    }
    #[doc = "Bit 3 - Isochronous enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<HccontrolSpec> {
        IeW::new(self, 3)
    }
    #[doc = "Bit 4 - Control list enable"]
    #[inline(always)]
    #[must_use]
    pub fn cle(&mut self) -> CleW<HccontrolSpec> {
        CleW::new(self, 4)
    }
    #[doc = "Bit 5 - Bulk list enable"]
    #[inline(always)]
    #[must_use]
    pub fn ble(&mut self) -> BleW<HccontrolSpec> {
        BleW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Host controller functional state"]
    #[inline(always)]
    #[must_use]
    pub fn hcfs(&mut self) -> HcfsW<HccontrolSpec> {
        HcfsW::new(self, 6)
    }
    #[doc = "Bit 8 - Interrupt routing"]
    #[inline(always)]
    #[must_use]
    pub fn ir(&mut self) -> IrW<HccontrolSpec> {
        IrW::new(self, 8)
    }
    #[doc = "Bit 9 - Remote wake-up connected"]
    #[inline(always)]
    #[must_use]
    pub fn rwc(&mut self) -> RwcW<HccontrolSpec> {
        RwcW::new(self, 9)
    }
    #[doc = "Bit 10 - Remote wake-up enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwe(&mut self) -> RweW<HccontrolSpec> {
        RweW::new(self, 10)
    }
}
#[doc = "HC Operating Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hccontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccontrolSpec;
impl crate::RegisterSpec for HccontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hccontrol::R`](R) reader structure"]
impl crate::Readable for HccontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`hccontrol::W`](W) writer structure"]
impl crate::Writable for HccontrolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCCONTROL to value 0"]
impl crate::Resettable for HccontrolSpec {
    const RESET_VALUE: u32 = 0;
}
