#[doc = "Register `MATRIX_WPMR` reader"]
pub type R = crate::R<MatrixWpmrSpec>;
#[doc = "Register `MATRIX_WPMR` writer"]
pub type W = crate::W<MatrixWpmrSpec>;
#[doc = "Field `WPEN` reader - Write Protection Enable"]
pub type WpenR = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protection Enable"]
pub type WpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Wpkey {
    #[doc = "5062996: Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    Passwd = 5062996,
}
impl From<Wpkey> for u32 {
    #[inline(always)]
    fn from(variant: Wpkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wpkey {
    type Ux = u32;
}
impl crate::IsEnum for Wpkey {}
#[doc = "Field `WPKEY` reader - Write Protection Key"]
pub type WpkeyR = crate::FieldReader<Wpkey>;
impl WpkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wpkey> {
        match self.bits {
            5062996 => Some(Wpkey::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Wpkey::Passwd
    }
}
#[doc = "Field `WPKEY` writer - Write Protection Key"]
pub type WpkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, Wpkey>;
impl<'a, REG> WpkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Wpkey::Passwd)
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WpenR {
        WpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&self) -> WpkeyR {
        WpkeyR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WpenW<MatrixWpmrSpec> {
        WpenW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WpkeyW<MatrixWpmrSpec> {
        WpkeyW::new(self, 8)
    }
}
#[doc = "Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_wpmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_wpmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatrixWpmrSpec;
impl crate::RegisterSpec for MatrixWpmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matrix_wpmr::R`](R) reader structure"]
impl crate::Readable for MatrixWpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`matrix_wpmr::W`](W) writer structure"]
impl crate::Writable for MatrixWpmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MATRIX_WPMR to value 0"]
impl crate::Resettable for MatrixWpmrSpec {
    const RESET_VALUE: u32 = 0;
}
