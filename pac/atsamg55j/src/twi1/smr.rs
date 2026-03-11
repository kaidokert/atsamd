#[doc = "Register `SMR` reader"]
pub type R = crate::R<SmrSpec>;
#[doc = "Register `SMR` writer"]
pub type W = crate::W<SmrSpec>;
#[doc = "Field `NACKEN` reader - Slave Receiver Data Phase NACK Enable"]
pub type NackenR = crate::BitReader;
#[doc = "Field `NACKEN` writer - Slave Receiver Data Phase NACK Enable"]
pub type NackenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMDA` reader - SMBus Default Address"]
pub type SmdaR = crate::BitReader;
#[doc = "Field `SMDA` writer - SMBus Default Address"]
pub type SmdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMHH` reader - SMBus Host Header"]
pub type SmhhR = crate::BitReader;
#[doc = "Field `SMHH` writer - SMBus Host Header"]
pub type SmhhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLWSDIS` reader - Clock Wait State Disable"]
pub type SclwsdisR = crate::BitReader;
#[doc = "Field `SCLWSDIS` writer - Clock Wait State Disable"]
pub type SclwsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK` reader - Slave Address Mask"]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Slave Address Mask"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADR` reader - Slave Address"]
pub type SadrR = crate::FieldReader;
#[doc = "Field `SADR` writer - Slave Address"]
pub type SadrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADR1EN` reader - Slave Address 1 Enable"]
pub type Sadr1enR = crate::BitReader;
#[doc = "Field `SADR1EN` writer - Slave Address 1 Enable"]
pub type Sadr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADR2EN` reader - Slave Address 2 Enable"]
pub type Sadr2enR = crate::BitReader;
#[doc = "Field `SADR2EN` writer - Slave Address 2 Enable"]
pub type Sadr2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADR3EN` reader - Slave Address 3 Enable"]
pub type Sadr3enR = crate::BitReader;
#[doc = "Field `SADR3EN` writer - Slave Address 3 Enable"]
pub type Sadr3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAMEN` reader - Data Matching Enable"]
pub type DatamenR = crate::BitReader;
#[doc = "Field `DATAMEN` writer - Data Matching Enable"]
pub type DatamenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK Enable"]
    #[inline(always)]
    pub fn nacken(&self) -> NackenR {
        NackenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&self) -> SmdaR {
        SmdaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&self) -> SmhhR {
        SmhhR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    pub fn sclwsdis(&self) -> SclwsdisR {
        SclwsdisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&self) -> SadrR {
        SadrR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - Slave Address 1 Enable"]
    #[inline(always)]
    pub fn sadr1en(&self) -> Sadr1enR {
        Sadr1enR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Slave Address 2 Enable"]
    #[inline(always)]
    pub fn sadr2en(&self) -> Sadr2enR {
        Sadr2enR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Slave Address 3 Enable"]
    #[inline(always)]
    pub fn sadr3en(&self) -> Sadr3enR {
        Sadr3enR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data Matching Enable"]
    #[inline(always)]
    pub fn datamen(&self) -> DatamenR {
        DatamenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nacken(&mut self) -> NackenW<SmrSpec> {
        NackenW::new(self, 0)
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    #[must_use]
    pub fn smda(&mut self) -> SmdaW<SmrSpec> {
        SmdaW::new(self, 2)
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    #[must_use]
    pub fn smhh(&mut self) -> SmhhW<SmrSpec> {
        SmhhW::new(self, 3)
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sclwsdis(&mut self) -> SclwsdisW<SmrSpec> {
        SclwsdisW::new(self, 6)
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<SmrSpec> {
        MaskW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn sadr(&mut self) -> SadrW<SmrSpec> {
        SadrW::new(self, 16)
    }
    #[doc = "Bit 28 - Slave Address 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sadr1en(&mut self) -> Sadr1enW<SmrSpec> {
        Sadr1enW::new(self, 28)
    }
    #[doc = "Bit 29 - Slave Address 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sadr2en(&mut self) -> Sadr2enW<SmrSpec> {
        Sadr2enW::new(self, 29)
    }
    #[doc = "Bit 30 - Slave Address 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sadr3en(&mut self) -> Sadr3enW<SmrSpec> {
        Sadr3enW::new(self, 30)
    }
    #[doc = "Bit 31 - Data Matching Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datamen(&mut self) -> DatamenW<SmrSpec> {
        DatamenW::new(self, 31)
    }
}
#[doc = "TWI Slave Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmrSpec;
impl crate::RegisterSpec for SmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smr::R`](R) reader structure"]
impl crate::Readable for SmrSpec {}
#[doc = "`write(|w| ..)` method takes [`smr::W`](W) writer structure"]
impl crate::Writable for SmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMR to value 0"]
impl crate::Resettable for SmrSpec {
    const RESET_VALUE: u32 = 0;
}
