#[doc = "Register `CExCTL3` reader"]
pub struct R(crate::R<CEXCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEXCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEXCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEXCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CExCTL3` writer"]
pub struct W(crate::W<CEXCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEXCTL3_SPEC>;
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
impl From<crate::W<CEXCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEXCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD0_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD0_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD0_1 = 1,
}
impl From<CEPD0_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD0` reader - Port disable"]
pub type CEPD0_R = crate::BitReader<CEPD0_A>;
impl CEPD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD0_A {
        match self.bits {
            false => CEPD0_A::CEPD0_0,
            true => CEPD0_A::CEPD0_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD0_0`"]
    #[inline(always)]
    pub fn is_cepd0_0(&self) -> bool {
        *self == CEPD0_A::CEPD0_0
    }
    #[doc = "Checks if the value of the field is `CEPD0_1`"]
    #[inline(always)]
    pub fn is_cepd0_1(&self) -> bool {
        *self == CEPD0_A::CEPD0_1
    }
}
#[doc = "Field `CEPD0` writer - Port disable"]
pub type CEPD0_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD0_A, O>;
impl<'a, const O: u8> CEPD0_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd0_0(self) -> &'a mut W {
        self.variant(CEPD0_A::CEPD0_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd0_1(self) -> &'a mut W {
        self.variant(CEPD0_A::CEPD0_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD1_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD1_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD1_1 = 1,
}
impl From<CEPD1_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD1` reader - Port disable"]
pub type CEPD1_R = crate::BitReader<CEPD1_A>;
impl CEPD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD1_A {
        match self.bits {
            false => CEPD1_A::CEPD1_0,
            true => CEPD1_A::CEPD1_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD1_0`"]
    #[inline(always)]
    pub fn is_cepd1_0(&self) -> bool {
        *self == CEPD1_A::CEPD1_0
    }
    #[doc = "Checks if the value of the field is `CEPD1_1`"]
    #[inline(always)]
    pub fn is_cepd1_1(&self) -> bool {
        *self == CEPD1_A::CEPD1_1
    }
}
#[doc = "Field `CEPD1` writer - Port disable"]
pub type CEPD1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD1_A, O>;
impl<'a, const O: u8> CEPD1_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd1_0(self) -> &'a mut W {
        self.variant(CEPD1_A::CEPD1_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd1_1(self) -> &'a mut W {
        self.variant(CEPD1_A::CEPD1_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD2_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD2_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD2_1 = 1,
}
impl From<CEPD2_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD2` reader - Port disable"]
pub type CEPD2_R = crate::BitReader<CEPD2_A>;
impl CEPD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD2_A {
        match self.bits {
            false => CEPD2_A::CEPD2_0,
            true => CEPD2_A::CEPD2_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD2_0`"]
    #[inline(always)]
    pub fn is_cepd2_0(&self) -> bool {
        *self == CEPD2_A::CEPD2_0
    }
    #[doc = "Checks if the value of the field is `CEPD2_1`"]
    #[inline(always)]
    pub fn is_cepd2_1(&self) -> bool {
        *self == CEPD2_A::CEPD2_1
    }
}
#[doc = "Field `CEPD2` writer - Port disable"]
pub type CEPD2_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD2_A, O>;
impl<'a, const O: u8> CEPD2_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd2_0(self) -> &'a mut W {
        self.variant(CEPD2_A::CEPD2_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd2_1(self) -> &'a mut W {
        self.variant(CEPD2_A::CEPD2_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD3_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD3_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD3_1 = 1,
}
impl From<CEPD3_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD3` reader - Port disable"]
pub type CEPD3_R = crate::BitReader<CEPD3_A>;
impl CEPD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD3_A {
        match self.bits {
            false => CEPD3_A::CEPD3_0,
            true => CEPD3_A::CEPD3_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD3_0`"]
    #[inline(always)]
    pub fn is_cepd3_0(&self) -> bool {
        *self == CEPD3_A::CEPD3_0
    }
    #[doc = "Checks if the value of the field is `CEPD3_1`"]
    #[inline(always)]
    pub fn is_cepd3_1(&self) -> bool {
        *self == CEPD3_A::CEPD3_1
    }
}
#[doc = "Field `CEPD3` writer - Port disable"]
pub type CEPD3_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD3_A, O>;
impl<'a, const O: u8> CEPD3_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd3_0(self) -> &'a mut W {
        self.variant(CEPD3_A::CEPD3_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd3_1(self) -> &'a mut W {
        self.variant(CEPD3_A::CEPD3_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD4_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD4_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD4_1 = 1,
}
impl From<CEPD4_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD4` reader - Port disable"]
pub type CEPD4_R = crate::BitReader<CEPD4_A>;
impl CEPD4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD4_A {
        match self.bits {
            false => CEPD4_A::CEPD4_0,
            true => CEPD4_A::CEPD4_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD4_0`"]
    #[inline(always)]
    pub fn is_cepd4_0(&self) -> bool {
        *self == CEPD4_A::CEPD4_0
    }
    #[doc = "Checks if the value of the field is `CEPD4_1`"]
    #[inline(always)]
    pub fn is_cepd4_1(&self) -> bool {
        *self == CEPD4_A::CEPD4_1
    }
}
#[doc = "Field `CEPD4` writer - Port disable"]
pub type CEPD4_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD4_A, O>;
impl<'a, const O: u8> CEPD4_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd4_0(self) -> &'a mut W {
        self.variant(CEPD4_A::CEPD4_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd4_1(self) -> &'a mut W {
        self.variant(CEPD4_A::CEPD4_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD5_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD5_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD5_1 = 1,
}
impl From<CEPD5_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD5` reader - Port disable"]
pub type CEPD5_R = crate::BitReader<CEPD5_A>;
impl CEPD5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD5_A {
        match self.bits {
            false => CEPD5_A::CEPD5_0,
            true => CEPD5_A::CEPD5_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD5_0`"]
    #[inline(always)]
    pub fn is_cepd5_0(&self) -> bool {
        *self == CEPD5_A::CEPD5_0
    }
    #[doc = "Checks if the value of the field is `CEPD5_1`"]
    #[inline(always)]
    pub fn is_cepd5_1(&self) -> bool {
        *self == CEPD5_A::CEPD5_1
    }
}
#[doc = "Field `CEPD5` writer - Port disable"]
pub type CEPD5_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD5_A, O>;
impl<'a, const O: u8> CEPD5_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd5_0(self) -> &'a mut W {
        self.variant(CEPD5_A::CEPD5_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd5_1(self) -> &'a mut W {
        self.variant(CEPD5_A::CEPD5_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD6_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD6_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD6_1 = 1,
}
impl From<CEPD6_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD6` reader - Port disable"]
pub type CEPD6_R = crate::BitReader<CEPD6_A>;
impl CEPD6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD6_A {
        match self.bits {
            false => CEPD6_A::CEPD6_0,
            true => CEPD6_A::CEPD6_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD6_0`"]
    #[inline(always)]
    pub fn is_cepd6_0(&self) -> bool {
        *self == CEPD6_A::CEPD6_0
    }
    #[doc = "Checks if the value of the field is `CEPD6_1`"]
    #[inline(always)]
    pub fn is_cepd6_1(&self) -> bool {
        *self == CEPD6_A::CEPD6_1
    }
}
#[doc = "Field `CEPD6` writer - Port disable"]
pub type CEPD6_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD6_A, O>;
impl<'a, const O: u8> CEPD6_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd6_0(self) -> &'a mut W {
        self.variant(CEPD6_A::CEPD6_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd6_1(self) -> &'a mut W {
        self.variant(CEPD6_A::CEPD6_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD7_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD7_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD7_1 = 1,
}
impl From<CEPD7_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD7` reader - Port disable"]
pub type CEPD7_R = crate::BitReader<CEPD7_A>;
impl CEPD7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD7_A {
        match self.bits {
            false => CEPD7_A::CEPD7_0,
            true => CEPD7_A::CEPD7_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD7_0`"]
    #[inline(always)]
    pub fn is_cepd7_0(&self) -> bool {
        *self == CEPD7_A::CEPD7_0
    }
    #[doc = "Checks if the value of the field is `CEPD7_1`"]
    #[inline(always)]
    pub fn is_cepd7_1(&self) -> bool {
        *self == CEPD7_A::CEPD7_1
    }
}
#[doc = "Field `CEPD7` writer - Port disable"]
pub type CEPD7_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD7_A, O>;
impl<'a, const O: u8> CEPD7_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd7_0(self) -> &'a mut W {
        self.variant(CEPD7_A::CEPD7_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd7_1(self) -> &'a mut W {
        self.variant(CEPD7_A::CEPD7_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD8_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD8_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD8_1 = 1,
}
impl From<CEPD8_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD8` reader - Port disable"]
pub type CEPD8_R = crate::BitReader<CEPD8_A>;
impl CEPD8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD8_A {
        match self.bits {
            false => CEPD8_A::CEPD8_0,
            true => CEPD8_A::CEPD8_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD8_0`"]
    #[inline(always)]
    pub fn is_cepd8_0(&self) -> bool {
        *self == CEPD8_A::CEPD8_0
    }
    #[doc = "Checks if the value of the field is `CEPD8_1`"]
    #[inline(always)]
    pub fn is_cepd8_1(&self) -> bool {
        *self == CEPD8_A::CEPD8_1
    }
}
#[doc = "Field `CEPD8` writer - Port disable"]
pub type CEPD8_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD8_A, O>;
impl<'a, const O: u8> CEPD8_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd8_0(self) -> &'a mut W {
        self.variant(CEPD8_A::CEPD8_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd8_1(self) -> &'a mut W {
        self.variant(CEPD8_A::CEPD8_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD9_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD9_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD9_1 = 1,
}
impl From<CEPD9_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD9` reader - Port disable"]
pub type CEPD9_R = crate::BitReader<CEPD9_A>;
impl CEPD9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD9_A {
        match self.bits {
            false => CEPD9_A::CEPD9_0,
            true => CEPD9_A::CEPD9_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD9_0`"]
    #[inline(always)]
    pub fn is_cepd9_0(&self) -> bool {
        *self == CEPD9_A::CEPD9_0
    }
    #[doc = "Checks if the value of the field is `CEPD9_1`"]
    #[inline(always)]
    pub fn is_cepd9_1(&self) -> bool {
        *self == CEPD9_A::CEPD9_1
    }
}
#[doc = "Field `CEPD9` writer - Port disable"]
pub type CEPD9_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD9_A, O>;
impl<'a, const O: u8> CEPD9_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd9_0(self) -> &'a mut W {
        self.variant(CEPD9_A::CEPD9_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd9_1(self) -> &'a mut W {
        self.variant(CEPD9_A::CEPD9_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD10_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD10_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD10_1 = 1,
}
impl From<CEPD10_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD10` reader - Port disable"]
pub type CEPD10_R = crate::BitReader<CEPD10_A>;
impl CEPD10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD10_A {
        match self.bits {
            false => CEPD10_A::CEPD10_0,
            true => CEPD10_A::CEPD10_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD10_0`"]
    #[inline(always)]
    pub fn is_cepd10_0(&self) -> bool {
        *self == CEPD10_A::CEPD10_0
    }
    #[doc = "Checks if the value of the field is `CEPD10_1`"]
    #[inline(always)]
    pub fn is_cepd10_1(&self) -> bool {
        *self == CEPD10_A::CEPD10_1
    }
}
#[doc = "Field `CEPD10` writer - Port disable"]
pub type CEPD10_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD10_A, O>;
impl<'a, const O: u8> CEPD10_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd10_0(self) -> &'a mut W {
        self.variant(CEPD10_A::CEPD10_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd10_1(self) -> &'a mut W {
        self.variant(CEPD10_A::CEPD10_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD11_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD11_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD11_1 = 1,
}
impl From<CEPD11_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD11` reader - Port disable"]
pub type CEPD11_R = crate::BitReader<CEPD11_A>;
impl CEPD11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD11_A {
        match self.bits {
            false => CEPD11_A::CEPD11_0,
            true => CEPD11_A::CEPD11_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD11_0`"]
    #[inline(always)]
    pub fn is_cepd11_0(&self) -> bool {
        *self == CEPD11_A::CEPD11_0
    }
    #[doc = "Checks if the value of the field is `CEPD11_1`"]
    #[inline(always)]
    pub fn is_cepd11_1(&self) -> bool {
        *self == CEPD11_A::CEPD11_1
    }
}
#[doc = "Field `CEPD11` writer - Port disable"]
pub type CEPD11_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD11_A, O>;
impl<'a, const O: u8> CEPD11_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd11_0(self) -> &'a mut W {
        self.variant(CEPD11_A::CEPD11_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd11_1(self) -> &'a mut W {
        self.variant(CEPD11_A::CEPD11_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD12_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD12_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD12_1 = 1,
}
impl From<CEPD12_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD12` reader - Port disable"]
pub type CEPD12_R = crate::BitReader<CEPD12_A>;
impl CEPD12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD12_A {
        match self.bits {
            false => CEPD12_A::CEPD12_0,
            true => CEPD12_A::CEPD12_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD12_0`"]
    #[inline(always)]
    pub fn is_cepd12_0(&self) -> bool {
        *self == CEPD12_A::CEPD12_0
    }
    #[doc = "Checks if the value of the field is `CEPD12_1`"]
    #[inline(always)]
    pub fn is_cepd12_1(&self) -> bool {
        *self == CEPD12_A::CEPD12_1
    }
}
#[doc = "Field `CEPD12` writer - Port disable"]
pub type CEPD12_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD12_A, O>;
impl<'a, const O: u8> CEPD12_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd12_0(self) -> &'a mut W {
        self.variant(CEPD12_A::CEPD12_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd12_1(self) -> &'a mut W {
        self.variant(CEPD12_A::CEPD12_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD13_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD13_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD13_1 = 1,
}
impl From<CEPD13_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD13` reader - Port disable"]
pub type CEPD13_R = crate::BitReader<CEPD13_A>;
impl CEPD13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD13_A {
        match self.bits {
            false => CEPD13_A::CEPD13_0,
            true => CEPD13_A::CEPD13_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD13_0`"]
    #[inline(always)]
    pub fn is_cepd13_0(&self) -> bool {
        *self == CEPD13_A::CEPD13_0
    }
    #[doc = "Checks if the value of the field is `CEPD13_1`"]
    #[inline(always)]
    pub fn is_cepd13_1(&self) -> bool {
        *self == CEPD13_A::CEPD13_1
    }
}
#[doc = "Field `CEPD13` writer - Port disable"]
pub type CEPD13_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD13_A, O>;
impl<'a, const O: u8> CEPD13_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd13_0(self) -> &'a mut W {
        self.variant(CEPD13_A::CEPD13_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd13_1(self) -> &'a mut W {
        self.variant(CEPD13_A::CEPD13_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD14_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD14_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD14_1 = 1,
}
impl From<CEPD14_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD14` reader - Port disable"]
pub type CEPD14_R = crate::BitReader<CEPD14_A>;
impl CEPD14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD14_A {
        match self.bits {
            false => CEPD14_A::CEPD14_0,
            true => CEPD14_A::CEPD14_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD14_0`"]
    #[inline(always)]
    pub fn is_cepd14_0(&self) -> bool {
        *self == CEPD14_A::CEPD14_0
    }
    #[doc = "Checks if the value of the field is `CEPD14_1`"]
    #[inline(always)]
    pub fn is_cepd14_1(&self) -> bool {
        *self == CEPD14_A::CEPD14_1
    }
}
#[doc = "Field `CEPD14` writer - Port disable"]
pub type CEPD14_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD14_A, O>;
impl<'a, const O: u8> CEPD14_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd14_0(self) -> &'a mut W {
        self.variant(CEPD14_A::CEPD14_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd14_1(self) -> &'a mut W {
        self.variant(CEPD14_A::CEPD14_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD15_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD15_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD15_1 = 1,
}
impl From<CEPD15_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD15` reader - Port disable"]
pub type CEPD15_R = crate::BitReader<CEPD15_A>;
impl CEPD15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD15_A {
        match self.bits {
            false => CEPD15_A::CEPD15_0,
            true => CEPD15_A::CEPD15_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD15_0`"]
    #[inline(always)]
    pub fn is_cepd15_0(&self) -> bool {
        *self == CEPD15_A::CEPD15_0
    }
    #[doc = "Checks if the value of the field is `CEPD15_1`"]
    #[inline(always)]
    pub fn is_cepd15_1(&self) -> bool {
        *self == CEPD15_A::CEPD15_1
    }
}
#[doc = "Field `CEPD15` writer - Port disable"]
pub type CEPD15_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL3_SPEC, CEPD15_A, O>;
impl<'a, const O: u8> CEPD15_W<'a, O> {
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd15_0(self) -> &'a mut W {
        self.variant(CEPD15_A::CEPD15_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd15_1(self) -> &'a mut W {
        self.variant(CEPD15_A::CEPD15_1)
    }
}
impl R {
    #[doc = "Bit 0 - Port disable"]
    #[inline(always)]
    pub fn cepd0(&self) -> CEPD0_R {
        CEPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port disable"]
    #[inline(always)]
    pub fn cepd1(&self) -> CEPD1_R {
        CEPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port disable"]
    #[inline(always)]
    pub fn cepd2(&self) -> CEPD2_R {
        CEPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port disable"]
    #[inline(always)]
    pub fn cepd3(&self) -> CEPD3_R {
        CEPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port disable"]
    #[inline(always)]
    pub fn cepd4(&self) -> CEPD4_R {
        CEPD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port disable"]
    #[inline(always)]
    pub fn cepd5(&self) -> CEPD5_R {
        CEPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port disable"]
    #[inline(always)]
    pub fn cepd6(&self) -> CEPD6_R {
        CEPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port disable"]
    #[inline(always)]
    pub fn cepd7(&self) -> CEPD7_R {
        CEPD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port disable"]
    #[inline(always)]
    pub fn cepd8(&self) -> CEPD8_R {
        CEPD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port disable"]
    #[inline(always)]
    pub fn cepd9(&self) -> CEPD9_R {
        CEPD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port disable"]
    #[inline(always)]
    pub fn cepd10(&self) -> CEPD10_R {
        CEPD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port disable"]
    #[inline(always)]
    pub fn cepd11(&self) -> CEPD11_R {
        CEPD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port disable"]
    #[inline(always)]
    pub fn cepd12(&self) -> CEPD12_R {
        CEPD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port disable"]
    #[inline(always)]
    pub fn cepd13(&self) -> CEPD13_R {
        CEPD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port disable"]
    #[inline(always)]
    pub fn cepd14(&self) -> CEPD14_R {
        CEPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port disable"]
    #[inline(always)]
    pub fn cepd15(&self) -> CEPD15_R {
        CEPD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port disable"]
    #[inline(always)]
    pub fn cepd0(&mut self) -> CEPD0_W<0> {
        CEPD0_W::new(self)
    }
    #[doc = "Bit 1 - Port disable"]
    #[inline(always)]
    pub fn cepd1(&mut self) -> CEPD1_W<1> {
        CEPD1_W::new(self)
    }
    #[doc = "Bit 2 - Port disable"]
    #[inline(always)]
    pub fn cepd2(&mut self) -> CEPD2_W<2> {
        CEPD2_W::new(self)
    }
    #[doc = "Bit 3 - Port disable"]
    #[inline(always)]
    pub fn cepd3(&mut self) -> CEPD3_W<3> {
        CEPD3_W::new(self)
    }
    #[doc = "Bit 4 - Port disable"]
    #[inline(always)]
    pub fn cepd4(&mut self) -> CEPD4_W<4> {
        CEPD4_W::new(self)
    }
    #[doc = "Bit 5 - Port disable"]
    #[inline(always)]
    pub fn cepd5(&mut self) -> CEPD5_W<5> {
        CEPD5_W::new(self)
    }
    #[doc = "Bit 6 - Port disable"]
    #[inline(always)]
    pub fn cepd6(&mut self) -> CEPD6_W<6> {
        CEPD6_W::new(self)
    }
    #[doc = "Bit 7 - Port disable"]
    #[inline(always)]
    pub fn cepd7(&mut self) -> CEPD7_W<7> {
        CEPD7_W::new(self)
    }
    #[doc = "Bit 8 - Port disable"]
    #[inline(always)]
    pub fn cepd8(&mut self) -> CEPD8_W<8> {
        CEPD8_W::new(self)
    }
    #[doc = "Bit 9 - Port disable"]
    #[inline(always)]
    pub fn cepd9(&mut self) -> CEPD9_W<9> {
        CEPD9_W::new(self)
    }
    #[doc = "Bit 10 - Port disable"]
    #[inline(always)]
    pub fn cepd10(&mut self) -> CEPD10_W<10> {
        CEPD10_W::new(self)
    }
    #[doc = "Bit 11 - Port disable"]
    #[inline(always)]
    pub fn cepd11(&mut self) -> CEPD11_W<11> {
        CEPD11_W::new(self)
    }
    #[doc = "Bit 12 - Port disable"]
    #[inline(always)]
    pub fn cepd12(&mut self) -> CEPD12_W<12> {
        CEPD12_W::new(self)
    }
    #[doc = "Bit 13 - Port disable"]
    #[inline(always)]
    pub fn cepd13(&mut self) -> CEPD13_W<13> {
        CEPD13_W::new(self)
    }
    #[doc = "Bit 14 - Port disable"]
    #[inline(always)]
    pub fn cepd14(&mut self) -> CEPD14_W<14> {
        CEPD14_W::new(self)
    }
    #[doc = "Bit 15 - Port disable"]
    #[inline(always)]
    pub fn cepd15(&mut self) -> CEPD15_W<15> {
        CEPD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_ctl3](index.html) module"]
pub struct CEXCTL3_SPEC;
impl crate::RegisterSpec for CEXCTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cex_ctl3::R](R) reader structure"]
impl crate::Readable for CEXCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cex_ctl3::W](W) writer structure"]
impl crate::Writable for CEXCTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CExCTL3 to value 0"]
impl crate::Resettable for CEXCTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
