#[doc = "Register `MATRIX_MCFG[%s]` reader"]
pub type R = crate::R<MatrixMcfgSpec>;
#[doc = "Register `MATRIX_MCFG[%s]` writer"]
pub type W = crate::W<MatrixMcfgSpec>;
#[doc = "Undefined Length Burst Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ulbt {
    #[doc = "0: No predicted end of burst is generated and therefore INCR bursts coming from this master cannot be broken."]
    Unlimited = 0,
    #[doc = "1: The undefined length burst is treated as a succession of single access allowing rearbitration at each beat of the INCR burst."]
    Single = 1,
    #[doc = "2: The undefined length burst is split into a 4-beat bursts allowing rearbitration at each 4-beat burst end."]
    _4Beat = 2,
    #[doc = "3: The undefined length burst is split into 8-beat bursts allowing rearbitration at each 8-beat burst end."]
    _8Beat = 3,
    #[doc = "4: The undefined length burst is split into 16-beat bursts allowing rearbitration at each 16-beat burst end."]
    _16Beat = 4,
}
impl From<Ulbt> for u8 {
    #[inline(always)]
    fn from(variant: Ulbt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ulbt {
    type Ux = u8;
}
impl crate::IsEnum for Ulbt {}
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type UlbtR = crate::FieldReader<Ulbt>;
impl UlbtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ulbt> {
        match self.bits {
            0 => Some(Ulbt::Unlimited),
            1 => Some(Ulbt::Single),
            2 => Some(Ulbt::_4Beat),
            3 => Some(Ulbt::_8Beat),
            4 => Some(Ulbt::_16Beat),
            _ => None,
        }
    }
    #[doc = "No predicted end of burst is generated and therefore INCR bursts coming from this master cannot be broken."]
    #[inline(always)]
    pub fn is_unlimited(&self) -> bool {
        *self == Ulbt::Unlimited
    }
    #[doc = "The undefined length burst is treated as a succession of single access allowing rearbitration at each beat of the INCR burst."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Ulbt::Single
    }
    #[doc = "The undefined length burst is split into a 4-beat bursts allowing rearbitration at each 4-beat burst end."]
    #[inline(always)]
    pub fn is_4_beat(&self) -> bool {
        *self == Ulbt::_4Beat
    }
    #[doc = "The undefined length burst is split into 8-beat bursts allowing rearbitration at each 8-beat burst end."]
    #[inline(always)]
    pub fn is_8_beat(&self) -> bool {
        *self == Ulbt::_8Beat
    }
    #[doc = "The undefined length burst is split into 16-beat bursts allowing rearbitration at each 16-beat burst end."]
    #[inline(always)]
    pub fn is_16_beat(&self) -> bool {
        *self == Ulbt::_16Beat
    }
}
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type UlbtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ulbt>;
impl<'a, REG> UlbtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No predicted end of burst is generated and therefore INCR bursts coming from this master cannot be broken."]
    #[inline(always)]
    pub fn unlimited(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbt::Unlimited)
    }
    #[doc = "The undefined length burst is treated as a succession of single access allowing rearbitration at each beat of the INCR burst."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbt::Single)
    }
    #[doc = "The undefined length burst is split into a 4-beat bursts allowing rearbitration at each 4-beat burst end."]
    #[inline(always)]
    pub fn _4_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbt::_4Beat)
    }
    #[doc = "The undefined length burst is split into 8-beat bursts allowing rearbitration at each 8-beat burst end."]
    #[inline(always)]
    pub fn _8_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbt::_8Beat)
    }
    #[doc = "The undefined length burst is split into 16-beat bursts allowing rearbitration at each 16-beat burst end."]
    #[inline(always)]
    pub fn _16_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbt::_16Beat)
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> UlbtR {
        UlbtR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    #[must_use]
    pub fn ulbt(&mut self) -> UlbtW<MatrixMcfgSpec> {
        UlbtW::new(self, 0)
    }
}
#[doc = "Master Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_mcfg::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_mcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatrixMcfgSpec;
impl crate::RegisterSpec for MatrixMcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matrix_mcfg::R`](R) reader structure"]
impl crate::Readable for MatrixMcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`matrix_mcfg::W`](W) writer structure"]
impl crate::Writable for MatrixMcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
