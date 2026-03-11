#[doc = "Register `MATRIX_SCFG[%s]` reader"]
pub type R = crate::R<MatrixScfgSpec>;
#[doc = "Register `MATRIX_SCFG[%s]` writer"]
pub type W = crate::W<MatrixScfgSpec>;
#[doc = "Field `SLOT_CYCLE` reader - Maximum Number of Allowed Cycles for a Burst"]
pub type SlotCycleR = crate::FieldReader;
#[doc = "Field `SLOT_CYCLE` writer - Maximum Number of Allowed Cycles for a Burst"]
pub type SlotCycleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Default Master Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DefmstrType {
    #[doc = "0: At the end of current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in having a one cycle latency for the first access of a burst transfer or for a single access."]
    NoDefault = 0,
    #[doc = "1: At the end of current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having the one cycle latency when the last master tries to access the slave again."]
    Last = 1,
    #[doc = "2: At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having the one cycle latency when the fixed master tries to access the slave again."]
    Fixed = 2,
}
impl From<DefmstrType> for u8 {
    #[inline(always)]
    fn from(variant: DefmstrType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DefmstrType {
    type Ux = u8;
}
impl crate::IsEnum for DefmstrType {}
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub type DefmstrTypeR = crate::FieldReader<DefmstrType>;
impl DefmstrTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DefmstrType> {
        match self.bits {
            0 => Some(DefmstrType::NoDefault),
            1 => Some(DefmstrType::Last),
            2 => Some(DefmstrType::Fixed),
            _ => None,
        }
    }
    #[doc = "At the end of current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in having a one cycle latency for the first access of a burst transfer or for a single access."]
    #[inline(always)]
    pub fn is_no_default(&self) -> bool {
        *self == DefmstrType::NoDefault
    }
    #[doc = "At the end of current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having the one cycle latency when the last master tries to access the slave again."]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == DefmstrType::Last
    }
    #[doc = "At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having the one cycle latency when the fixed master tries to access the slave again."]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == DefmstrType::Fixed
    }
}
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub type DefmstrTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, DefmstrType>;
impl<'a, REG> DefmstrTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "At the end of current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in having a one cycle latency for the first access of a burst transfer or for a single access."]
    #[inline(always)]
    pub fn no_default(self) -> &'a mut crate::W<REG> {
        self.variant(DefmstrType::NoDefault)
    }
    #[doc = "At the end of current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having the one cycle latency when the last master tries to access the slave again."]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(DefmstrType::Last)
    }
    #[doc = "At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having the one cycle latency when the fixed master tries to access the slave again."]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(DefmstrType::Fixed)
    }
}
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Default Master"]
pub type FixedDefmstrR = crate::FieldReader;
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Default Master"]
pub type FixedDefmstrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SlotCycleR {
        SlotCycleR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DefmstrTypeR {
        DefmstrTypeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FixedDefmstrR {
        FixedDefmstrR::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    #[must_use]
    pub fn slot_cycle(&mut self) -> SlotCycleW<MatrixScfgSpec> {
        SlotCycleW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    #[must_use]
    pub fn defmstr_type(&mut self) -> DefmstrTypeW<MatrixScfgSpec> {
        DefmstrTypeW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    #[must_use]
    pub fn fixed_defmstr(&mut self) -> FixedDefmstrW<MatrixScfgSpec> {
        FixedDefmstrW::new(self, 18)
    }
}
#[doc = "Slave Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_scfg::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_scfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatrixScfgSpec;
impl crate::RegisterSpec for MatrixScfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matrix_scfg::R`](R) reader structure"]
impl crate::Readable for MatrixScfgSpec {}
#[doc = "`write(|w| ..)` method takes [`matrix_scfg::W`](W) writer structure"]
impl crate::Writable for MatrixScfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
