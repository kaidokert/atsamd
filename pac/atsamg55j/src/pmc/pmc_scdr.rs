#[doc = "Register `PMC_SCDR` writer"]
pub type W = crate::W<PmcScdrSpec>;
#[doc = "Field `UHP` writer - USB Host Port Clock Disable"]
pub type UhpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDP` writer - "]
pub type UdpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK0` writer - Programmable Clock 0 Output Disable"]
pub type Pck0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK1` writer - Programmable Clock 1 Output Disable"]
pub type Pck1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK2` writer - Programmable Clock 2 Output Disable"]
pub type Pck2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK3` writer - Programmable Clock 3 Output Disable"]
pub type Pck3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK4` writer - Programmable Clock 4 Output Disable"]
pub type Pck4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK5` writer - Programmable Clock 5 Output Disable"]
pub type Pck5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK6` writer - Programmable Clock 6 Output Disable"]
pub type Pck6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK7` writer - Programmable Clock 7 Output Disable"]
pub type Pck7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 6 - USB Host Port Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn uhp(&mut self) -> UhpW<PmcScdrSpec> {
        UhpW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn udp(&mut self) -> UdpW<PmcScdrSpec> {
        UdpW::new(self, 7)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pck0(&mut self) -> Pck0W<PmcScdrSpec> {
        Pck0W::new(self, 8)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pck1(&mut self) -> Pck1W<PmcScdrSpec> {
        Pck1W::new(self, 9)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pck2(&mut self) -> Pck2W<PmcScdrSpec> {
        Pck2W::new(self, 10)
    }
    #[doc = "Bit 11 - Programmable Clock 3 Output Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pck3(&mut self) -> Pck3W<PmcScdrSpec> {
        Pck3W::new(self, 11)
    }
    #[doc = "Bit 12 - Programmable Clock 4 Output Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pck4(&mut self) -> Pck4W<PmcScdrSpec> {
        Pck4W::new(self, 12)
    }
    #[doc = "Bit 13 - Programmable Clock 5 Output Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pck5(&mut self) -> Pck5W<PmcScdrSpec> {
        Pck5W::new(self, 13)
    }
    #[doc = "Bit 14 - Programmable Clock 6 Output Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pck6(&mut self) -> Pck6W<PmcScdrSpec> {
        Pck6W::new(self, 14)
    }
    #[doc = "Bit 15 - Programmable Clock 7 Output Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pck7(&mut self) -> Pck7W<PmcScdrSpec> {
        Pck7W::new(self, 15)
    }
}
#[doc = "System Clock Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_scdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcScdrSpec;
impl crate::RegisterSpec for PmcScdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_scdr::W`](W) writer structure"]
impl crate::Writable for PmcScdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
