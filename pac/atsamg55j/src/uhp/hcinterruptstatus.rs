#[doc = "Register `HCINTERRUPTSTATUS` reader"]
pub type R = crate::R<HcinterruptstatusSpec>;
#[doc = "Register `HCINTERRUPTSTATUS` writer"]
pub type W = crate::W<HcinterruptstatusSpec>;
#[doc = "Field `SO` reader - Scheduling overrun (read/write, write '1' to clear)"]
pub type SoR = crate::BitReader;
#[doc = "Field `SO` writer - Scheduling overrun (read/write, write '1' to clear)"]
pub type SoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDH` reader - Write done head (read/write, write '1' to clear)"]
pub type WdhR = crate::BitReader;
#[doc = "Field `WDH` writer - Write done head (read/write, write '1' to clear)"]
pub type WdhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SF` reader - Start of frame (read/write, write '1' to clear)"]
pub type SfR = crate::BitReader;
#[doc = "Field `SF` writer - Start of frame (read/write, write '1' to clear)"]
pub type SfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - Resume detected (read/write, write '1' to clear)"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - Resume detected (read/write, write '1' to clear)"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UE` reader - Unrecoverable error (read/write, write '1' to clear)"]
pub type UeR = crate::BitReader;
#[doc = "Field `UE` writer - Unrecoverable error (read/write, write '1' to clear)"]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FNO` reader - Frame number overflow (read/write, write '1' to clear)"]
pub type FnoR = crate::BitReader;
#[doc = "Field `FNO` writer - Frame number overflow (read/write, write '1' to clear)"]
pub type FnoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RHSC` reader - Root hub status change (read/write, write '1' to clear)"]
pub type RhscR = crate::BitReader;
#[doc = "Field `RHSC` writer - Root hub status change (read/write, write '1' to clear)"]
pub type RhscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC` reader - Ownership change (read-only)"]
pub type OcR = crate::BitReader;
#[doc = "Field `OC` writer - Ownership change (read-only)"]
pub type OcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Scheduling overrun (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn so(&self) -> SoR {
        SoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write done head (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn wdh(&self) -> WdhR {
        WdhR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start of frame (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn sf(&self) -> SfR {
        SfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Resume detected (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Unrecoverable error (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame number overflow (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn fno(&self) -> FnoR {
        FnoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Root hub status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn rhsc(&self) -> RhscR {
        RhscR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 30 - Ownership change (read-only)"]
    #[inline(always)]
    pub fn oc(&self) -> OcR {
        OcR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scheduling overrun (read/write, write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn so(&mut self) -> SoW<HcinterruptstatusSpec> {
        SoW::new(self, 0)
    }
    #[doc = "Bit 1 - Write done head (read/write, write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn wdh(&mut self) -> WdhW<HcinterruptstatusSpec> {
        WdhW::new(self, 1)
    }
    #[doc = "Bit 2 - Start of frame (read/write, write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sf(&mut self) -> SfW<HcinterruptstatusSpec> {
        SfW::new(self, 2)
    }
    #[doc = "Bit 3 - Resume detected (read/write, write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RdW<HcinterruptstatusSpec> {
        RdW::new(self, 3)
    }
    #[doc = "Bit 4 - Unrecoverable error (read/write, write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UeW<HcinterruptstatusSpec> {
        UeW::new(self, 4)
    }
    #[doc = "Bit 5 - Frame number overflow (read/write, write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn fno(&mut self) -> FnoW<HcinterruptstatusSpec> {
        FnoW::new(self, 5)
    }
    #[doc = "Bit 6 - Root hub status change (read/write, write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn rhsc(&mut self) -> RhscW<HcinterruptstatusSpec> {
        RhscW::new(self, 6)
    }
    #[doc = "Bit 30 - Ownership change (read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn oc(&mut self) -> OcW<HcinterruptstatusSpec> {
        OcW::new(self, 30)
    }
}
#[doc = "HC Interrupt and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcinterruptstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcinterruptstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcinterruptstatusSpec;
impl crate::RegisterSpec for HcinterruptstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcinterruptstatus::R`](R) reader structure"]
impl crate::Readable for HcinterruptstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hcinterruptstatus::W`](W) writer structure"]
impl crate::Writable for HcinterruptstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINTERRUPTSTATUS to value 0"]
impl crate::Resettable for HcinterruptstatusSpec {
    const RESET_VALUE: u32 = 0;
}
