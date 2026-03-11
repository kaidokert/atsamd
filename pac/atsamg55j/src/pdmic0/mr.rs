#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `CLKS` reader - Clock Source Selection"]
pub type ClksR = crate::BitReader;
#[doc = "Field `CLKS` writer - Clock Source Selection"]
pub type ClksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCAL` reader - Prescaler Rate Selection"]
pub type PrescalR = crate::FieldReader;
#[doc = "Field `PRESCAL` writer - Prescaler Rate Selection"]
pub type PrescalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 4 - Clock Source Selection"]
    #[inline(always)]
    pub fn clks(&self) -> ClksR {
        ClksR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&self) -> PrescalR {
        PrescalR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clks(&mut self) -> ClksW<MrSpec> {
        ClksW::new(self, 4)
    }
    #[doc = "Bits 8:14 - Prescaler Rate Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prescal(&mut self) -> PrescalW<MrSpec> {
        PrescalW::new(self, 8)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0x00f0_0000"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0x00f0_0000;
}
