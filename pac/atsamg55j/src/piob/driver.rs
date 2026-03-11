#[doc = "Register `DRIVER` reader"]
pub type R = crate::R<DriverSpec>;
#[doc = "Register `DRIVER` writer"]
pub type W = crate::W<DriverSpec>;
#[doc = "Drive of PIO Line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line0 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line0> for bool {
    #[inline(always)]
    fn from(variant: Line0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE0` reader - Drive of PIO Line 0"]
pub type Line0R = crate::BitReader<Line0>;
impl Line0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line0 {
        match self.bits {
            false => Line0::LowDrive,
            true => Line0::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line0::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line0::HighDrive
    }
}
#[doc = "Field `LINE0` writer - Drive of PIO Line 0"]
pub type Line0W<'a, REG> = crate::BitWriter<'a, REG, Line0>;
impl<'a, REG> Line0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line0::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line0::HighDrive)
    }
}
#[doc = "Drive of PIO Line 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line1 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line1> for bool {
    #[inline(always)]
    fn from(variant: Line1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE1` reader - Drive of PIO Line 1"]
pub type Line1R = crate::BitReader<Line1>;
impl Line1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line1 {
        match self.bits {
            false => Line1::LowDrive,
            true => Line1::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line1::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line1::HighDrive
    }
}
#[doc = "Field `LINE1` writer - Drive of PIO Line 1"]
pub type Line1W<'a, REG> = crate::BitWriter<'a, REG, Line1>;
impl<'a, REG> Line1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line1::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line1::HighDrive)
    }
}
#[doc = "Drive of PIO Line 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line2 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line2> for bool {
    #[inline(always)]
    fn from(variant: Line2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE2` reader - Drive of PIO Line 2"]
pub type Line2R = crate::BitReader<Line2>;
impl Line2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line2 {
        match self.bits {
            false => Line2::LowDrive,
            true => Line2::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line2::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line2::HighDrive
    }
}
#[doc = "Field `LINE2` writer - Drive of PIO Line 2"]
pub type Line2W<'a, REG> = crate::BitWriter<'a, REG, Line2>;
impl<'a, REG> Line2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line2::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line2::HighDrive)
    }
}
#[doc = "Drive of PIO Line 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line3 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line3> for bool {
    #[inline(always)]
    fn from(variant: Line3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE3` reader - Drive of PIO Line 3"]
pub type Line3R = crate::BitReader<Line3>;
impl Line3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line3 {
        match self.bits {
            false => Line3::LowDrive,
            true => Line3::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line3::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line3::HighDrive
    }
}
#[doc = "Field `LINE3` writer - Drive of PIO Line 3"]
pub type Line3W<'a, REG> = crate::BitWriter<'a, REG, Line3>;
impl<'a, REG> Line3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line3::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line3::HighDrive)
    }
}
#[doc = "Drive of PIO Line 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line4 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line4> for bool {
    #[inline(always)]
    fn from(variant: Line4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE4` reader - Drive of PIO Line 4"]
pub type Line4R = crate::BitReader<Line4>;
impl Line4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line4 {
        match self.bits {
            false => Line4::LowDrive,
            true => Line4::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line4::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line4::HighDrive
    }
}
#[doc = "Field `LINE4` writer - Drive of PIO Line 4"]
pub type Line4W<'a, REG> = crate::BitWriter<'a, REG, Line4>;
impl<'a, REG> Line4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line4::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line4::HighDrive)
    }
}
#[doc = "Drive of PIO Line 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line5 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line5> for bool {
    #[inline(always)]
    fn from(variant: Line5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE5` reader - Drive of PIO Line 5"]
pub type Line5R = crate::BitReader<Line5>;
impl Line5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line5 {
        match self.bits {
            false => Line5::LowDrive,
            true => Line5::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line5::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line5::HighDrive
    }
}
#[doc = "Field `LINE5` writer - Drive of PIO Line 5"]
pub type Line5W<'a, REG> = crate::BitWriter<'a, REG, Line5>;
impl<'a, REG> Line5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line5::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line5::HighDrive)
    }
}
#[doc = "Drive of PIO Line 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line6 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line6> for bool {
    #[inline(always)]
    fn from(variant: Line6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE6` reader - Drive of PIO Line 6"]
pub type Line6R = crate::BitReader<Line6>;
impl Line6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line6 {
        match self.bits {
            false => Line6::LowDrive,
            true => Line6::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line6::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line6::HighDrive
    }
}
#[doc = "Field `LINE6` writer - Drive of PIO Line 6"]
pub type Line6W<'a, REG> = crate::BitWriter<'a, REG, Line6>;
impl<'a, REG> Line6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line6::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line6::HighDrive)
    }
}
#[doc = "Drive of PIO Line 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line7 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line7> for bool {
    #[inline(always)]
    fn from(variant: Line7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE7` reader - Drive of PIO Line 7"]
pub type Line7R = crate::BitReader<Line7>;
impl Line7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line7 {
        match self.bits {
            false => Line7::LowDrive,
            true => Line7::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line7::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line7::HighDrive
    }
}
#[doc = "Field `LINE7` writer - Drive of PIO Line 7"]
pub type Line7W<'a, REG> = crate::BitWriter<'a, REG, Line7>;
impl<'a, REG> Line7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line7::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line7::HighDrive)
    }
}
#[doc = "Drive of PIO Line 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line8 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line8> for bool {
    #[inline(always)]
    fn from(variant: Line8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE8` reader - Drive of PIO Line 8"]
pub type Line8R = crate::BitReader<Line8>;
impl Line8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line8 {
        match self.bits {
            false => Line8::LowDrive,
            true => Line8::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line8::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line8::HighDrive
    }
}
#[doc = "Field `LINE8` writer - Drive of PIO Line 8"]
pub type Line8W<'a, REG> = crate::BitWriter<'a, REG, Line8>;
impl<'a, REG> Line8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line8::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line8::HighDrive)
    }
}
#[doc = "Drive of PIO Line 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line9 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line9> for bool {
    #[inline(always)]
    fn from(variant: Line9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE9` reader - Drive of PIO Line 9"]
pub type Line9R = crate::BitReader<Line9>;
impl Line9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line9 {
        match self.bits {
            false => Line9::LowDrive,
            true => Line9::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line9::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line9::HighDrive
    }
}
#[doc = "Field `LINE9` writer - Drive of PIO Line 9"]
pub type Line9W<'a, REG> = crate::BitWriter<'a, REG, Line9>;
impl<'a, REG> Line9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line9::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line9::HighDrive)
    }
}
#[doc = "Drive of PIO Line 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line10 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line10> for bool {
    #[inline(always)]
    fn from(variant: Line10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE10` reader - Drive of PIO Line 10"]
pub type Line10R = crate::BitReader<Line10>;
impl Line10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line10 {
        match self.bits {
            false => Line10::LowDrive,
            true => Line10::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line10::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line10::HighDrive
    }
}
#[doc = "Field `LINE10` writer - Drive of PIO Line 10"]
pub type Line10W<'a, REG> = crate::BitWriter<'a, REG, Line10>;
impl<'a, REG> Line10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line10::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line10::HighDrive)
    }
}
#[doc = "Drive of PIO Line 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line11 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line11> for bool {
    #[inline(always)]
    fn from(variant: Line11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE11` reader - Drive of PIO Line 11"]
pub type Line11R = crate::BitReader<Line11>;
impl Line11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line11 {
        match self.bits {
            false => Line11::LowDrive,
            true => Line11::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line11::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line11::HighDrive
    }
}
#[doc = "Field `LINE11` writer - Drive of PIO Line 11"]
pub type Line11W<'a, REG> = crate::BitWriter<'a, REG, Line11>;
impl<'a, REG> Line11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line11::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line11::HighDrive)
    }
}
#[doc = "Drive of PIO Line 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line12 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line12> for bool {
    #[inline(always)]
    fn from(variant: Line12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE12` reader - Drive of PIO Line 12"]
pub type Line12R = crate::BitReader<Line12>;
impl Line12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line12 {
        match self.bits {
            false => Line12::LowDrive,
            true => Line12::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line12::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line12::HighDrive
    }
}
#[doc = "Field `LINE12` writer - Drive of PIO Line 12"]
pub type Line12W<'a, REG> = crate::BitWriter<'a, REG, Line12>;
impl<'a, REG> Line12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line12::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line12::HighDrive)
    }
}
#[doc = "Drive of PIO Line 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line13 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line13> for bool {
    #[inline(always)]
    fn from(variant: Line13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE13` reader - Drive of PIO Line 13"]
pub type Line13R = crate::BitReader<Line13>;
impl Line13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line13 {
        match self.bits {
            false => Line13::LowDrive,
            true => Line13::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line13::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line13::HighDrive
    }
}
#[doc = "Field `LINE13` writer - Drive of PIO Line 13"]
pub type Line13W<'a, REG> = crate::BitWriter<'a, REG, Line13>;
impl<'a, REG> Line13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line13::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line13::HighDrive)
    }
}
#[doc = "Drive of PIO Line 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line14 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line14> for bool {
    #[inline(always)]
    fn from(variant: Line14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE14` reader - Drive of PIO Line 14"]
pub type Line14R = crate::BitReader<Line14>;
impl Line14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line14 {
        match self.bits {
            false => Line14::LowDrive,
            true => Line14::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line14::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line14::HighDrive
    }
}
#[doc = "Field `LINE14` writer - Drive of PIO Line 14"]
pub type Line14W<'a, REG> = crate::BitWriter<'a, REG, Line14>;
impl<'a, REG> Line14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line14::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line14::HighDrive)
    }
}
#[doc = "Drive of PIO Line 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line15 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line15> for bool {
    #[inline(always)]
    fn from(variant: Line15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE15` reader - Drive of PIO Line 15"]
pub type Line15R = crate::BitReader<Line15>;
impl Line15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line15 {
        match self.bits {
            false => Line15::LowDrive,
            true => Line15::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line15::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line15::HighDrive
    }
}
#[doc = "Field `LINE15` writer - Drive of PIO Line 15"]
pub type Line15W<'a, REG> = crate::BitWriter<'a, REG, Line15>;
impl<'a, REG> Line15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line15::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line15::HighDrive)
    }
}
#[doc = "Drive of PIO Line 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line16 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line16> for bool {
    #[inline(always)]
    fn from(variant: Line16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE16` reader - Drive of PIO Line 16"]
pub type Line16R = crate::BitReader<Line16>;
impl Line16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line16 {
        match self.bits {
            false => Line16::LowDrive,
            true => Line16::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line16::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line16::HighDrive
    }
}
#[doc = "Field `LINE16` writer - Drive of PIO Line 16"]
pub type Line16W<'a, REG> = crate::BitWriter<'a, REG, Line16>;
impl<'a, REG> Line16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line16::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line16::HighDrive)
    }
}
#[doc = "Drive of PIO Line 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line17 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line17> for bool {
    #[inline(always)]
    fn from(variant: Line17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE17` reader - Drive of PIO Line 17"]
pub type Line17R = crate::BitReader<Line17>;
impl Line17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line17 {
        match self.bits {
            false => Line17::LowDrive,
            true => Line17::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line17::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line17::HighDrive
    }
}
#[doc = "Field `LINE17` writer - Drive of PIO Line 17"]
pub type Line17W<'a, REG> = crate::BitWriter<'a, REG, Line17>;
impl<'a, REG> Line17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line17::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line17::HighDrive)
    }
}
#[doc = "Drive of PIO Line 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line18 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line18> for bool {
    #[inline(always)]
    fn from(variant: Line18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE18` reader - Drive of PIO Line 18"]
pub type Line18R = crate::BitReader<Line18>;
impl Line18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line18 {
        match self.bits {
            false => Line18::LowDrive,
            true => Line18::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line18::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line18::HighDrive
    }
}
#[doc = "Field `LINE18` writer - Drive of PIO Line 18"]
pub type Line18W<'a, REG> = crate::BitWriter<'a, REG, Line18>;
impl<'a, REG> Line18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line18::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line18::HighDrive)
    }
}
#[doc = "Drive of PIO Line 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line19 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line19> for bool {
    #[inline(always)]
    fn from(variant: Line19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE19` reader - Drive of PIO Line 19"]
pub type Line19R = crate::BitReader<Line19>;
impl Line19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line19 {
        match self.bits {
            false => Line19::LowDrive,
            true => Line19::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line19::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line19::HighDrive
    }
}
#[doc = "Field `LINE19` writer - Drive of PIO Line 19"]
pub type Line19W<'a, REG> = crate::BitWriter<'a, REG, Line19>;
impl<'a, REG> Line19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line19::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line19::HighDrive)
    }
}
#[doc = "Drive of PIO Line 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line20 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line20> for bool {
    #[inline(always)]
    fn from(variant: Line20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE20` reader - Drive of PIO Line 20"]
pub type Line20R = crate::BitReader<Line20>;
impl Line20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line20 {
        match self.bits {
            false => Line20::LowDrive,
            true => Line20::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line20::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line20::HighDrive
    }
}
#[doc = "Field `LINE20` writer - Drive of PIO Line 20"]
pub type Line20W<'a, REG> = crate::BitWriter<'a, REG, Line20>;
impl<'a, REG> Line20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line20::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line20::HighDrive)
    }
}
#[doc = "Drive of PIO Line 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line21 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line21> for bool {
    #[inline(always)]
    fn from(variant: Line21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE21` reader - Drive of PIO Line 21"]
pub type Line21R = crate::BitReader<Line21>;
impl Line21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line21 {
        match self.bits {
            false => Line21::LowDrive,
            true => Line21::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line21::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line21::HighDrive
    }
}
#[doc = "Field `LINE21` writer - Drive of PIO Line 21"]
pub type Line21W<'a, REG> = crate::BitWriter<'a, REG, Line21>;
impl<'a, REG> Line21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line21::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line21::HighDrive)
    }
}
#[doc = "Drive of PIO Line 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line22 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line22> for bool {
    #[inline(always)]
    fn from(variant: Line22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE22` reader - Drive of PIO Line 22"]
pub type Line22R = crate::BitReader<Line22>;
impl Line22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line22 {
        match self.bits {
            false => Line22::LowDrive,
            true => Line22::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line22::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line22::HighDrive
    }
}
#[doc = "Field `LINE22` writer - Drive of PIO Line 22"]
pub type Line22W<'a, REG> = crate::BitWriter<'a, REG, Line22>;
impl<'a, REG> Line22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line22::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line22::HighDrive)
    }
}
#[doc = "Drive of PIO Line 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line23 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line23> for bool {
    #[inline(always)]
    fn from(variant: Line23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE23` reader - Drive of PIO Line 23"]
pub type Line23R = crate::BitReader<Line23>;
impl Line23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line23 {
        match self.bits {
            false => Line23::LowDrive,
            true => Line23::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line23::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line23::HighDrive
    }
}
#[doc = "Field `LINE23` writer - Drive of PIO Line 23"]
pub type Line23W<'a, REG> = crate::BitWriter<'a, REG, Line23>;
impl<'a, REG> Line23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line23::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line23::HighDrive)
    }
}
#[doc = "Drive of PIO Line 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line24 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line24> for bool {
    #[inline(always)]
    fn from(variant: Line24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE24` reader - Drive of PIO Line 24"]
pub type Line24R = crate::BitReader<Line24>;
impl Line24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line24 {
        match self.bits {
            false => Line24::LowDrive,
            true => Line24::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line24::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line24::HighDrive
    }
}
#[doc = "Field `LINE24` writer - Drive of PIO Line 24"]
pub type Line24W<'a, REG> = crate::BitWriter<'a, REG, Line24>;
impl<'a, REG> Line24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line24::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line24::HighDrive)
    }
}
#[doc = "Drive of PIO Line 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line25 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line25> for bool {
    #[inline(always)]
    fn from(variant: Line25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE25` reader - Drive of PIO Line 25"]
pub type Line25R = crate::BitReader<Line25>;
impl Line25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line25 {
        match self.bits {
            false => Line25::LowDrive,
            true => Line25::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line25::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line25::HighDrive
    }
}
#[doc = "Field `LINE25` writer - Drive of PIO Line 25"]
pub type Line25W<'a, REG> = crate::BitWriter<'a, REG, Line25>;
impl<'a, REG> Line25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line25::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line25::HighDrive)
    }
}
#[doc = "Drive of PIO Line 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line26 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line26> for bool {
    #[inline(always)]
    fn from(variant: Line26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE26` reader - Drive of PIO Line 26"]
pub type Line26R = crate::BitReader<Line26>;
impl Line26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line26 {
        match self.bits {
            false => Line26::LowDrive,
            true => Line26::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line26::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line26::HighDrive
    }
}
#[doc = "Field `LINE26` writer - Drive of PIO Line 26"]
pub type Line26W<'a, REG> = crate::BitWriter<'a, REG, Line26>;
impl<'a, REG> Line26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line26::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line26::HighDrive)
    }
}
#[doc = "Drive of PIO Line 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line27 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line27> for bool {
    #[inline(always)]
    fn from(variant: Line27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE27` reader - Drive of PIO Line 27"]
pub type Line27R = crate::BitReader<Line27>;
impl Line27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line27 {
        match self.bits {
            false => Line27::LowDrive,
            true => Line27::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line27::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line27::HighDrive
    }
}
#[doc = "Field `LINE27` writer - Drive of PIO Line 27"]
pub type Line27W<'a, REG> = crate::BitWriter<'a, REG, Line27>;
impl<'a, REG> Line27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line27::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line27::HighDrive)
    }
}
#[doc = "Drive of PIO Line 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line28 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line28> for bool {
    #[inline(always)]
    fn from(variant: Line28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE28` reader - Drive of PIO Line 28"]
pub type Line28R = crate::BitReader<Line28>;
impl Line28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line28 {
        match self.bits {
            false => Line28::LowDrive,
            true => Line28::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line28::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line28::HighDrive
    }
}
#[doc = "Field `LINE28` writer - Drive of PIO Line 28"]
pub type Line28W<'a, REG> = crate::BitWriter<'a, REG, Line28>;
impl<'a, REG> Line28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line28::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line28::HighDrive)
    }
}
#[doc = "Drive of PIO Line 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line29 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line29> for bool {
    #[inline(always)]
    fn from(variant: Line29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE29` reader - Drive of PIO Line 29"]
pub type Line29R = crate::BitReader<Line29>;
impl Line29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line29 {
        match self.bits {
            false => Line29::LowDrive,
            true => Line29::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line29::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line29::HighDrive
    }
}
#[doc = "Field `LINE29` writer - Drive of PIO Line 29"]
pub type Line29W<'a, REG> = crate::BitWriter<'a, REG, Line29>;
impl<'a, REG> Line29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line29::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line29::HighDrive)
    }
}
#[doc = "Drive of PIO Line 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line30 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line30> for bool {
    #[inline(always)]
    fn from(variant: Line30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE30` reader - Drive of PIO Line 30"]
pub type Line30R = crate::BitReader<Line30>;
impl Line30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line30 {
        match self.bits {
            false => Line30::LowDrive,
            true => Line30::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line30::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line30::HighDrive
    }
}
#[doc = "Field `LINE30` writer - Drive of PIO Line 30"]
pub type Line30W<'a, REG> = crate::BitWriter<'a, REG, Line30>;
impl<'a, REG> Line30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line30::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line30::HighDrive)
    }
}
#[doc = "Drive of PIO Line 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line31 {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line31> for bool {
    #[inline(always)]
    fn from(variant: Line31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE31` reader - Drive of PIO Line 31"]
pub type Line31R = crate::BitReader<Line31>;
impl Line31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line31 {
        match self.bits {
            false => Line31::LowDrive,
            true => Line31::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line31::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line31::HighDrive
    }
}
#[doc = "Field `LINE31` writer - Drive of PIO Line 31"]
pub type Line31W<'a, REG> = crate::BitWriter<'a, REG, Line31>;
impl<'a, REG> Line31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line31::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line31::HighDrive)
    }
}
impl R {
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline(always)]
    pub fn line0(&self) -> Line0R {
        Line0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline(always)]
    pub fn line1(&self) -> Line1R {
        Line1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline(always)]
    pub fn line2(&self) -> Line2R {
        Line2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline(always)]
    pub fn line3(&self) -> Line3R {
        Line3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline(always)]
    pub fn line4(&self) -> Line4R {
        Line4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline(always)]
    pub fn line5(&self) -> Line5R {
        Line5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline(always)]
    pub fn line6(&self) -> Line6R {
        Line6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline(always)]
    pub fn line7(&self) -> Line7R {
        Line7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline(always)]
    pub fn line8(&self) -> Line8R {
        Line8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline(always)]
    pub fn line9(&self) -> Line9R {
        Line9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline(always)]
    pub fn line10(&self) -> Line10R {
        Line10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline(always)]
    pub fn line11(&self) -> Line11R {
        Line11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline(always)]
    pub fn line12(&self) -> Line12R {
        Line12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline(always)]
    pub fn line13(&self) -> Line13R {
        Line13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline(always)]
    pub fn line14(&self) -> Line14R {
        Line14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline(always)]
    pub fn line15(&self) -> Line15R {
        Line15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline(always)]
    pub fn line16(&self) -> Line16R {
        Line16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline(always)]
    pub fn line17(&self) -> Line17R {
        Line17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline(always)]
    pub fn line18(&self) -> Line18R {
        Line18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline(always)]
    pub fn line19(&self) -> Line19R {
        Line19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline(always)]
    pub fn line20(&self) -> Line20R {
        Line20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline(always)]
    pub fn line21(&self) -> Line21R {
        Line21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline(always)]
    pub fn line22(&self) -> Line22R {
        Line22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline(always)]
    pub fn line23(&self) -> Line23R {
        Line23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline(always)]
    pub fn line24(&self) -> Line24R {
        Line24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline(always)]
    pub fn line25(&self) -> Line25R {
        Line25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline(always)]
    pub fn line26(&self) -> Line26R {
        Line26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline(always)]
    pub fn line27(&self) -> Line27R {
        Line27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline(always)]
    pub fn line28(&self) -> Line28R {
        Line28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline(always)]
    pub fn line29(&self) -> Line29R {
        Line29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline(always)]
    pub fn line30(&self) -> Line30R {
        Line30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline(always)]
    pub fn line31(&self) -> Line31R {
        Line31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline(always)]
    #[must_use]
    pub fn line0(&mut self) -> Line0W<DriverSpec> {
        Line0W::new(self, 0)
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline(always)]
    #[must_use]
    pub fn line1(&mut self) -> Line1W<DriverSpec> {
        Line1W::new(self, 1)
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline(always)]
    #[must_use]
    pub fn line2(&mut self) -> Line2W<DriverSpec> {
        Line2W::new(self, 2)
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline(always)]
    #[must_use]
    pub fn line3(&mut self) -> Line3W<DriverSpec> {
        Line3W::new(self, 3)
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline(always)]
    #[must_use]
    pub fn line4(&mut self) -> Line4W<DriverSpec> {
        Line4W::new(self, 4)
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline(always)]
    #[must_use]
    pub fn line5(&mut self) -> Line5W<DriverSpec> {
        Line5W::new(self, 5)
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline(always)]
    #[must_use]
    pub fn line6(&mut self) -> Line6W<DriverSpec> {
        Line6W::new(self, 6)
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline(always)]
    #[must_use]
    pub fn line7(&mut self) -> Line7W<DriverSpec> {
        Line7W::new(self, 7)
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline(always)]
    #[must_use]
    pub fn line8(&mut self) -> Line8W<DriverSpec> {
        Line8W::new(self, 8)
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline(always)]
    #[must_use]
    pub fn line9(&mut self) -> Line9W<DriverSpec> {
        Line9W::new(self, 9)
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline(always)]
    #[must_use]
    pub fn line10(&mut self) -> Line10W<DriverSpec> {
        Line10W::new(self, 10)
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline(always)]
    #[must_use]
    pub fn line11(&mut self) -> Line11W<DriverSpec> {
        Line11W::new(self, 11)
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline(always)]
    #[must_use]
    pub fn line12(&mut self) -> Line12W<DriverSpec> {
        Line12W::new(self, 12)
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline(always)]
    #[must_use]
    pub fn line13(&mut self) -> Line13W<DriverSpec> {
        Line13W::new(self, 13)
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline(always)]
    #[must_use]
    pub fn line14(&mut self) -> Line14W<DriverSpec> {
        Line14W::new(self, 14)
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline(always)]
    #[must_use]
    pub fn line15(&mut self) -> Line15W<DriverSpec> {
        Line15W::new(self, 15)
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline(always)]
    #[must_use]
    pub fn line16(&mut self) -> Line16W<DriverSpec> {
        Line16W::new(self, 16)
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline(always)]
    #[must_use]
    pub fn line17(&mut self) -> Line17W<DriverSpec> {
        Line17W::new(self, 17)
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline(always)]
    #[must_use]
    pub fn line18(&mut self) -> Line18W<DriverSpec> {
        Line18W::new(self, 18)
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline(always)]
    #[must_use]
    pub fn line19(&mut self) -> Line19W<DriverSpec> {
        Line19W::new(self, 19)
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline(always)]
    #[must_use]
    pub fn line20(&mut self) -> Line20W<DriverSpec> {
        Line20W::new(self, 20)
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline(always)]
    #[must_use]
    pub fn line21(&mut self) -> Line21W<DriverSpec> {
        Line21W::new(self, 21)
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline(always)]
    #[must_use]
    pub fn line22(&mut self) -> Line22W<DriverSpec> {
        Line22W::new(self, 22)
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline(always)]
    #[must_use]
    pub fn line23(&mut self) -> Line23W<DriverSpec> {
        Line23W::new(self, 23)
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline(always)]
    #[must_use]
    pub fn line24(&mut self) -> Line24W<DriverSpec> {
        Line24W::new(self, 24)
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline(always)]
    #[must_use]
    pub fn line25(&mut self) -> Line25W<DriverSpec> {
        Line25W::new(self, 25)
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline(always)]
    #[must_use]
    pub fn line26(&mut self) -> Line26W<DriverSpec> {
        Line26W::new(self, 26)
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline(always)]
    #[must_use]
    pub fn line27(&mut self) -> Line27W<DriverSpec> {
        Line27W::new(self, 27)
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline(always)]
    #[must_use]
    pub fn line28(&mut self) -> Line28W<DriverSpec> {
        Line28W::new(self, 28)
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline(always)]
    #[must_use]
    pub fn line29(&mut self) -> Line29W<DriverSpec> {
        Line29W::new(self, 29)
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline(always)]
    #[must_use]
    pub fn line30(&mut self) -> Line30W<DriverSpec> {
        Line30W::new(self, 30)
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline(always)]
    #[must_use]
    pub fn line31(&mut self) -> Line31W<DriverSpec> {
        Line31W::new(self, 31)
    }
}
#[doc = "I/O Drive Register\n\nYou can [`read`](crate::Reg::read) this register and get [`driver::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`driver::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DriverSpec;
impl crate::RegisterSpec for DriverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`driver::R`](R) reader structure"]
impl crate::Readable for DriverSpec {}
#[doc = "`write(|w| ..)` method takes [`driver::W`](W) writer structure"]
impl crate::Writable for DriverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRIVER to value 0"]
impl crate::Resettable for DriverSpec {
    const RESET_VALUE: u32 = 0;
}
