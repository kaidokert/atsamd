#[doc = "Register `HCFMINTERVAL` reader"]
pub type R = crate::R<HcfmintervalSpec>;
#[doc = "Register `HCFMINTERVAL` writer"]
pub type W = crate::W<HcfmintervalSpec>;
#[doc = "Field `FRAMEINTERVAL` reader - Frame interval"]
pub type FrameintervalR = crate::FieldReader<u16>;
#[doc = "Field `FRAMEINTERVAL` writer - Frame interval"]
pub type FrameintervalW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `FSMPS` reader - Largest data packet"]
pub type FsmpsR = crate::FieldReader<u16>;
#[doc = "Field `FSMPS` writer - Largest data packet"]
pub type FsmpsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `FIT` reader - Frame interval toggle"]
pub type FitR = crate::BitReader;
#[doc = "Field `FIT` writer - Frame interval toggle"]
pub type FitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Frame interval"]
    #[inline(always)]
    pub fn frameinterval(&self) -> FrameintervalR {
        FrameintervalR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:30 - Largest data packet"]
    #[inline(always)]
    pub fn fsmps(&self) -> FsmpsR {
        FsmpsR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Frame interval toggle"]
    #[inline(always)]
    pub fn fit(&self) -> FitR {
        FitR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame interval"]
    #[inline(always)]
    #[must_use]
    pub fn frameinterval(&mut self) -> FrameintervalW<HcfmintervalSpec> {
        FrameintervalW::new(self, 0)
    }
    #[doc = "Bits 16:30 - Largest data packet"]
    #[inline(always)]
    #[must_use]
    pub fn fsmps(&mut self) -> FsmpsW<HcfmintervalSpec> {
        FsmpsW::new(self, 16)
    }
    #[doc = "Bit 31 - Frame interval toggle"]
    #[inline(always)]
    #[must_use]
    pub fn fit(&mut self) -> FitW<HcfmintervalSpec> {
        FitW::new(self, 31)
    }
}
#[doc = "HC Frame Interval Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfminterval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfminterval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcfmintervalSpec;
impl crate::RegisterSpec for HcfmintervalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfminterval::R`](R) reader structure"]
impl crate::Readable for HcfmintervalSpec {}
#[doc = "`write(|w| ..)` method takes [`hcfminterval::W`](W) writer structure"]
impl crate::Writable for HcfmintervalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCFMINTERVAL to value 0x2edf"]
impl crate::Resettable for HcfmintervalSpec {
    const RESET_VALUE: u32 = 0x2edf;
}
