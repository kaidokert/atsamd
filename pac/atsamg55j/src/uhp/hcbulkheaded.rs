#[doc = "Register `HCBULKHEADED` reader"]
pub type R = crate::R<HcbulkheadedSpec>;
#[doc = "Register `HCBULKHEADED` writer"]
pub type W = crate::W<HcbulkheadedSpec>;
#[doc = "Field `BHED` reader - Physical address of the head ED on the bulk ED list"]
pub type BhedR = crate::FieldReader<u32>;
#[doc = "Field `BHED` writer - Physical address of the head ED on the bulk ED list"]
pub type BhedW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - Physical address of the head ED on the bulk ED list"]
    #[inline(always)]
    pub fn bhed(&self) -> BhedR {
        BhedR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - Physical address of the head ED on the bulk ED list"]
    #[inline(always)]
    #[must_use]
    pub fn bhed(&mut self) -> BhedW<HcbulkheadedSpec> {
        BhedW::new(self, 4)
    }
}
#[doc = "HC Head Bulk Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbulkheaded::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcbulkheaded::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcbulkheadedSpec;
impl crate::RegisterSpec for HcbulkheadedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcbulkheaded::R`](R) reader structure"]
impl crate::Readable for HcbulkheadedSpec {}
#[doc = "`write(|w| ..)` method takes [`hcbulkheaded::W`](W) writer structure"]
impl crate::Writable for HcbulkheadedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCBULKHEADED to value 0"]
impl crate::Resettable for HcbulkheadedSpec {
    const RESET_VALUE: u32 = 0;
}
