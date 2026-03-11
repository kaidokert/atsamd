#[doc = "Register `TYPE` reader"]
pub type R = crate::R<TypeSpec>;
#[doc = "Field `AP` reader - Access Port Access Allowed"]
pub type ApR = crate::BitReader;
#[doc = "Field `GCLK` reader - Dynamic Clock Gating Supported"]
pub type GclkR = crate::BitReader;
#[doc = "Field `RANDP` reader - Random Selection Policy Supported"]
pub type RandpR = crate::BitReader;
#[doc = "Field `LRUP` reader - Least Recently Used Policy Supported"]
pub type LrupR = crate::BitReader;
#[doc = "Field `RRP` reader - Random Selection Policy Supported"]
pub type RrpR = crate::BitReader;
#[doc = "Number of Ways\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Waynum {
    #[doc = "0: Direct Mapped Cache"]
    Dmapped = 0,
    #[doc = "1: 2-way set associative"]
    Arch2way = 1,
    #[doc = "2: 4-way set associative"]
    Arch4way = 2,
    #[doc = "3: 8-way set associative"]
    Arch8way = 3,
}
impl From<Waynum> for u8 {
    #[inline(always)]
    fn from(variant: Waynum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Waynum {
    type Ux = u8;
}
impl crate::IsEnum for Waynum {}
#[doc = "Field `WAYNUM` reader - Number of Ways"]
pub type WaynumR = crate::FieldReader<Waynum>;
impl WaynumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Waynum {
        match self.bits {
            0 => Waynum::Dmapped,
            1 => Waynum::Arch2way,
            2 => Waynum::Arch4way,
            3 => Waynum::Arch8way,
            _ => unreachable!(),
        }
    }
    #[doc = "Direct Mapped Cache"]
    #[inline(always)]
    pub fn is_dmapped(&self) -> bool {
        *self == Waynum::Dmapped
    }
    #[doc = "2-way set associative"]
    #[inline(always)]
    pub fn is_arch2way(&self) -> bool {
        *self == Waynum::Arch2way
    }
    #[doc = "4-way set associative"]
    #[inline(always)]
    pub fn is_arch4way(&self) -> bool {
        *self == Waynum::Arch4way
    }
    #[doc = "8-way set associative"]
    #[inline(always)]
    pub fn is_arch8way(&self) -> bool {
        *self == Waynum::Arch8way
    }
}
#[doc = "Field `LCKDOWN` reader - Lockdown Supported"]
pub type LckdownR = crate::BitReader;
#[doc = "Data Cache Size\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csize {
    #[doc = "0: Data cache size is 1 Kbyte"]
    Csize1kb = 0,
    #[doc = "1: Data cache size is 2 Kbytes"]
    Csize2kb = 1,
    #[doc = "2: Data cache size is 4 Kbytes"]
    Csize4kb = 2,
    #[doc = "3: Data cache size is 8 Kbytes"]
    Csize8kb = 3,
}
impl From<Csize> for u8 {
    #[inline(always)]
    fn from(variant: Csize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csize {
    type Ux = u8;
}
impl crate::IsEnum for Csize {}
#[doc = "Field `CSIZE` reader - Data Cache Size"]
pub type CsizeR = crate::FieldReader<Csize>;
impl CsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csize> {
        match self.bits {
            0 => Some(Csize::Csize1kb),
            1 => Some(Csize::Csize2kb),
            2 => Some(Csize::Csize4kb),
            3 => Some(Csize::Csize8kb),
            _ => None,
        }
    }
    #[doc = "Data cache size is 1 Kbyte"]
    #[inline(always)]
    pub fn is_csize_1kb(&self) -> bool {
        *self == Csize::Csize1kb
    }
    #[doc = "Data cache size is 2 Kbytes"]
    #[inline(always)]
    pub fn is_csize_2kb(&self) -> bool {
        *self == Csize::Csize2kb
    }
    #[doc = "Data cache size is 4 Kbytes"]
    #[inline(always)]
    pub fn is_csize_4kb(&self) -> bool {
        *self == Csize::Csize4kb
    }
    #[doc = "Data cache size is 8 Kbytes"]
    #[inline(always)]
    pub fn is_csize_8kb(&self) -> bool {
        *self == Csize::Csize8kb
    }
}
#[doc = "Cache LIne Size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clsize {
    #[doc = "0: Cache line size is 4 bytes"]
    Clsize1kb = 0,
    #[doc = "1: Cache line size is 8 bytes"]
    Clsize2kb = 1,
    #[doc = "2: Cache line size is 16 bytes"]
    Clsize4kb = 2,
    #[doc = "3: Cache line size is 32 bytes"]
    Clsize8kb = 3,
}
impl From<Clsize> for u8 {
    #[inline(always)]
    fn from(variant: Clsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clsize {
    type Ux = u8;
}
impl crate::IsEnum for Clsize {}
#[doc = "Field `CLSIZE` reader - Cache LIne Size"]
pub type ClsizeR = crate::FieldReader<Clsize>;
impl ClsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clsize> {
        match self.bits {
            0 => Some(Clsize::Clsize1kb),
            1 => Some(Clsize::Clsize2kb),
            2 => Some(Clsize::Clsize4kb),
            3 => Some(Clsize::Clsize8kb),
            _ => None,
        }
    }
    #[doc = "Cache line size is 4 bytes"]
    #[inline(always)]
    pub fn is_clsize_1kb(&self) -> bool {
        *self == Clsize::Clsize1kb
    }
    #[doc = "Cache line size is 8 bytes"]
    #[inline(always)]
    pub fn is_clsize_2kb(&self) -> bool {
        *self == Clsize::Clsize2kb
    }
    #[doc = "Cache line size is 16 bytes"]
    #[inline(always)]
    pub fn is_clsize_4kb(&self) -> bool {
        *self == Clsize::Clsize4kb
    }
    #[doc = "Cache line size is 32 bytes"]
    #[inline(always)]
    pub fn is_clsize_8kb(&self) -> bool {
        *self == Clsize::Clsize8kb
    }
}
impl R {
    #[doc = "Bit 0 - Access Port Access Allowed"]
    #[inline(always)]
    pub fn ap(&self) -> ApR {
        ApR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dynamic Clock Gating Supported"]
    #[inline(always)]
    pub fn gclk(&self) -> GclkR {
        GclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Random Selection Policy Supported"]
    #[inline(always)]
    pub fn randp(&self) -> RandpR {
        RandpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Least Recently Used Policy Supported"]
    #[inline(always)]
    pub fn lrup(&self) -> LrupR {
        LrupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Random Selection Policy Supported"]
    #[inline(always)]
    pub fn rrp(&self) -> RrpR {
        RrpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Number of Ways"]
    #[inline(always)]
    pub fn waynum(&self) -> WaynumR {
        WaynumR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Lockdown Supported"]
    #[inline(always)]
    pub fn lckdown(&self) -> LckdownR {
        LckdownR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Cache Size"]
    #[inline(always)]
    pub fn csize(&self) -> CsizeR {
        CsizeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Cache LIne Size"]
    #[inline(always)]
    pub fn clsize(&self) -> ClsizeR {
        ClsizeR::new(((self.bits >> 11) & 7) as u8)
    }
}
#[doc = "Cache Controller Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`type_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TypeSpec;
impl crate::RegisterSpec for TypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`type_::R`](R) reader structure"]
impl crate::Readable for TypeSpec {}
#[doc = "`reset()` method sets TYPE to value 0x13d7"]
impl crate::Resettable for TypeSpec {
    const RESET_VALUE: u32 = 0x13d7;
}
