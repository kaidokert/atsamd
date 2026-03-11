#[doc = "Register `OVER` reader"]
pub type R = crate::R<OverSpec>;
#[doc = "Field `OVRE0` reader - Overrun Error 0"]
pub type Ovre0R = crate::BitReader;
#[doc = "Field `OVRE1` reader - Overrun Error 1"]
pub type Ovre1R = crate::BitReader;
#[doc = "Field `OVRE2` reader - Overrun Error 2"]
pub type Ovre2R = crate::BitReader;
#[doc = "Field `OVRE3` reader - Overrun Error 3"]
pub type Ovre3R = crate::BitReader;
#[doc = "Field `OVRE4` reader - Overrun Error 4"]
pub type Ovre4R = crate::BitReader;
#[doc = "Field `OVRE5` reader - Overrun Error 5"]
pub type Ovre5R = crate::BitReader;
#[doc = "Field `OVRE6` reader - Overrun Error 6"]
pub type Ovre6R = crate::BitReader;
#[doc = "Field `OVRE7` reader - Overrun Error 7"]
pub type Ovre7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Overrun Error 0"]
    #[inline(always)]
    pub fn ovre0(&self) -> Ovre0R {
        Ovre0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error 1"]
    #[inline(always)]
    pub fn ovre1(&self) -> Ovre1R {
        Ovre1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overrun Error 2"]
    #[inline(always)]
    pub fn ovre2(&self) -> Ovre2R {
        Ovre2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error 3"]
    #[inline(always)]
    pub fn ovre3(&self) -> Ovre3R {
        Ovre3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overrun Error 4"]
    #[inline(always)]
    pub fn ovre4(&self) -> Ovre4R {
        Ovre4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error 5"]
    #[inline(always)]
    pub fn ovre5(&self) -> Ovre5R {
        Ovre5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun Error 6"]
    #[inline(always)]
    pub fn ovre6(&self) -> Ovre6R {
        Ovre6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Overrun Error 7"]
    #[inline(always)]
    pub fn ovre7(&self) -> Ovre7R {
        Ovre7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Overrun Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`over::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OverSpec;
impl crate::RegisterSpec for OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`over::R`](R) reader structure"]
impl crate::Readable for OverSpec {}
#[doc = "`reset()` method sets OVER to value 0"]
impl crate::Resettable for OverSpec {
    const RESET_VALUE: u32 = 0;
}
