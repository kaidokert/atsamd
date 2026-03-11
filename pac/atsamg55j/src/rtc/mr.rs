#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `HRMOD` reader - 12-/24-hour Mode"]
pub type HrmodR = crate::BitReader;
#[doc = "Field `HRMOD` writer - 12-/24-hour Mode"]
pub type HrmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERSIAN` reader - PERSIAN Calendar"]
pub type PersianR = crate::BitReader;
#[doc = "Field `PERSIAN` writer - PERSIAN Calendar"]
pub type PersianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEGPPM` reader - NEGative PPM Correction"]
pub type NegppmR = crate::BitReader;
#[doc = "Field `NEGPPM` writer - NEGative PPM Correction"]
pub type NegppmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORRECTION` reader - Slow Clock Correction"]
pub type CorrectionR = crate::FieldReader;
#[doc = "Field `CORRECTION` writer - Slow Clock Correction"]
pub type CorrectionW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HIGHPPM` reader - HIGH PPM Correction"]
pub type HighppmR = crate::BitReader;
#[doc = "Field `HIGHPPM` writer - HIGH PPM Correction"]
pub type HighppmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "All ADC Channel Trigger Event Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Out0 {
    #[doc = "0: No waveform, stuck at '0'"]
    NoWave = 0,
    #[doc = "1: 1 Hz square wave"]
    Freq1hz = 1,
    #[doc = "2: 32 Hz square wave"]
    Freq32hz = 2,
    #[doc = "3: 64 Hz square wave"]
    Freq64hz = 3,
    #[doc = "4: 512 Hz square wave"]
    Freq512hz = 4,
    #[doc = "6: Output is a copy of the alarm flag"]
    AlarmFlag = 6,
}
impl From<Out0> for u8 {
    #[inline(always)]
    fn from(variant: Out0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Out0 {
    type Ux = u8;
}
impl crate::IsEnum for Out0 {}
#[doc = "Field `OUT0` reader - All ADC Channel Trigger Event Source Selection"]
pub type Out0R = crate::FieldReader<Out0>;
impl Out0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Out0> {
        match self.bits {
            0 => Some(Out0::NoWave),
            1 => Some(Out0::Freq1hz),
            2 => Some(Out0::Freq32hz),
            3 => Some(Out0::Freq64hz),
            4 => Some(Out0::Freq512hz),
            6 => Some(Out0::AlarmFlag),
            _ => None,
        }
    }
    #[doc = "No waveform, stuck at '0'"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        *self == Out0::NoWave
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == Out0::Freq1hz
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        *self == Out0::Freq32hz
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        *self == Out0::Freq64hz
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == Out0::Freq512hz
    }
    #[doc = "Output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        *self == Out0::AlarmFlag
    }
}
#[doc = "Field `OUT0` writer - All ADC Channel Trigger Event Source Selection"]
pub type Out0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Out0>;
impl<'a, REG> Out0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::NoWave)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Freq1hz)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Freq32hz)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Freq64hz)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Freq512hz)
    }
    #[doc = "Output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::AlarmFlag)
    }
}
#[doc = "ADC Last Channel Trigger Event Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Out1 {
    #[doc = "0: No waveform, stuck at '0'"]
    NoWave = 0,
    #[doc = "1: 1 Hz square wave"]
    Freq1hz = 1,
    #[doc = "2: 32 Hz square wave"]
    Freq32hz = 2,
    #[doc = "3: 64 Hz square wave"]
    Freq64hz = 3,
    #[doc = "4: 512 Hz square wave"]
    Freq512hz = 4,
    #[doc = "6: Output is a copy of the alarm flag"]
    AlarmFlag = 6,
}
impl From<Out1> for u8 {
    #[inline(always)]
    fn from(variant: Out1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Out1 {
    type Ux = u8;
}
impl crate::IsEnum for Out1 {}
#[doc = "Field `OUT1` reader - ADC Last Channel Trigger Event Source Selection"]
pub type Out1R = crate::FieldReader<Out1>;
impl Out1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Out1> {
        match self.bits {
            0 => Some(Out1::NoWave),
            1 => Some(Out1::Freq1hz),
            2 => Some(Out1::Freq32hz),
            3 => Some(Out1::Freq64hz),
            4 => Some(Out1::Freq512hz),
            6 => Some(Out1::AlarmFlag),
            _ => None,
        }
    }
    #[doc = "No waveform, stuck at '0'"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        *self == Out1::NoWave
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == Out1::Freq1hz
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        *self == Out1::Freq32hz
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        *self == Out1::Freq64hz
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == Out1::Freq512hz
    }
    #[doc = "Output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        *self == Out1::AlarmFlag
    }
}
#[doc = "Field `OUT1` writer - ADC Last Channel Trigger Event Source Selection"]
pub type Out1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Out1>;
impl<'a, REG> Out1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::NoWave)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Freq1hz)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Freq32hz)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Freq64hz)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Freq512hz)
    }
    #[doc = "Output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::AlarmFlag)
    }
}
impl R {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    pub fn hrmod(&self) -> HrmodR {
        HrmodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    pub fn persian(&self) -> PersianR {
        PersianR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    pub fn negppm(&self) -> NegppmR {
        NegppmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    pub fn correction(&self) -> CorrectionR {
        CorrectionR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    pub fn highppm(&self) -> HighppmR {
        HighppmR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - All ADC Channel Trigger Event Source Selection"]
    #[inline(always)]
    pub fn out0(&self) -> Out0R {
        Out0R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - ADC Last Channel Trigger Event Source Selection"]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hrmod(&mut self) -> HrmodW<MrSpec> {
        HrmodW::new(self, 0)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    #[must_use]
    pub fn persian(&mut self) -> PersianW<MrSpec> {
        PersianW::new(self, 1)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    #[must_use]
    pub fn negppm(&mut self) -> NegppmW<MrSpec> {
        NegppmW::new(self, 4)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    #[must_use]
    pub fn correction(&mut self) -> CorrectionW<MrSpec> {
        CorrectionW::new(self, 8)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    #[must_use]
    pub fn highppm(&mut self) -> HighppmW<MrSpec> {
        HighppmW::new(self, 15)
    }
    #[doc = "Bits 16:18 - All ADC Channel Trigger Event Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> Out0W<MrSpec> {
        Out0W::new(self, 16)
    }
    #[doc = "Bits 20:22 - ADC Last Channel Trigger Event Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> Out1W<MrSpec> {
        Out1W::new(self, 20)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
