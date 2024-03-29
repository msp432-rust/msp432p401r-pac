#[doc = "Register `CExINT` reader"]
pub struct R(crate::R<CEXINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEXINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEXINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEXINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CExINT` writer"]
pub struct W(crate::W<CEXINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEXINT_SPEC>;
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
impl From<crate::W<CEXINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEXINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparator output interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEIFG_A {
    #[doc = "0: No interrupt pending"]
    CEIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    CEIFG_1 = 1,
}
impl From<CEIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CEIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIFG` reader - Comparator output interrupt flag"]
pub type CEIFG_R = crate::BitReader<CEIFG_A>;
impl CEIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIFG_A {
        match self.bits {
            false => CEIFG_A::CEIFG_0,
            true => CEIFG_A::CEIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEIFG_0`"]
    #[inline(always)]
    pub fn is_ceifg_0(&self) -> bool {
        *self == CEIFG_A::CEIFG_0
    }
    #[doc = "Checks if the value of the field is `CEIFG_1`"]
    #[inline(always)]
    pub fn is_ceifg_1(&self) -> bool {
        *self == CEIFG_A::CEIFG_1
    }
}
#[doc = "Field `CEIFG` writer - Comparator output interrupt flag"]
pub type CEIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXINT_SPEC, CEIFG_A, O>;
impl<'a, const O: u8> CEIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ceifg_0(self) -> &'a mut W {
        self.variant(CEIFG_A::CEIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ceifg_1(self) -> &'a mut W {
        self.variant(CEIFG_A::CEIFG_1)
    }
}
#[doc = "Comparator output inverted interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEIIFG_A {
    #[doc = "0: No interrupt pending"]
    CEIIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    CEIIFG_1 = 1,
}
impl From<CEIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CEIIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIIFG` reader - Comparator output inverted interrupt flag"]
pub type CEIIFG_R = crate::BitReader<CEIIFG_A>;
impl CEIIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIIFG_A {
        match self.bits {
            false => CEIIFG_A::CEIIFG_0,
            true => CEIIFG_A::CEIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEIIFG_0`"]
    #[inline(always)]
    pub fn is_ceiifg_0(&self) -> bool {
        *self == CEIIFG_A::CEIIFG_0
    }
    #[doc = "Checks if the value of the field is `CEIIFG_1`"]
    #[inline(always)]
    pub fn is_ceiifg_1(&self) -> bool {
        *self == CEIIFG_A::CEIIFG_1
    }
}
#[doc = "Field `CEIIFG` writer - Comparator output inverted interrupt flag"]
pub type CEIIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXINT_SPEC, CEIIFG_A, O>;
impl<'a, const O: u8> CEIIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ceiifg_0(self) -> &'a mut W {
        self.variant(CEIIFG_A::CEIIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ceiifg_1(self) -> &'a mut W {
        self.variant(CEIIFG_A::CEIIFG_1)
    }
}
#[doc = "Comparator ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CERDYIFG_A {
    #[doc = "0: No interrupt pending"]
    CERDYIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    CERDYIFG_1 = 1,
}
impl From<CERDYIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CERDYIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERDYIFG` reader - Comparator ready interrupt flag"]
pub type CERDYIFG_R = crate::BitReader<CERDYIFG_A>;
impl CERDYIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERDYIFG_A {
        match self.bits {
            false => CERDYIFG_A::CERDYIFG_0,
            true => CERDYIFG_A::CERDYIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CERDYIFG_0`"]
    #[inline(always)]
    pub fn is_cerdyifg_0(&self) -> bool {
        *self == CERDYIFG_A::CERDYIFG_0
    }
    #[doc = "Checks if the value of the field is `CERDYIFG_1`"]
    #[inline(always)]
    pub fn is_cerdyifg_1(&self) -> bool {
        *self == CERDYIFG_A::CERDYIFG_1
    }
}
#[doc = "Field `CERDYIFG` writer - Comparator ready interrupt flag"]
pub type CERDYIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXINT_SPEC, CERDYIFG_A, O>;
impl<'a, const O: u8> CERDYIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn cerdyifg_0(self) -> &'a mut W {
        self.variant(CERDYIFG_A::CERDYIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn cerdyifg_1(self) -> &'a mut W {
        self.variant(CERDYIFG_A::CERDYIFG_1)
    }
}
#[doc = "Comparator output interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEIE_A {
    #[doc = "0: Interrupt disabled"]
    CEIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    CEIE_1 = 1,
}
impl From<CEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIE` reader - Comparator output interrupt enable"]
pub type CEIE_R = crate::BitReader<CEIE_A>;
impl CEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIE_A {
        match self.bits {
            false => CEIE_A::CEIE_0,
            true => CEIE_A::CEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEIE_0`"]
    #[inline(always)]
    pub fn is_ceie_0(&self) -> bool {
        *self == CEIE_A::CEIE_0
    }
    #[doc = "Checks if the value of the field is `CEIE_1`"]
    #[inline(always)]
    pub fn is_ceie_1(&self) -> bool {
        *self == CEIE_A::CEIE_1
    }
}
#[doc = "Field `CEIE` writer - Comparator output interrupt enable"]
pub type CEIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXINT_SPEC, CEIE_A, O>;
impl<'a, const O: u8> CEIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ceie_0(self) -> &'a mut W {
        self.variant(CEIE_A::CEIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ceie_1(self) -> &'a mut W {
        self.variant(CEIE_A::CEIE_1)
    }
}
#[doc = "Comparator output interrupt enable inverted polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEIIE_A {
    #[doc = "0: Interrupt disabled"]
    CEIIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    CEIIE_1 = 1,
}
impl From<CEIIE_A> for bool {
    #[inline(always)]
    fn from(variant: CEIIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIIE` reader - Comparator output interrupt enable inverted polarity"]
pub type CEIIE_R = crate::BitReader<CEIIE_A>;
impl CEIIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIIE_A {
        match self.bits {
            false => CEIIE_A::CEIIE_0,
            true => CEIIE_A::CEIIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEIIE_0`"]
    #[inline(always)]
    pub fn is_ceiie_0(&self) -> bool {
        *self == CEIIE_A::CEIIE_0
    }
    #[doc = "Checks if the value of the field is `CEIIE_1`"]
    #[inline(always)]
    pub fn is_ceiie_1(&self) -> bool {
        *self == CEIIE_A::CEIIE_1
    }
}
#[doc = "Field `CEIIE` writer - Comparator output interrupt enable inverted polarity"]
pub type CEIIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXINT_SPEC, CEIIE_A, O>;
impl<'a, const O: u8> CEIIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ceiie_0(self) -> &'a mut W {
        self.variant(CEIIE_A::CEIIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ceiie_1(self) -> &'a mut W {
        self.variant(CEIIE_A::CEIIE_1)
    }
}
#[doc = "Comparator ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CERDYIE_A {
    #[doc = "0: Interrupt disabled"]
    CERDYIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    CERDYIE_1 = 1,
}
impl From<CERDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: CERDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERDYIE` reader - Comparator ready interrupt enable"]
pub type CERDYIE_R = crate::BitReader<CERDYIE_A>;
impl CERDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERDYIE_A {
        match self.bits {
            false => CERDYIE_A::CERDYIE_0,
            true => CERDYIE_A::CERDYIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CERDYIE_0`"]
    #[inline(always)]
    pub fn is_cerdyie_0(&self) -> bool {
        *self == CERDYIE_A::CERDYIE_0
    }
    #[doc = "Checks if the value of the field is `CERDYIE_1`"]
    #[inline(always)]
    pub fn is_cerdyie_1(&self) -> bool {
        *self == CERDYIE_A::CERDYIE_1
    }
}
#[doc = "Field `CERDYIE` writer - Comparator ready interrupt enable"]
pub type CERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXINT_SPEC, CERDYIE_A, O>;
impl<'a, const O: u8> CERDYIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn cerdyie_0(self) -> &'a mut W {
        self.variant(CERDYIE_A::CERDYIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn cerdyie_1(self) -> &'a mut W {
        self.variant(CERDYIE_A::CERDYIE_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn ceifg(&self) -> CEIFG_R {
        CEIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn ceiifg(&self) -> CEIIFG_R {
        CEIIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator ready interrupt flag"]
    #[inline(always)]
    pub fn cerdyifg(&self) -> CERDYIFG_R {
        CERDYIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparator output interrupt enable"]
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator output interrupt enable inverted polarity"]
    #[inline(always)]
    pub fn ceiie(&self) -> CEIIE_R {
        CEIIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparator ready interrupt enable"]
    #[inline(always)]
    pub fn cerdyie(&self) -> CERDYIE_R {
        CERDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn ceifg(&mut self) -> CEIFG_W<0> {
        CEIFG_W::new(self)
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn ceiifg(&mut self) -> CEIIFG_W<1> {
        CEIIFG_W::new(self)
    }
    #[doc = "Bit 4 - Comparator ready interrupt flag"]
    #[inline(always)]
    pub fn cerdyifg(&mut self) -> CERDYIFG_W<4> {
        CERDYIFG_W::new(self)
    }
    #[doc = "Bit 8 - Comparator output interrupt enable"]
    #[inline(always)]
    pub fn ceie(&mut self) -> CEIE_W<8> {
        CEIE_W::new(self)
    }
    #[doc = "Bit 9 - Comparator output interrupt enable inverted polarity"]
    #[inline(always)]
    pub fn ceiie(&mut self) -> CEIIE_W<9> {
        CEIIE_W::new(self)
    }
    #[doc = "Bit 12 - Comparator ready interrupt enable"]
    #[inline(always)]
    pub fn cerdyie(&mut self) -> CERDYIE_W<12> {
        CERDYIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_int](index.html) module"]
pub struct CEXINT_SPEC;
impl crate::RegisterSpec for CEXINT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cex_int::R](R) reader structure"]
impl crate::Readable for CEXINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cex_int::W](W) writer structure"]
impl crate::Writable for CEXINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CExINT to value 0"]
impl crate::Resettable for CEXINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
