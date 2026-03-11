#[doc = "Register `COR` reader"]
pub type R = crate::R<CorSpec>;
#[doc = "Register `COR` writer"]
pub type W = crate::W<CorSpec>;
#[doc = "Field `OFF0` reader - Offset for Channel 0"]
pub type Off0R = crate::BitReader;
#[doc = "Field `OFF0` writer - Offset for Channel 0"]
pub type Off0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFF1` reader - Offset for Channel 1"]
pub type Off1R = crate::BitReader;
#[doc = "Field `OFF1` writer - Offset for Channel 1"]
pub type Off1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFF2` reader - Offset for Channel 2"]
pub type Off2R = crate::BitReader;
#[doc = "Field `OFF2` writer - Offset for Channel 2"]
pub type Off2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFF3` reader - Offset for Channel 3"]
pub type Off3R = crate::BitReader;
#[doc = "Field `OFF3` writer - Offset for Channel 3"]
pub type Off3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFF4` reader - Offset for Channel 4"]
pub type Off4R = crate::BitReader;
#[doc = "Field `OFF4` writer - Offset for Channel 4"]
pub type Off4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFF5` reader - Offset for Channel 5"]
pub type Off5R = crate::BitReader;
#[doc = "Field `OFF5` writer - Offset for Channel 5"]
pub type Off5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFF6` reader - Offset for Channel 6"]
pub type Off6R = crate::BitReader;
#[doc = "Field `OFF6` writer - Offset for Channel 6"]
pub type Off6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFF7` reader - Offset for Channel 7"]
pub type Off7R = crate::BitReader;
#[doc = "Field `OFF7` writer - Offset for Channel 7"]
pub type Off7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF0` reader - Differential Inputs for Channel 0"]
pub type Diff0R = crate::BitReader;
#[doc = "Field `DIFF0` writer - Differential Inputs for Channel 0"]
pub type Diff0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF1` reader - Differential Inputs for Channel 1"]
pub type Diff1R = crate::BitReader;
#[doc = "Field `DIFF1` writer - Differential Inputs for Channel 1"]
pub type Diff1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF2` reader - Differential Inputs for Channel 2"]
pub type Diff2R = crate::BitReader;
#[doc = "Field `DIFF2` writer - Differential Inputs for Channel 2"]
pub type Diff2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF3` reader - Differential Inputs for Channel 3"]
pub type Diff3R = crate::BitReader;
#[doc = "Field `DIFF3` writer - Differential Inputs for Channel 3"]
pub type Diff3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF4` reader - Differential Inputs for Channel 4"]
pub type Diff4R = crate::BitReader;
#[doc = "Field `DIFF4` writer - Differential Inputs for Channel 4"]
pub type Diff4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF5` reader - Differential Inputs for Channel 5"]
pub type Diff5R = crate::BitReader;
#[doc = "Field `DIFF5` writer - Differential Inputs for Channel 5"]
pub type Diff5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF6` reader - Differential Inputs for Channel 6"]
pub type Diff6R = crate::BitReader;
#[doc = "Field `DIFF6` writer - Differential Inputs for Channel 6"]
pub type Diff6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF7` reader - Differential Inputs for Channel 7"]
pub type Diff7R = crate::BitReader;
#[doc = "Field `DIFF7` writer - Differential Inputs for Channel 7"]
pub type Diff7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Offset for Channel 0"]
    #[inline(always)]
    pub fn off0(&self) -> Off0R {
        Off0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Offset for Channel 1"]
    #[inline(always)]
    pub fn off1(&self) -> Off1R {
        Off1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Offset for Channel 2"]
    #[inline(always)]
    pub fn off2(&self) -> Off2R {
        Off2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Offset for Channel 3"]
    #[inline(always)]
    pub fn off3(&self) -> Off3R {
        Off3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Offset for Channel 4"]
    #[inline(always)]
    pub fn off4(&self) -> Off4R {
        Off4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Offset for Channel 5"]
    #[inline(always)]
    pub fn off5(&self) -> Off5R {
        Off5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Offset for Channel 6"]
    #[inline(always)]
    pub fn off6(&self) -> Off6R {
        Off6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Offset for Channel 7"]
    #[inline(always)]
    pub fn off7(&self) -> Off7R {
        Off7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Differential Inputs for Channel 0"]
    #[inline(always)]
    pub fn diff0(&self) -> Diff0R {
        Diff0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Differential Inputs for Channel 1"]
    #[inline(always)]
    pub fn diff1(&self) -> Diff1R {
        Diff1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Differential Inputs for Channel 2"]
    #[inline(always)]
    pub fn diff2(&self) -> Diff2R {
        Diff2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Differential Inputs for Channel 3"]
    #[inline(always)]
    pub fn diff3(&self) -> Diff3R {
        Diff3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Differential Inputs for Channel 4"]
    #[inline(always)]
    pub fn diff4(&self) -> Diff4R {
        Diff4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Differential Inputs for Channel 5"]
    #[inline(always)]
    pub fn diff5(&self) -> Diff5R {
        Diff5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Differential Inputs for Channel 6"]
    #[inline(always)]
    pub fn diff6(&self) -> Diff6R {
        Diff6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Differential Inputs for Channel 7"]
    #[inline(always)]
    pub fn diff7(&self) -> Diff7R {
        Diff7R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Offset for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn off0(&mut self) -> Off0W<CorSpec> {
        Off0W::new(self, 0)
    }
    #[doc = "Bit 1 - Offset for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn off1(&mut self) -> Off1W<CorSpec> {
        Off1W::new(self, 1)
    }
    #[doc = "Bit 2 - Offset for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn off2(&mut self) -> Off2W<CorSpec> {
        Off2W::new(self, 2)
    }
    #[doc = "Bit 3 - Offset for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn off3(&mut self) -> Off3W<CorSpec> {
        Off3W::new(self, 3)
    }
    #[doc = "Bit 4 - Offset for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn off4(&mut self) -> Off4W<CorSpec> {
        Off4W::new(self, 4)
    }
    #[doc = "Bit 5 - Offset for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn off5(&mut self) -> Off5W<CorSpec> {
        Off5W::new(self, 5)
    }
    #[doc = "Bit 6 - Offset for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn off6(&mut self) -> Off6W<CorSpec> {
        Off6W::new(self, 6)
    }
    #[doc = "Bit 7 - Offset for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn off7(&mut self) -> Off7W<CorSpec> {
        Off7W::new(self, 7)
    }
    #[doc = "Bit 16 - Differential Inputs for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn diff0(&mut self) -> Diff0W<CorSpec> {
        Diff0W::new(self, 16)
    }
    #[doc = "Bit 17 - Differential Inputs for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn diff1(&mut self) -> Diff1W<CorSpec> {
        Diff1W::new(self, 17)
    }
    #[doc = "Bit 18 - Differential Inputs for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn diff2(&mut self) -> Diff2W<CorSpec> {
        Diff2W::new(self, 18)
    }
    #[doc = "Bit 19 - Differential Inputs for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn diff3(&mut self) -> Diff3W<CorSpec> {
        Diff3W::new(self, 19)
    }
    #[doc = "Bit 20 - Differential Inputs for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn diff4(&mut self) -> Diff4W<CorSpec> {
        Diff4W::new(self, 20)
    }
    #[doc = "Bit 21 - Differential Inputs for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn diff5(&mut self) -> Diff5W<CorSpec> {
        Diff5W::new(self, 21)
    }
    #[doc = "Bit 22 - Differential Inputs for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn diff6(&mut self) -> Diff6W<CorSpec> {
        Diff6W::new(self, 22)
    }
    #[doc = "Bit 23 - Differential Inputs for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn diff7(&mut self) -> Diff7W<CorSpec> {
        Diff7W::new(self, 23)
    }
}
#[doc = "Channel Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CorSpec;
impl crate::RegisterSpec for CorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cor::R`](R) reader structure"]
impl crate::Readable for CorSpec {}
#[doc = "`write(|w| ..)` method takes [`cor::W`](W) writer structure"]
impl crate::Writable for CorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COR to value 0"]
impl crate::Resettable for CorSpec {
    const RESET_VALUE: u32 = 0;
}
