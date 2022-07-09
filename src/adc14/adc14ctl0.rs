#[doc = "Register `ADC14CTL0` reader"]
pub struct R(crate::R<ADC14CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC14CTL0` writer"]
pub struct W(crate::W<ADC14CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC14CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ADC14CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC14CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC14 start conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14SC_A {
    #[doc = "0: No sample-and-conversion-start"]
    ADC14SC_0 = 0,
    #[doc = "1: Start sample-and-conversion"]
    ADC14SC_1 = 1,
}
impl From<ADC14SC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14SC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14SC` reader - ADC14 start conversion"]
pub type ADC14SC_R = crate::BitReader<ADC14SC_A>;
impl ADC14SC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14SC_A {
        match self.bits {
            false => ADC14SC_A::ADC14SC_0,
            true => ADC14SC_A::ADC14SC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14SC_0`"]
    #[inline(always)]
    pub fn is_adc14sc_0(&self) -> bool {
        *self == ADC14SC_A::ADC14SC_0
    }
    #[doc = "Checks if the value of the field is `ADC14SC_1`"]
    #[inline(always)]
    pub fn is_adc14sc_1(&self) -> bool {
        *self == ADC14SC_A::ADC14SC_1
    }
}
#[doc = "Field `ADC14SC` writer - ADC14 start conversion"]
pub type ADC14SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14CTL0_SPEC, ADC14SC_A, O>;
impl<'a, const O: u8> ADC14SC_W<'a, O> {
    #[doc = "No sample-and-conversion-start"]
    #[inline(always)]
    pub fn adc14sc_0(self) -> &'a mut W {
        self.variant(ADC14SC_A::ADC14SC_0)
    }
    #[doc = "Start sample-and-conversion"]
    #[inline(always)]
    pub fn adc14sc_1(self) -> &'a mut W {
        self.variant(ADC14SC_A::ADC14SC_1)
    }
}
#[doc = "ADC14 enable conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14ENC_A {
    #[doc = "0: ADC14 disabled"]
    ADC14ENC_0 = 0,
    #[doc = "1: ADC14 enabled"]
    ADC14ENC_1 = 1,
}
impl From<ADC14ENC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14ENC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14ENC` reader - ADC14 enable conversion"]
pub type ADC14ENC_R = crate::BitReader<ADC14ENC_A>;
impl ADC14ENC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14ENC_A {
        match self.bits {
            false => ADC14ENC_A::ADC14ENC_0,
            true => ADC14ENC_A::ADC14ENC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14ENC_0`"]
    #[inline(always)]
    pub fn is_adc14enc_0(&self) -> bool {
        *self == ADC14ENC_A::ADC14ENC_0
    }
    #[doc = "Checks if the value of the field is `ADC14ENC_1`"]
    #[inline(always)]
    pub fn is_adc14enc_1(&self) -> bool {
        *self == ADC14ENC_A::ADC14ENC_1
    }
}
#[doc = "Field `ADC14ENC` writer - ADC14 enable conversion"]
pub type ADC14ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14CTL0_SPEC, ADC14ENC_A, O>;
impl<'a, const O: u8> ADC14ENC_W<'a, O> {
    #[doc = "ADC14 disabled"]
    #[inline(always)]
    pub fn adc14enc_0(self) -> &'a mut W {
        self.variant(ADC14ENC_A::ADC14ENC_0)
    }
    #[doc = "ADC14 enabled"]
    #[inline(always)]
    pub fn adc14enc_1(self) -> &'a mut W {
        self.variant(ADC14ENC_A::ADC14ENC_1)
    }
}
#[doc = "ADC14 on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14ON_A {
    #[doc = "0: ADC14 off"]
    ADC14ON_0 = 0,
    #[doc = "1: ADC14 on. ADC core is ready to power up when a valid conversion is triggered."]
    ADC14ON_1 = 1,
}
impl From<ADC14ON_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14ON` reader - ADC14 on"]
pub type ADC14ON_R = crate::BitReader<ADC14ON_A>;
impl ADC14ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14ON_A {
        match self.bits {
            false => ADC14ON_A::ADC14ON_0,
            true => ADC14ON_A::ADC14ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14ON_0`"]
    #[inline(always)]
    pub fn is_adc14on_0(&self) -> bool {
        *self == ADC14ON_A::ADC14ON_0
    }
    #[doc = "Checks if the value of the field is `ADC14ON_1`"]
    #[inline(always)]
    pub fn is_adc14on_1(&self) -> bool {
        *self == ADC14ON_A::ADC14ON_1
    }
}
#[doc = "Field `ADC14ON` writer - ADC14 on"]
pub type ADC14ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14CTL0_SPEC, ADC14ON_A, O>;
impl<'a, const O: u8> ADC14ON_W<'a, O> {
    #[doc = "ADC14 off"]
    #[inline(always)]
    pub fn adc14on_0(self) -> &'a mut W {
        self.variant(ADC14ON_A::ADC14ON_0)
    }
    #[doc = "ADC14 on. ADC core is ready to power up when a valid conversion is triggered."]
    #[inline(always)]
    pub fn adc14on_1(self) -> &'a mut W {
        self.variant(ADC14ON_A::ADC14ON_1)
    }
}
#[doc = "ADC14 multiple sample and conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14MSC_A {
    #[doc = "0: The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert"]
    ADC14MSC_0 = 0,
    #[doc = "1: The first rising edge of the SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed"]
    ADC14MSC_1 = 1,
}
impl From<ADC14MSC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14MSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14MSC` reader - ADC14 multiple sample and conversion"]
pub type ADC14MSC_R = crate::BitReader<ADC14MSC_A>;
impl ADC14MSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14MSC_A {
        match self.bits {
            false => ADC14MSC_A::ADC14MSC_0,
            true => ADC14MSC_A::ADC14MSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14MSC_0`"]
    #[inline(always)]
    pub fn is_adc14msc_0(&self) -> bool {
        *self == ADC14MSC_A::ADC14MSC_0
    }
    #[doc = "Checks if the value of the field is `ADC14MSC_1`"]
    #[inline(always)]
    pub fn is_adc14msc_1(&self) -> bool {
        *self == ADC14MSC_A::ADC14MSC_1
    }
}
#[doc = "Field `ADC14MSC` writer - ADC14 multiple sample and conversion"]
pub type ADC14MSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14CTL0_SPEC, ADC14MSC_A, O>;
impl<'a, const O: u8> ADC14MSC_W<'a, O> {
    #[doc = "The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert"]
    #[inline(always)]
    pub fn adc14msc_0(self) -> &'a mut W {
        self.variant(ADC14MSC_A::ADC14MSC_0)
    }
    #[doc = "The first rising edge of the SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed"]
    #[inline(always)]
    pub fn adc14msc_1(self) -> &'a mut W {
        self.variant(ADC14MSC_A::ADC14MSC_1)
    }
}
#[doc = "ADC14 sample-and-hold time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC14SHT0_A {
    #[doc = "0: 4"]
    ADC14SHT0_0 = 0,
    #[doc = "1: 8"]
    ADC14SHT0_1 = 1,
    #[doc = "2: 16"]
    ADC14SHT0_2 = 2,
    #[doc = "3: 32"]
    ADC14SHT0_3 = 3,
    #[doc = "4: 64"]
    ADC14SHT0_4 = 4,
    #[doc = "5: 96"]
    ADC14SHT0_5 = 5,
    #[doc = "6: 128"]
    ADC14SHT0_6 = 6,
    #[doc = "7: 192"]
    ADC14SHT0_7 = 7,
}
impl From<ADC14SHT0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14SHT0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC14SHT0` reader - ADC14 sample-and-hold time"]
pub type ADC14SHT0_R = crate::FieldReader<u8, ADC14SHT0_A>;
impl ADC14SHT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC14SHT0_A> {
        match self.bits {
            0 => Some(ADC14SHT0_A::ADC14SHT0_0),
            1 => Some(ADC14SHT0_A::ADC14SHT0_1),
            2 => Some(ADC14SHT0_A::ADC14SHT0_2),
            3 => Some(ADC14SHT0_A::ADC14SHT0_3),
            4 => Some(ADC14SHT0_A::ADC14SHT0_4),
            5 => Some(ADC14SHT0_A::ADC14SHT0_5),
            6 => Some(ADC14SHT0_A::ADC14SHT0_6),
            7 => Some(ADC14SHT0_A::ADC14SHT0_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14SHT0_0`"]
    #[inline(always)]
    pub fn is_adc14sht0_0(&self) -> bool {
        *self == ADC14SHT0_A::ADC14SHT0_0
    }
    #[doc = "Checks if the value of the field is `ADC14SHT0_1`"]
    #[inline(always)]
    pub fn is_adc14sht0_1(&self) -> bool {
        *self == ADC14SHT0_A::ADC14SHT0_1
    }
    #[doc = "Checks if the value of the field is `ADC14SHT0_2`"]
    #[inline(always)]
    pub fn is_adc14sht0_2(&self) -> bool {
        *self == ADC14SHT0_A::ADC14SHT0_2
    }
    #[doc = "Checks if the value of the field is `ADC14SHT0_3`"]
    #[inline(always)]
    pub fn is_adc14sht0_3(&self) -> bool {
        *self == ADC14SHT0_A::ADC14SHT0_3
    }
    #[doc = "Checks if the value of the field is `ADC14SHT0_4`"]
    #[inline(always)]
    pub fn is_adc14sht0_4(&self) -> bool {
        *self == ADC14SHT0_A::ADC14SHT0_4
    }
    #[doc = "Checks if the value of the field is `ADC14SHT0_5`"]
    #[inline(always)]
    pub fn is_adc14sht0_5(&self) -> bool {
        *self == ADC14SHT0_A::ADC14SHT0_5
    }
    #[doc = "Checks if the value of the field is `ADC14SHT0_6`"]
    #[inline(always)]
    pub fn is_adc14sht0_6(&self) -> bool {
        *self == ADC14SHT0_A::ADC14SHT0_6
    }
    #[doc = "Checks if the value of the field is `ADC14SHT0_7`"]
    #[inline(always)]
    pub fn is_adc14sht0_7(&self) -> bool {
        *self == ADC14SHT0_A::ADC14SHT0_7
    }
}
#[doc = "Field `ADC14SHT0` writer - ADC14 sample-and-hold time"]
pub type ADC14SHT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC14CTL0_SPEC, u8, ADC14SHT0_A, 4, O>;
impl<'a, const O: u8> ADC14SHT0_W<'a, O> {
    #[doc = "4"]
    #[inline(always)]
    pub fn adc14sht0_0(self) -> &'a mut W {
        self.variant(ADC14SHT0_A::ADC14SHT0_0)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn adc14sht0_1(self) -> &'a mut W {
        self.variant(ADC14SHT0_A::ADC14SHT0_1)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn adc14sht0_2(self) -> &'a mut W {
        self.variant(ADC14SHT0_A::ADC14SHT0_2)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn adc14sht0_3(self) -> &'a mut W {
        self.variant(ADC14SHT0_A::ADC14SHT0_3)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn adc14sht0_4(self) -> &'a mut W {
        self.variant(ADC14SHT0_A::ADC14SHT0_4)
    }
    #[doc = "96"]
    #[inline(always)]
    pub fn adc14sht0_5(self) -> &'a mut W {
        self.variant(ADC14SHT0_A::ADC14SHT0_5)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn adc14sht0_6(self) -> &'a mut W {
        self.variant(ADC14SHT0_A::ADC14SHT0_6)
    }
    #[doc = "192"]
    #[inline(always)]
    pub fn adc14sht0_7(self) -> &'a mut W {
        self.variant(ADC14SHT0_A::ADC14SHT0_7)
    }
}
#[doc = "ADC14 sample-and-hold time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC14SHT1_A {
    #[doc = "0: 4"]
    ADC14SHT1_0 = 0,
    #[doc = "1: 8"]
    ADC14SHT1_1 = 1,
    #[doc = "2: 16"]
    ADC14SHT1_2 = 2,
    #[doc = "3: 32"]
    ADC14SHT1_3 = 3,
    #[doc = "4: 64"]
    ADC14SHT1_4 = 4,
    #[doc = "5: 96"]
    ADC14SHT1_5 = 5,
    #[doc = "6: 128"]
    ADC14SHT1_6 = 6,
    #[doc = "7: 192"]
    ADC14SHT1_7 = 7,
}
impl From<ADC14SHT1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14SHT1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC14SHT1` reader - ADC14 sample-and-hold time"]
pub type ADC14SHT1_R = crate::FieldReader<u8, ADC14SHT1_A>;
impl ADC14SHT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC14SHT1_A> {
        match self.bits {
            0 => Some(ADC14SHT1_A::ADC14SHT1_0),
            1 => Some(ADC14SHT1_A::ADC14SHT1_1),
            2 => Some(ADC14SHT1_A::ADC14SHT1_2),
            3 => Some(ADC14SHT1_A::ADC14SHT1_3),
            4 => Some(ADC14SHT1_A::ADC14SHT1_4),
            5 => Some(ADC14SHT1_A::ADC14SHT1_5),
            6 => Some(ADC14SHT1_A::ADC14SHT1_6),
            7 => Some(ADC14SHT1_A::ADC14SHT1_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14SHT1_0`"]
    #[inline(always)]
    pub fn is_adc14sht1_0(&self) -> bool {
        *self == ADC14SHT1_A::ADC14SHT1_0
    }
    #[doc = "Checks if the value of the field is `ADC14SHT1_1`"]
    #[inline(always)]
    pub fn is_adc14sht1_1(&self) -> bool {
        *self == ADC14SHT1_A::ADC14SHT1_1
    }
    #[doc = "Checks if the value of the field is `ADC14SHT1_2`"]
    #[inline(always)]
    pub fn is_adc14sht1_2(&self) -> bool {
        *self == ADC14SHT1_A::ADC14SHT1_2
    }
    #[doc = "Checks if the value of the field is `ADC14SHT1_3`"]
    #[inline(always)]
    pub fn is_adc14sht1_3(&self) -> bool {
        *self == ADC14SHT1_A::ADC14SHT1_3
    }
    #[doc = "Checks if the value of the field is `ADC14SHT1_4`"]
    #[inline(always)]
    pub fn is_adc14sht1_4(&self) -> bool {
        *self == ADC14SHT1_A::ADC14SHT1_4
    }
    #[doc = "Checks if the value of the field is `ADC14SHT1_5`"]
    #[inline(always)]
    pub fn is_adc14sht1_5(&self) -> bool {
        *self == ADC14SHT1_A::ADC14SHT1_5
    }
    #[doc = "Checks if the value of the field is `ADC14SHT1_6`"]
    #[inline(always)]
    pub fn is_adc14sht1_6(&self) -> bool {
        *self == ADC14SHT1_A::ADC14SHT1_6
    }
    #[doc = "Checks if the value of the field is `ADC14SHT1_7`"]
    #[inline(always)]
    pub fn is_adc14sht1_7(&self) -> bool {
        *self == ADC14SHT1_A::ADC14SHT1_7
    }
}
#[doc = "Field `ADC14SHT1` writer - ADC14 sample-and-hold time"]
pub type ADC14SHT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC14CTL0_SPEC, u8, ADC14SHT1_A, 4, O>;
impl<'a, const O: u8> ADC14SHT1_W<'a, O> {
    #[doc = "4"]
    #[inline(always)]
    pub fn adc14sht1_0(self) -> &'a mut W {
        self.variant(ADC14SHT1_A::ADC14SHT1_0)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn adc14sht1_1(self) -> &'a mut W {
        self.variant(ADC14SHT1_A::ADC14SHT1_1)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn adc14sht1_2(self) -> &'a mut W {
        self.variant(ADC14SHT1_A::ADC14SHT1_2)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn adc14sht1_3(self) -> &'a mut W {
        self.variant(ADC14SHT1_A::ADC14SHT1_3)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn adc14sht1_4(self) -> &'a mut W {
        self.variant(ADC14SHT1_A::ADC14SHT1_4)
    }
    #[doc = "96"]
    #[inline(always)]
    pub fn adc14sht1_5(self) -> &'a mut W {
        self.variant(ADC14SHT1_A::ADC14SHT1_5)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn adc14sht1_6(self) -> &'a mut W {
        self.variant(ADC14SHT1_A::ADC14SHT1_6)
    }
    #[doc = "192"]
    #[inline(always)]
    pub fn adc14sht1_7(self) -> &'a mut W {
        self.variant(ADC14SHT1_A::ADC14SHT1_7)
    }
}
#[doc = "ADC14 busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14BUSY_A {
    #[doc = "0: No operation is active"]
    ADC14BUSY_0 = 0,
    #[doc = "1: A sequence, sample, or conversion is active"]
    ADC14BUSY_1 = 1,
}
impl From<ADC14BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14BUSY` reader - ADC14 busy"]
pub type ADC14BUSY_R = crate::BitReader<ADC14BUSY_A>;
impl ADC14BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14BUSY_A {
        match self.bits {
            false => ADC14BUSY_A::ADC14BUSY_0,
            true => ADC14BUSY_A::ADC14BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14BUSY_0`"]
    #[inline(always)]
    pub fn is_adc14busy_0(&self) -> bool {
        *self == ADC14BUSY_A::ADC14BUSY_0
    }
    #[doc = "Checks if the value of the field is `ADC14BUSY_1`"]
    #[inline(always)]
    pub fn is_adc14busy_1(&self) -> bool {
        *self == ADC14BUSY_A::ADC14BUSY_1
    }
}
#[doc = "ADC14 conversion sequence mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC14CONSEQ_A {
    #[doc = "0: Single-channel, single-conversion"]
    ADC14CONSEQ_0 = 0,
    #[doc = "1: Sequence-of-channels"]
    ADC14CONSEQ_1 = 1,
    #[doc = "2: Repeat-single-channel"]
    ADC14CONSEQ_2 = 2,
    #[doc = "3: Repeat-sequence-of-channels"]
    ADC14CONSEQ_3 = 3,
}
impl From<ADC14CONSEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14CONSEQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC14CONSEQ` reader - ADC14 conversion sequence mode select"]
pub type ADC14CONSEQ_R = crate::FieldReader<u8, ADC14CONSEQ_A>;
impl ADC14CONSEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14CONSEQ_A {
        match self.bits {
            0 => ADC14CONSEQ_A::ADC14CONSEQ_0,
            1 => ADC14CONSEQ_A::ADC14CONSEQ_1,
            2 => ADC14CONSEQ_A::ADC14CONSEQ_2,
            3 => ADC14CONSEQ_A::ADC14CONSEQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC14CONSEQ_0`"]
    #[inline(always)]
    pub fn is_adc14conseq_0(&self) -> bool {
        *self == ADC14CONSEQ_A::ADC14CONSEQ_0
    }
    #[doc = "Checks if the value of the field is `ADC14CONSEQ_1`"]
    #[inline(always)]
    pub fn is_adc14conseq_1(&self) -> bool {
        *self == ADC14CONSEQ_A::ADC14CONSEQ_1
    }
    #[doc = "Checks if the value of the field is `ADC14CONSEQ_2`"]
    #[inline(always)]
    pub fn is_adc14conseq_2(&self) -> bool {
        *self == ADC14CONSEQ_A::ADC14CONSEQ_2
    }
    #[doc = "Checks if the value of the field is `ADC14CONSEQ_3`"]
    #[inline(always)]
    pub fn is_adc14conseq_3(&self) -> bool {
        *self == ADC14CONSEQ_A::ADC14CONSEQ_3
    }
}
#[doc = "Field `ADC14CONSEQ` writer - ADC14 conversion sequence mode select"]
pub type ADC14CONSEQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC14CTL0_SPEC, u8, ADC14CONSEQ_A, 2, O>;
impl<'a, const O: u8> ADC14CONSEQ_W<'a, O> {
    #[doc = "Single-channel, single-conversion"]
    #[inline(always)]
    pub fn adc14conseq_0(self) -> &'a mut W {
        self.variant(ADC14CONSEQ_A::ADC14CONSEQ_0)
    }
    #[doc = "Sequence-of-channels"]
    #[inline(always)]
    pub fn adc14conseq_1(self) -> &'a mut W {
        self.variant(ADC14CONSEQ_A::ADC14CONSEQ_1)
    }
    #[doc = "Repeat-single-channel"]
    #[inline(always)]
    pub fn adc14conseq_2(self) -> &'a mut W {
        self.variant(ADC14CONSEQ_A::ADC14CONSEQ_2)
    }
    #[doc = "Repeat-sequence-of-channels"]
    #[inline(always)]
    pub fn adc14conseq_3(self) -> &'a mut W {
        self.variant(ADC14CONSEQ_A::ADC14CONSEQ_3)
    }
}
#[doc = "ADC14 clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC14SSEL_A {
    #[doc = "0: MODCLK"]
    ADC14SSEL_0 = 0,
    #[doc = "1: SYSCLK"]
    ADC14SSEL_1 = 1,
    #[doc = "2: ACLK"]
    ADC14SSEL_2 = 2,
    #[doc = "3: MCLK"]
    ADC14SSEL_3 = 3,
    #[doc = "4: SMCLK"]
    ADC14SSEL_4 = 4,
    #[doc = "5: HSMCLK"]
    ADC14SSEL_5 = 5,
}
impl From<ADC14SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14SSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC14SSEL` reader - ADC14 clock source select"]
pub type ADC14SSEL_R = crate::FieldReader<u8, ADC14SSEL_A>;
impl ADC14SSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC14SSEL_A> {
        match self.bits {
            0 => Some(ADC14SSEL_A::ADC14SSEL_0),
            1 => Some(ADC14SSEL_A::ADC14SSEL_1),
            2 => Some(ADC14SSEL_A::ADC14SSEL_2),
            3 => Some(ADC14SSEL_A::ADC14SSEL_3),
            4 => Some(ADC14SSEL_A::ADC14SSEL_4),
            5 => Some(ADC14SSEL_A::ADC14SSEL_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14SSEL_0`"]
    #[inline(always)]
    pub fn is_adc14ssel_0(&self) -> bool {
        *self == ADC14SSEL_A::ADC14SSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC14SSEL_1`"]
    #[inline(always)]
    pub fn is_adc14ssel_1(&self) -> bool {
        *self == ADC14SSEL_A::ADC14SSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC14SSEL_2`"]
    #[inline(always)]
    pub fn is_adc14ssel_2(&self) -> bool {
        *self == ADC14SSEL_A::ADC14SSEL_2
    }
    #[doc = "Checks if the value of the field is `ADC14SSEL_3`"]
    #[inline(always)]
    pub fn is_adc14ssel_3(&self) -> bool {
        *self == ADC14SSEL_A::ADC14SSEL_3
    }
    #[doc = "Checks if the value of the field is `ADC14SSEL_4`"]
    #[inline(always)]
    pub fn is_adc14ssel_4(&self) -> bool {
        *self == ADC14SSEL_A::ADC14SSEL_4
    }
    #[doc = "Checks if the value of the field is `ADC14SSEL_5`"]
    #[inline(always)]
    pub fn is_adc14ssel_5(&self) -> bool {
        *self == ADC14SSEL_A::ADC14SSEL_5
    }
}
#[doc = "Field `ADC14SSEL` writer - ADC14 clock source select"]
pub type ADC14SSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC14CTL0_SPEC, u8, ADC14SSEL_A, 3, O>;
impl<'a, const O: u8> ADC14SSEL_W<'a, O> {
    #[doc = "MODCLK"]
    #[inline(always)]
    pub fn adc14ssel_0(self) -> &'a mut W {
        self.variant(ADC14SSEL_A::ADC14SSEL_0)
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn adc14ssel_1(self) -> &'a mut W {
        self.variant(ADC14SSEL_A::ADC14SSEL_1)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn adc14ssel_2(self) -> &'a mut W {
        self.variant(ADC14SSEL_A::ADC14SSEL_2)
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn adc14ssel_3(self) -> &'a mut W {
        self.variant(ADC14SSEL_A::ADC14SSEL_3)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn adc14ssel_4(self) -> &'a mut W {
        self.variant(ADC14SSEL_A::ADC14SSEL_4)
    }
    #[doc = "HSMCLK"]
    #[inline(always)]
    pub fn adc14ssel_5(self) -> &'a mut W {
        self.variant(ADC14SSEL_A::ADC14SSEL_5)
    }
}
#[doc = "ADC14 clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC14DIV_A {
    #[doc = "0: /1"]
    ADC14DIV_0 = 0,
    #[doc = "1: /2"]
    ADC14DIV_1 = 1,
    #[doc = "2: /3"]
    ADC14DIV_2 = 2,
    #[doc = "3: /4"]
    ADC14DIV_3 = 3,
    #[doc = "4: /5"]
    ADC14DIV_4 = 4,
    #[doc = "5: /6"]
    ADC14DIV_5 = 5,
    #[doc = "6: /7"]
    ADC14DIV_6 = 6,
    #[doc = "7: /8"]
    ADC14DIV_7 = 7,
}
impl From<ADC14DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC14DIV` reader - ADC14 clock divider"]
pub type ADC14DIV_R = crate::FieldReader<u8, ADC14DIV_A>;
impl ADC14DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14DIV_A {
        match self.bits {
            0 => ADC14DIV_A::ADC14DIV_0,
            1 => ADC14DIV_A::ADC14DIV_1,
            2 => ADC14DIV_A::ADC14DIV_2,
            3 => ADC14DIV_A::ADC14DIV_3,
            4 => ADC14DIV_A::ADC14DIV_4,
            5 => ADC14DIV_A::ADC14DIV_5,
            6 => ADC14DIV_A::ADC14DIV_6,
            7 => ADC14DIV_A::ADC14DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC14DIV_0`"]
    #[inline(always)]
    pub fn is_adc14div_0(&self) -> bool {
        *self == ADC14DIV_A::ADC14DIV_0
    }
    #[doc = "Checks if the value of the field is `ADC14DIV_1`"]
    #[inline(always)]
    pub fn is_adc14div_1(&self) -> bool {
        *self == ADC14DIV_A::ADC14DIV_1
    }
    #[doc = "Checks if the value of the field is `ADC14DIV_2`"]
    #[inline(always)]
    pub fn is_adc14div_2(&self) -> bool {
        *self == ADC14DIV_A::ADC14DIV_2
    }
    #[doc = "Checks if the value of the field is `ADC14DIV_3`"]
    #[inline(always)]
    pub fn is_adc14div_3(&self) -> bool {
        *self == ADC14DIV_A::ADC14DIV_3
    }
    #[doc = "Checks if the value of the field is `ADC14DIV_4`"]
    #[inline(always)]
    pub fn is_adc14div_4(&self) -> bool {
        *self == ADC14DIV_A::ADC14DIV_4
    }
    #[doc = "Checks if the value of the field is `ADC14DIV_5`"]
    #[inline(always)]
    pub fn is_adc14div_5(&self) -> bool {
        *self == ADC14DIV_A::ADC14DIV_5
    }
    #[doc = "Checks if the value of the field is `ADC14DIV_6`"]
    #[inline(always)]
    pub fn is_adc14div_6(&self) -> bool {
        *self == ADC14DIV_A::ADC14DIV_6
    }
    #[doc = "Checks if the value of the field is `ADC14DIV_7`"]
    #[inline(always)]
    pub fn is_adc14div_7(&self) -> bool {
        *self == ADC14DIV_A::ADC14DIV_7
    }
}
#[doc = "Field `ADC14DIV` writer - ADC14 clock divider"]
pub type ADC14DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC14CTL0_SPEC, u8, ADC14DIV_A, 3, O>;
impl<'a, const O: u8> ADC14DIV_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn adc14div_0(self) -> &'a mut W {
        self.variant(ADC14DIV_A::ADC14DIV_0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn adc14div_1(self) -> &'a mut W {
        self.variant(ADC14DIV_A::ADC14DIV_1)
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn adc14div_2(self) -> &'a mut W {
        self.variant(ADC14DIV_A::ADC14DIV_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn adc14div_3(self) -> &'a mut W {
        self.variant(ADC14DIV_A::ADC14DIV_3)
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn adc14div_4(self) -> &'a mut W {
        self.variant(ADC14DIV_A::ADC14DIV_4)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn adc14div_5(self) -> &'a mut W {
        self.variant(ADC14DIV_A::ADC14DIV_5)
    }
    #[doc = "/7"]
    #[inline(always)]
    pub fn adc14div_6(self) -> &'a mut W {
        self.variant(ADC14DIV_A::ADC14DIV_6)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn adc14div_7(self) -> &'a mut W {
        self.variant(ADC14DIV_A::ADC14DIV_7)
    }
}
#[doc = "ADC14 invert signal sample-and-hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14ISSH_A {
    #[doc = "0: The sample-input signal is not inverted"]
    ADC14ISSH_0 = 0,
    #[doc = "1: The sample-input signal is inverted"]
    ADC14ISSH_1 = 1,
}
impl From<ADC14ISSH_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14ISSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14ISSH` reader - ADC14 invert signal sample-and-hold"]
pub type ADC14ISSH_R = crate::BitReader<ADC14ISSH_A>;
impl ADC14ISSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14ISSH_A {
        match self.bits {
            false => ADC14ISSH_A::ADC14ISSH_0,
            true => ADC14ISSH_A::ADC14ISSH_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14ISSH_0`"]
    #[inline(always)]
    pub fn is_adc14issh_0(&self) -> bool {
        *self == ADC14ISSH_A::ADC14ISSH_0
    }
    #[doc = "Checks if the value of the field is `ADC14ISSH_1`"]
    #[inline(always)]
    pub fn is_adc14issh_1(&self) -> bool {
        *self == ADC14ISSH_A::ADC14ISSH_1
    }
}
#[doc = "Field `ADC14ISSH` writer - ADC14 invert signal sample-and-hold"]
pub type ADC14ISSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14CTL0_SPEC, ADC14ISSH_A, O>;
impl<'a, const O: u8> ADC14ISSH_W<'a, O> {
    #[doc = "The sample-input signal is not inverted"]
    #[inline(always)]
    pub fn adc14issh_0(self) -> &'a mut W {
        self.variant(ADC14ISSH_A::ADC14ISSH_0)
    }
    #[doc = "The sample-input signal is inverted"]
    #[inline(always)]
    pub fn adc14issh_1(self) -> &'a mut W {
        self.variant(ADC14ISSH_A::ADC14ISSH_1)
    }
}
#[doc = "ADC14 sample-and-hold pulse-mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14SHP_A {
    #[doc = "0: SAMPCON signal is sourced from the sample-input signal"]
    ADC14SHP_0 = 0,
    #[doc = "1: SAMPCON signal is sourced from the sampling timer"]
    ADC14SHP_1 = 1,
}
impl From<ADC14SHP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14SHP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14SHP` reader - ADC14 sample-and-hold pulse-mode select"]
pub type ADC14SHP_R = crate::BitReader<ADC14SHP_A>;
impl ADC14SHP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14SHP_A {
        match self.bits {
            false => ADC14SHP_A::ADC14SHP_0,
            true => ADC14SHP_A::ADC14SHP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14SHP_0`"]
    #[inline(always)]
    pub fn is_adc14shp_0(&self) -> bool {
        *self == ADC14SHP_A::ADC14SHP_0
    }
    #[doc = "Checks if the value of the field is `ADC14SHP_1`"]
    #[inline(always)]
    pub fn is_adc14shp_1(&self) -> bool {
        *self == ADC14SHP_A::ADC14SHP_1
    }
}
#[doc = "Field `ADC14SHP` writer - ADC14 sample-and-hold pulse-mode select"]
pub type ADC14SHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14CTL0_SPEC, ADC14SHP_A, O>;
impl<'a, const O: u8> ADC14SHP_W<'a, O> {
    #[doc = "SAMPCON signal is sourced from the sample-input signal"]
    #[inline(always)]
    pub fn adc14shp_0(self) -> &'a mut W {
        self.variant(ADC14SHP_A::ADC14SHP_0)
    }
    #[doc = "SAMPCON signal is sourced from the sampling timer"]
    #[inline(always)]
    pub fn adc14shp_1(self) -> &'a mut W {
        self.variant(ADC14SHP_A::ADC14SHP_1)
    }
}
#[doc = "ADC14 sample-and-hold source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC14SHS_A {
    #[doc = "0: ADC14SC bit"]
    ADC14SHS_0 = 0,
    #[doc = "1: See device-specific data sheet for source"]
    ADC14SHS_1 = 1,
    #[doc = "2: See device-specific data sheet for source"]
    ADC14SHS_2 = 2,
    #[doc = "3: See device-specific data sheet for source"]
    ADC14SHS_3 = 3,
    #[doc = "4: See device-specific data sheet for source"]
    ADC14SHS_4 = 4,
    #[doc = "5: See device-specific data sheet for source"]
    ADC14SHS_5 = 5,
    #[doc = "6: See device-specific data sheet for source"]
    ADC14SHS_6 = 6,
    #[doc = "7: See device-specific data sheet for source"]
    ADC14SHS_7 = 7,
}
impl From<ADC14SHS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14SHS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC14SHS` reader - ADC14 sample-and-hold source select"]
pub type ADC14SHS_R = crate::FieldReader<u8, ADC14SHS_A>;
impl ADC14SHS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14SHS_A {
        match self.bits {
            0 => ADC14SHS_A::ADC14SHS_0,
            1 => ADC14SHS_A::ADC14SHS_1,
            2 => ADC14SHS_A::ADC14SHS_2,
            3 => ADC14SHS_A::ADC14SHS_3,
            4 => ADC14SHS_A::ADC14SHS_4,
            5 => ADC14SHS_A::ADC14SHS_5,
            6 => ADC14SHS_A::ADC14SHS_6,
            7 => ADC14SHS_A::ADC14SHS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC14SHS_0`"]
    #[inline(always)]
    pub fn is_adc14shs_0(&self) -> bool {
        *self == ADC14SHS_A::ADC14SHS_0
    }
    #[doc = "Checks if the value of the field is `ADC14SHS_1`"]
    #[inline(always)]
    pub fn is_adc14shs_1(&self) -> bool {
        *self == ADC14SHS_A::ADC14SHS_1
    }
    #[doc = "Checks if the value of the field is `ADC14SHS_2`"]
    #[inline(always)]
    pub fn is_adc14shs_2(&self) -> bool {
        *self == ADC14SHS_A::ADC14SHS_2
    }
    #[doc = "Checks if the value of the field is `ADC14SHS_3`"]
    #[inline(always)]
    pub fn is_adc14shs_3(&self) -> bool {
        *self == ADC14SHS_A::ADC14SHS_3
    }
    #[doc = "Checks if the value of the field is `ADC14SHS_4`"]
    #[inline(always)]
    pub fn is_adc14shs_4(&self) -> bool {
        *self == ADC14SHS_A::ADC14SHS_4
    }
    #[doc = "Checks if the value of the field is `ADC14SHS_5`"]
    #[inline(always)]
    pub fn is_adc14shs_5(&self) -> bool {
        *self == ADC14SHS_A::ADC14SHS_5
    }
    #[doc = "Checks if the value of the field is `ADC14SHS_6`"]
    #[inline(always)]
    pub fn is_adc14shs_6(&self) -> bool {
        *self == ADC14SHS_A::ADC14SHS_6
    }
    #[doc = "Checks if the value of the field is `ADC14SHS_7`"]
    #[inline(always)]
    pub fn is_adc14shs_7(&self) -> bool {
        *self == ADC14SHS_A::ADC14SHS_7
    }
}
#[doc = "Field `ADC14SHS` writer - ADC14 sample-and-hold source select"]
pub type ADC14SHS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC14CTL0_SPEC, u8, ADC14SHS_A, 3, O>;
impl<'a, const O: u8> ADC14SHS_W<'a, O> {
    #[doc = "ADC14SC bit"]
    #[inline(always)]
    pub fn adc14shs_0(self) -> &'a mut W {
        self.variant(ADC14SHS_A::ADC14SHS_0)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_1(self) -> &'a mut W {
        self.variant(ADC14SHS_A::ADC14SHS_1)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_2(self) -> &'a mut W {
        self.variant(ADC14SHS_A::ADC14SHS_2)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_3(self) -> &'a mut W {
        self.variant(ADC14SHS_A::ADC14SHS_3)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_4(self) -> &'a mut W {
        self.variant(ADC14SHS_A::ADC14SHS_4)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_5(self) -> &'a mut W {
        self.variant(ADC14SHS_A::ADC14SHS_5)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_6(self) -> &'a mut W {
        self.variant(ADC14SHS_A::ADC14SHS_6)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_7(self) -> &'a mut W {
        self.variant(ADC14SHS_A::ADC14SHS_7)
    }
}
#[doc = "ADC14 predivider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC14PDIV_A {
    #[doc = "0: Predivide by 1"]
    ADC14PDIV_0 = 0,
    #[doc = "1: Predivide by 4"]
    ADC14PDIV_1 = 1,
    #[doc = "2: Predivide by 32"]
    ADC14PDIV_2 = 2,
    #[doc = "3: Predivide by 64"]
    ADC14PDIV_3 = 3,
}
impl From<ADC14PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14PDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC14PDIV` reader - ADC14 predivider"]
pub type ADC14PDIV_R = crate::FieldReader<u8, ADC14PDIV_A>;
impl ADC14PDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14PDIV_A {
        match self.bits {
            0 => ADC14PDIV_A::ADC14PDIV_0,
            1 => ADC14PDIV_A::ADC14PDIV_1,
            2 => ADC14PDIV_A::ADC14PDIV_2,
            3 => ADC14PDIV_A::ADC14PDIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC14PDIV_0`"]
    #[inline(always)]
    pub fn is_adc14pdiv_0(&self) -> bool {
        *self == ADC14PDIV_A::ADC14PDIV_0
    }
    #[doc = "Checks if the value of the field is `ADC14PDIV_1`"]
    #[inline(always)]
    pub fn is_adc14pdiv_1(&self) -> bool {
        *self == ADC14PDIV_A::ADC14PDIV_1
    }
    #[doc = "Checks if the value of the field is `ADC14PDIV_2`"]
    #[inline(always)]
    pub fn is_adc14pdiv_2(&self) -> bool {
        *self == ADC14PDIV_A::ADC14PDIV_2
    }
    #[doc = "Checks if the value of the field is `ADC14PDIV_3`"]
    #[inline(always)]
    pub fn is_adc14pdiv_3(&self) -> bool {
        *self == ADC14PDIV_A::ADC14PDIV_3
    }
}
#[doc = "Field `ADC14PDIV` writer - ADC14 predivider"]
pub type ADC14PDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC14CTL0_SPEC, u8, ADC14PDIV_A, 2, O>;
impl<'a, const O: u8> ADC14PDIV_W<'a, O> {
    #[doc = "Predivide by 1"]
    #[inline(always)]
    pub fn adc14pdiv_0(self) -> &'a mut W {
        self.variant(ADC14PDIV_A::ADC14PDIV_0)
    }
    #[doc = "Predivide by 4"]
    #[inline(always)]
    pub fn adc14pdiv_1(self) -> &'a mut W {
        self.variant(ADC14PDIV_A::ADC14PDIV_1)
    }
    #[doc = "Predivide by 32"]
    #[inline(always)]
    pub fn adc14pdiv_2(self) -> &'a mut W {
        self.variant(ADC14PDIV_A::ADC14PDIV_2)
    }
    #[doc = "Predivide by 64"]
    #[inline(always)]
    pub fn adc14pdiv_3(self) -> &'a mut W {
        self.variant(ADC14PDIV_A::ADC14PDIV_3)
    }
}
impl R {
    #[doc = "Bit 0 - ADC14 start conversion"]
    #[inline(always)]
    pub fn adc14sc(&self) -> ADC14SC_R {
        ADC14SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC14 enable conversion"]
    #[inline(always)]
    pub fn adc14enc(&self) -> ADC14ENC_R {
        ADC14ENC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC14 on"]
    #[inline(always)]
    pub fn adc14on(&self) -> ADC14ON_R {
        ADC14ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC14 multiple sample and conversion"]
    #[inline(always)]
    pub fn adc14msc(&self) -> ADC14MSC_R {
        ADC14MSC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADC14 sample-and-hold time"]
    #[inline(always)]
    pub fn adc14sht0(&self) -> ADC14SHT0_R {
        ADC14SHT0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ADC14 sample-and-hold time"]
    #[inline(always)]
    pub fn adc14sht1(&self) -> ADC14SHT1_R {
        ADC14SHT1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - ADC14 busy"]
    #[inline(always)]
    pub fn adc14busy(&self) -> ADC14BUSY_R {
        ADC14BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - ADC14 conversion sequence mode select"]
    #[inline(always)]
    pub fn adc14conseq(&self) -> ADC14CONSEQ_R {
        ADC14CONSEQ_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - ADC14 clock source select"]
    #[inline(always)]
    pub fn adc14ssel(&self) -> ADC14SSEL_R {
        ADC14SSEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - ADC14 clock divider"]
    #[inline(always)]
    pub fn adc14div(&self) -> ADC14DIV_R {
        ADC14DIV_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - ADC14 invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adc14issh(&self) -> ADC14ISSH_R {
        ADC14ISSH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ADC14 sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adc14shp(&self) -> ADC14SHP_R {
        ADC14SHP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - ADC14 sample-and-hold source select"]
    #[inline(always)]
    pub fn adc14shs(&self) -> ADC14SHS_R {
        ADC14SHS_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - ADC14 predivider"]
    #[inline(always)]
    pub fn adc14pdiv(&self) -> ADC14PDIV_R {
        ADC14PDIV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC14 start conversion"]
    #[inline(always)]
    pub fn adc14sc(&mut self) -> ADC14SC_W<0> {
        ADC14SC_W::new(self)
    }
    #[doc = "Bit 1 - ADC14 enable conversion"]
    #[inline(always)]
    pub fn adc14enc(&mut self) -> ADC14ENC_W<1> {
        ADC14ENC_W::new(self)
    }
    #[doc = "Bit 4 - ADC14 on"]
    #[inline(always)]
    pub fn adc14on(&mut self) -> ADC14ON_W<4> {
        ADC14ON_W::new(self)
    }
    #[doc = "Bit 7 - ADC14 multiple sample and conversion"]
    #[inline(always)]
    pub fn adc14msc(&mut self) -> ADC14MSC_W<7> {
        ADC14MSC_W::new(self)
    }
    #[doc = "Bits 8:11 - ADC14 sample-and-hold time"]
    #[inline(always)]
    pub fn adc14sht0(&mut self) -> ADC14SHT0_W<8> {
        ADC14SHT0_W::new(self)
    }
    #[doc = "Bits 12:15 - ADC14 sample-and-hold time"]
    #[inline(always)]
    pub fn adc14sht1(&mut self) -> ADC14SHT1_W<12> {
        ADC14SHT1_W::new(self)
    }
    #[doc = "Bits 17:18 - ADC14 conversion sequence mode select"]
    #[inline(always)]
    pub fn adc14conseq(&mut self) -> ADC14CONSEQ_W<17> {
        ADC14CONSEQ_W::new(self)
    }
    #[doc = "Bits 19:21 - ADC14 clock source select"]
    #[inline(always)]
    pub fn adc14ssel(&mut self) -> ADC14SSEL_W<19> {
        ADC14SSEL_W::new(self)
    }
    #[doc = "Bits 22:24 - ADC14 clock divider"]
    #[inline(always)]
    pub fn adc14div(&mut self) -> ADC14DIV_W<22> {
        ADC14DIV_W::new(self)
    }
    #[doc = "Bit 25 - ADC14 invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adc14issh(&mut self) -> ADC14ISSH_W<25> {
        ADC14ISSH_W::new(self)
    }
    #[doc = "Bit 26 - ADC14 sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adc14shp(&mut self) -> ADC14SHP_W<26> {
        ADC14SHP_W::new(self)
    }
    #[doc = "Bits 27:29 - ADC14 sample-and-hold source select"]
    #[inline(always)]
    pub fn adc14shs(&mut self) -> ADC14SHS_W<27> {
        ADC14SHS_W::new(self)
    }
    #[doc = "Bits 30:31 - ADC14 predivider"]
    #[inline(always)]
    pub fn adc14pdiv(&mut self) -> ADC14PDIV_W<30> {
        ADC14PDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14ctl0](index.html) module"]
pub struct ADC14CTL0_SPEC;
impl crate::RegisterSpec for ADC14CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14ctl0::R](R) reader structure"]
impl crate::Readable for ADC14CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc14ctl0::W](W) writer structure"]
impl crate::Writable for ADC14CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC14CTL0 to value 0"]
impl crate::Resettable for ADC14CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
