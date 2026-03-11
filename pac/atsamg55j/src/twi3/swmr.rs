#[doc = "Register `SWMR` reader"]
pub type R = crate::R<SwmrSpec>;
#[doc = "Register `SWMR` writer"]
pub type W = crate::W<SwmrSpec>;
#[doc = "Field `SADR1` reader - Slave Address 1"]
pub type Sadr1R = crate::FieldReader;
#[doc = "Field `SADR1` writer - Slave Address 1"]
pub type Sadr1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADR2` reader - Slave Address 2"]
pub type Sadr2R = crate::FieldReader;
#[doc = "Field `SADR2` writer - Slave Address 2"]
pub type Sadr2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADR3` reader - Slave Address 3"]
pub type Sadr3R = crate::FieldReader;
#[doc = "Field `SADR3` writer - Slave Address 3"]
pub type Sadr3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DATAM` reader - Data Match"]
pub type DatamR = crate::FieldReader;
#[doc = "Field `DATAM` writer - Data Match"]
pub type DatamW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - Slave Address 1"]
    #[inline(always)]
    pub fn sadr1(&self) -> Sadr1R {
        Sadr1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Slave Address 2"]
    #[inline(always)]
    pub fn sadr2(&self) -> Sadr2R {
        Sadr2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Slave Address 3"]
    #[inline(always)]
    pub fn sadr3(&self) -> Sadr3R {
        Sadr3R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - Data Match"]
    #[inline(always)]
    pub fn datam(&self) -> DatamR {
        DatamR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn sadr1(&mut self) -> Sadr1W<SwmrSpec> {
        Sadr1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Slave Address 2"]
    #[inline(always)]
    #[must_use]
    pub fn sadr2(&mut self) -> Sadr2W<SwmrSpec> {
        Sadr2W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Slave Address 3"]
    #[inline(always)]
    #[must_use]
    pub fn sadr3(&mut self) -> Sadr3W<SwmrSpec> {
        Sadr3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data Match"]
    #[inline(always)]
    #[must_use]
    pub fn datam(&mut self) -> DatamW<SwmrSpec> {
        DatamW::new(self, 24)
    }
}
#[doc = "TWI SleepWalking Matching Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwmrSpec;
impl crate::RegisterSpec for SwmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swmr::R`](R) reader structure"]
impl crate::Readable for SwmrSpec {}
#[doc = "`write(|w| ..)` method takes [`swmr::W`](W) writer structure"]
impl crate::Writable for SwmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWMR to value 0"]
impl crate::Resettable for SwmrSpec {
    const RESET_VALUE: u32 = 0;
}
