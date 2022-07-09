#[doc = "Register `T32CONTROL2` reader"]
pub struct R(crate::R<T32CONTROL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T32CONTROL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T32CONTROL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T32CONTROL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T32CONTROL2` writer"]
pub struct W(crate::W<T32CONTROL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T32CONTROL2_SPEC>;
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
impl From<crate::W<T32CONTROL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T32CONTROL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects one-shot or wrapping counter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOT_A {
    #[doc = "0: wrapping mode"]
    ONESHOT_0 = 0,
    #[doc = "1: one-shot mode"]
    ONESHOT_1 = 1,
}
impl From<ONESHOT_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONESHOT` reader - Selects one-shot or wrapping counter mode"]
pub type ONESHOT_R = crate::BitReader<ONESHOT_A>;
impl ONESHOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOT_A {
        match self.bits {
            false => ONESHOT_A::ONESHOT_0,
            true => ONESHOT_A::ONESHOT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONESHOT_0`"]
    #[inline(always)]
    pub fn is_oneshot_0(&self) -> bool {
        *self == ONESHOT_A::ONESHOT_0
    }
    #[doc = "Checks if the value of the field is `ONESHOT_1`"]
    #[inline(always)]
    pub fn is_oneshot_1(&self) -> bool {
        *self == ONESHOT_A::ONESHOT_1
    }
}
#[doc = "Field `ONESHOT` writer - Selects one-shot or wrapping counter mode"]
pub type ONESHOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, T32CONTROL2_SPEC, ONESHOT_A, O>;
impl<'a, const O: u8> ONESHOT_W<'a, O> {
    #[doc = "wrapping mode"]
    #[inline(always)]
    pub fn oneshot_0(self) -> &'a mut W {
        self.variant(ONESHOT_A::ONESHOT_0)
    }
    #[doc = "one-shot mode"]
    #[inline(always)]
    pub fn oneshot_1(self) -> &'a mut W {
        self.variant(ONESHOT_A::ONESHOT_1)
    }
}
#[doc = "Selects 16 or 32 bit counter operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZE_A {
    #[doc = "0: 16-bit counter"]
    SIZE_0 = 0,
    #[doc = "1: 32-bit counter"]
    SIZE_1 = 1,
}
impl From<SIZE_A> for bool {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIZE` reader - Selects 16 or 32 bit counter operation"]
pub type SIZE_R = crate::BitReader<SIZE_A>;
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIZE_A {
        match self.bits {
            false => SIZE_A::SIZE_0,
            true => SIZE_A::SIZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SIZE_0`"]
    #[inline(always)]
    pub fn is_size_0(&self) -> bool {
        *self == SIZE_A::SIZE_0
    }
    #[doc = "Checks if the value of the field is `SIZE_1`"]
    #[inline(always)]
    pub fn is_size_1(&self) -> bool {
        *self == SIZE_A::SIZE_1
    }
}
#[doc = "Field `SIZE` writer - Selects 16 or 32 bit counter operation"]
pub type SIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, T32CONTROL2_SPEC, SIZE_A, O>;
impl<'a, const O: u8> SIZE_W<'a, O> {
    #[doc = "16-bit counter"]
    #[inline(always)]
    pub fn size_0(self) -> &'a mut W {
        self.variant(SIZE_A::SIZE_0)
    }
    #[doc = "32-bit counter"]
    #[inline(always)]
    pub fn size_1(self) -> &'a mut W {
        self.variant(SIZE_A::SIZE_1)
    }
}
#[doc = "Prescale bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: 0 stages of prescale, clock is divided by 1"]
    PRESCALE_0 = 0,
    #[doc = "1: 4 stages of prescale, clock is divided by 16"]
    PRESCALE_1 = 1,
    #[doc = "2: 8 stages of prescale, clock is divided by 256"]
    PRESCALE_2 = 2,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESCALE` reader - Prescale bits"]
pub type PRESCALE_R = crate::FieldReader<u8, PRESCALE_A>;
impl PRESCALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCALE_A> {
        match self.bits {
            0 => Some(PRESCALE_A::PRESCALE_0),
            1 => Some(PRESCALE_A::PRESCALE_1),
            2 => Some(PRESCALE_A::PRESCALE_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALE_0`"]
    #[inline(always)]
    pub fn is_prescale_0(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_0
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1`"]
    #[inline(always)]
    pub fn is_prescale_1(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `PRESCALE_2`"]
    #[inline(always)]
    pub fn is_prescale_2(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_2
    }
}
#[doc = "Field `PRESCALE` writer - Prescale bits"]
pub type PRESCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, T32CONTROL2_SPEC, u8, PRESCALE_A, 2, O>;
impl<'a, const O: u8> PRESCALE_W<'a, O> {
    #[doc = "0 stages of prescale, clock is divided by 1"]
    #[inline(always)]
    pub fn prescale_0(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_0)
    }
    #[doc = "4 stages of prescale, clock is divided by 16"]
    #[inline(always)]
    pub fn prescale_1(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_1)
    }
    #[doc = "8 stages of prescale, clock is divided by 256"]
    #[inline(always)]
    pub fn prescale_2(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_2)
    }
}
#[doc = "Interrupt enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_A {
    #[doc = "0: Timer interrupt disabled"]
    IE_0 = 0,
    #[doc = "1: Timer interrupt enabled"]
    IE_1 = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt enable bit"]
pub type IE_R = crate::BitReader<IE_A>;
impl IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::IE_0,
            true => IE_A::IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IE_0`"]
    #[inline(always)]
    pub fn is_ie_0(&self) -> bool {
        *self == IE_A::IE_0
    }
    #[doc = "Checks if the value of the field is `IE_1`"]
    #[inline(always)]
    pub fn is_ie_1(&self) -> bool {
        *self == IE_A::IE_1
    }
}
#[doc = "Field `IE` writer - Interrupt enable bit"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, T32CONTROL2_SPEC, IE_A, O>;
impl<'a, const O: u8> IE_W<'a, O> {
    #[doc = "Timer interrupt disabled"]
    #[inline(always)]
    pub fn ie_0(self) -> &'a mut W {
        self.variant(IE_A::IE_0)
    }
    #[doc = "Timer interrupt enabled"]
    #[inline(always)]
    pub fn ie_1(self) -> &'a mut W {
        self.variant(IE_A::IE_1)
    }
}
#[doc = "Mode bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Timer is in free-running mode"]
    MODE_0 = 0,
    #[doc = "1: Timer is in periodic mode"]
    MODE_1 = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Mode bit"]
pub type MODE_R = crate::BitReader<MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::MODE_0,
            true => MODE_A::MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == MODE_A::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == MODE_A::MODE_1
    }
}
#[doc = "Field `MODE` writer - Mode bit"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, T32CONTROL2_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Timer is in free-running mode"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODE_A::MODE_0)
    }
    #[doc = "Timer is in periodic mode"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODE_A::MODE_1)
    }
}
#[doc = "Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Timer disabled"]
    ENABLE_0 = 0,
    #[doc = "1: Timer enabled"]
    ENABLE_1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable bit"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::ENABLE_0,
            true => ENABLE_A::ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == ENABLE_A::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == ENABLE_A::ENABLE_1
    }
}
#[doc = "Field `ENABLE` writer - Enable bit"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, T32CONTROL2_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Timer disabled"]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_0)
    }
    #[doc = "Timer enabled"]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_1)
    }
}
impl R {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects 16 or 32 bit counter operation"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Prescale bits"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable bit"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mode bit"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable bit"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> ONESHOT_W<0> {
        ONESHOT_W::new(self)
    }
    #[doc = "Bit 1 - Selects 16 or 32 bit counter operation"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W<1> {
        SIZE_W::new(self)
    }
    #[doc = "Bits 2:3 - Prescale bits"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W<2> {
        PRESCALE_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable bit"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<5> {
        IE_W::new(self)
    }
    #[doc = "Bit 6 - Mode bit"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<6> {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - Enable bit"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<7> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer 2 Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32control2](index.html) module"]
pub struct T32CONTROL2_SPEC;
impl crate::RegisterSpec for T32CONTROL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t32control2::R](R) reader structure"]
impl crate::Readable for T32CONTROL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t32control2::W](W) writer structure"]
impl crate::Writable for T32CONTROL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T32CONTROL2 to value 0x20"]
impl crate::Resettable for T32CONTROL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
