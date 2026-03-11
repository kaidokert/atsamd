#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Transfer Size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsize {
    #[doc = "0: The buffer size is defined in byte."]
    T8bit = 0,
    #[doc = "1: The buffer size is defined in half-word (16-bit)."]
    T16bit = 1,
    #[doc = "2: The buffer size is defined in word (32-bit). Default value."]
    T32bit = 2,
}
impl From<Tsize> for u8 {
    #[inline(always)]
    fn from(variant: Tsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsize {
    type Ux = u8;
}
impl crate::IsEnum for Tsize {}
#[doc = "Field `TSIZE` reader - Transfer Size"]
pub type TsizeR = crate::FieldReader<Tsize>;
impl TsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsize> {
        match self.bits {
            0 => Some(Tsize::T8bit),
            1 => Some(Tsize::T16bit),
            2 => Some(Tsize::T32bit),
            _ => None,
        }
    }
    #[doc = "The buffer size is defined in byte."]
    #[inline(always)]
    pub fn is_t_8bit(&self) -> bool {
        *self == Tsize::T8bit
    }
    #[doc = "The buffer size is defined in half-word (16-bit)."]
    #[inline(always)]
    pub fn is_t_16bit(&self) -> bool {
        *self == Tsize::T16bit
    }
    #[doc = "The buffer size is defined in word (32-bit). Default value."]
    #[inline(always)]
    pub fn is_t_32bit(&self) -> bool {
        *self == Tsize::T32bit
    }
}
#[doc = "Field `TSIZE` writer - Transfer Size"]
pub type TsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tsize>;
impl<'a, REG> TsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The buffer size is defined in byte."]
    #[inline(always)]
    pub fn t_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Tsize::T8bit)
    }
    #[doc = "The buffer size is defined in half-word (16-bit)."]
    #[inline(always)]
    pub fn t_16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Tsize::T16bit)
    }
    #[doc = "The buffer size is defined in word (32-bit). Default value."]
    #[inline(always)]
    pub fn t_32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Tsize::T32bit)
    }
}
impl R {
    #[doc = "Bits 0:1 - Transfer Size"]
    #[inline(always)]
    pub fn tsize(&self) -> TsizeR {
        TsizeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn tsize(&mut self) -> TsizeW<MrSpec> {
        TsizeW::new(self, 0)
    }
}
#[doc = "Memory to Memory Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets MR to value 0x02"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0x02;
}
