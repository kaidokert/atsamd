#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "FLEXCOM Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opmode {
    #[doc = "0: No communication"]
    NoCom = 0,
    #[doc = "1: All related USART related protocols are selected (RS232, RS485, IrDA, ISO7816, LIN,)All SPI/TWI related registers are not accessible and have no impact on IOs."]
    Usart = 1,
    #[doc = "2: SPI operating mode is selected.All USART/TWI related registers are not accessible and have no impact on IOs."]
    Spi = 2,
    #[doc = "3: All related TWI protocols are selected (TWI, SMBUS). All USART/SPI related registers are not accessible and have no impact on IOs."]
    Twi = 3,
}
impl From<Opmode> for u8 {
    #[inline(always)]
    fn from(variant: Opmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opmode {
    type Ux = u8;
}
impl crate::IsEnum for Opmode {}
#[doc = "Field `OPMODE` reader - FLEXCOM Operating Mode"]
pub type OpmodeR = crate::FieldReader<Opmode>;
impl OpmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opmode {
        match self.bits {
            0 => Opmode::NoCom,
            1 => Opmode::Usart,
            2 => Opmode::Spi,
            3 => Opmode::Twi,
            _ => unreachable!(),
        }
    }
    #[doc = "No communication"]
    #[inline(always)]
    pub fn is_no_com(&self) -> bool {
        *self == Opmode::NoCom
    }
    #[doc = "All related USART related protocols are selected (RS232, RS485, IrDA, ISO7816, LIN,)All SPI/TWI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn is_usart(&self) -> bool {
        *self == Opmode::Usart
    }
    #[doc = "SPI operating mode is selected.All USART/TWI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Opmode::Spi
    }
    #[doc = "All related TWI protocols are selected (TWI, SMBUS). All USART/SPI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn is_twi(&self) -> bool {
        *self == Opmode::Twi
    }
}
#[doc = "Field `OPMODE` writer - FLEXCOM Operating Mode"]
pub type OpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Opmode, crate::Safe>;
impl<'a, REG> OpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No communication"]
    #[inline(always)]
    pub fn no_com(self) -> &'a mut crate::W<REG> {
        self.variant(Opmode::NoCom)
    }
    #[doc = "All related USART related protocols are selected (RS232, RS485, IrDA, ISO7816, LIN,)All SPI/TWI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn usart(self) -> &'a mut crate::W<REG> {
        self.variant(Opmode::Usart)
    }
    #[doc = "SPI operating mode is selected.All USART/TWI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Opmode::Spi)
    }
    #[doc = "All related TWI protocols are selected (TWI, SMBUS). All USART/SPI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn twi(self) -> &'a mut crate::W<REG> {
        self.variant(Opmode::Twi)
    }
}
impl R {
    #[doc = "Bits 0:1 - FLEXCOM Operating Mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OpmodeR {
        OpmodeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FLEXCOM Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn opmode(&mut self) -> OpmodeW<MrSpec> {
        OpmodeW::new(self, 0)
    }
}
#[doc = "FLEXCOM Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
