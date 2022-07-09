#[doc = "Register `ADC14CLRIFGR0` writer"]
pub struct W(crate::W<ADC14CLRIFGR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC14CLRIFGR0_SPEC>;
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
impl From<crate::W<ADC14CLRIFGR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC14CLRIFGR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "clear ADC14IFG0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG0_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG0_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG0_1 = 1,
}
impl From<CLRADC14IFG0_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG0` writer - clear ADC14IFG0"]
pub type CLRADC14IFG0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG0_AW, O>;
impl<'a, const O: u8> CLRADC14IFG0_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg0_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG0_AW::CLRADC14IFG0_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg0_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG0_AW::CLRADC14IFG0_1)
    }
}
#[doc = "clear ADC14IFG1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG1_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG1_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG1_1 = 1,
}
impl From<CLRADC14IFG1_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG1` writer - clear ADC14IFG1"]
pub type CLRADC14IFG1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG1_AW, O>;
impl<'a, const O: u8> CLRADC14IFG1_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg1_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG1_AW::CLRADC14IFG1_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg1_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG1_AW::CLRADC14IFG1_1)
    }
}
#[doc = "clear ADC14IFG2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG2_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG2_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG2_1 = 1,
}
impl From<CLRADC14IFG2_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG2` writer - clear ADC14IFG2"]
pub type CLRADC14IFG2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG2_AW, O>;
impl<'a, const O: u8> CLRADC14IFG2_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg2_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG2_AW::CLRADC14IFG2_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg2_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG2_AW::CLRADC14IFG2_1)
    }
}
#[doc = "clear ADC14IFG3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG3_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG3_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG3_1 = 1,
}
impl From<CLRADC14IFG3_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG3` writer - clear ADC14IFG3"]
pub type CLRADC14IFG3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG3_AW, O>;
impl<'a, const O: u8> CLRADC14IFG3_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg3_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG3_AW::CLRADC14IFG3_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg3_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG3_AW::CLRADC14IFG3_1)
    }
}
#[doc = "clear ADC14IFG4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG4_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG4_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG4_1 = 1,
}
impl From<CLRADC14IFG4_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG4` writer - clear ADC14IFG4"]
pub type CLRADC14IFG4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG4_AW, O>;
impl<'a, const O: u8> CLRADC14IFG4_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg4_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG4_AW::CLRADC14IFG4_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg4_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG4_AW::CLRADC14IFG4_1)
    }
}
#[doc = "clear ADC14IFG5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG5_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG5_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG5_1 = 1,
}
impl From<CLRADC14IFG5_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG5` writer - clear ADC14IFG5"]
pub type CLRADC14IFG5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG5_AW, O>;
impl<'a, const O: u8> CLRADC14IFG5_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg5_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG5_AW::CLRADC14IFG5_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg5_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG5_AW::CLRADC14IFG5_1)
    }
}
#[doc = "clear ADC14IFG6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG6_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG6_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG6_1 = 1,
}
impl From<CLRADC14IFG6_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG6` writer - clear ADC14IFG6"]
pub type CLRADC14IFG6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG6_AW, O>;
impl<'a, const O: u8> CLRADC14IFG6_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg6_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG6_AW::CLRADC14IFG6_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg6_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG6_AW::CLRADC14IFG6_1)
    }
}
#[doc = "clear ADC14IFG7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG7_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG7_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG7_1 = 1,
}
impl From<CLRADC14IFG7_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG7` writer - clear ADC14IFG7"]
pub type CLRADC14IFG7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG7_AW, O>;
impl<'a, const O: u8> CLRADC14IFG7_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg7_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG7_AW::CLRADC14IFG7_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg7_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG7_AW::CLRADC14IFG7_1)
    }
}
#[doc = "clear ADC14IFG8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG8_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG8_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG8_1 = 1,
}
impl From<CLRADC14IFG8_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG8` writer - clear ADC14IFG8"]
pub type CLRADC14IFG8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG8_AW, O>;
impl<'a, const O: u8> CLRADC14IFG8_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg8_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG8_AW::CLRADC14IFG8_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg8_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG8_AW::CLRADC14IFG8_1)
    }
}
#[doc = "clear ADC14IFG9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG9_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG9_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG9_1 = 1,
}
impl From<CLRADC14IFG9_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG9` writer - clear ADC14IFG9"]
pub type CLRADC14IFG9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG9_AW, O>;
impl<'a, const O: u8> CLRADC14IFG9_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg9_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG9_AW::CLRADC14IFG9_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg9_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG9_AW::CLRADC14IFG9_1)
    }
}
#[doc = "clear ADC14IFG10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG10_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG10_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG10_1 = 1,
}
impl From<CLRADC14IFG10_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG10` writer - clear ADC14IFG10"]
pub type CLRADC14IFG10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG10_AW, O>;
impl<'a, const O: u8> CLRADC14IFG10_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg10_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG10_AW::CLRADC14IFG10_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg10_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG10_AW::CLRADC14IFG10_1)
    }
}
#[doc = "clear ADC14IFG11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG11_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG11_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG11_1 = 1,
}
impl From<CLRADC14IFG11_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG11` writer - clear ADC14IFG11"]
pub type CLRADC14IFG11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG11_AW, O>;
impl<'a, const O: u8> CLRADC14IFG11_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg11_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG11_AW::CLRADC14IFG11_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg11_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG11_AW::CLRADC14IFG11_1)
    }
}
#[doc = "clear ADC14IFG12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG12_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG12_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG12_1 = 1,
}
impl From<CLRADC14IFG12_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG12` writer - clear ADC14IFG12"]
pub type CLRADC14IFG12_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG12_AW, O>;
impl<'a, const O: u8> CLRADC14IFG12_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg12_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG12_AW::CLRADC14IFG12_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg12_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG12_AW::CLRADC14IFG12_1)
    }
}
#[doc = "clear ADC14IFG13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG13_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG13_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG13_1 = 1,
}
impl From<CLRADC14IFG13_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG13` writer - clear ADC14IFG13"]
pub type CLRADC14IFG13_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG13_AW, O>;
impl<'a, const O: u8> CLRADC14IFG13_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg13_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG13_AW::CLRADC14IFG13_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg13_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG13_AW::CLRADC14IFG13_1)
    }
}
#[doc = "clear ADC14IFG14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG14_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG14_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG14_1 = 1,
}
impl From<CLRADC14IFG14_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG14` writer - clear ADC14IFG14"]
pub type CLRADC14IFG14_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG14_AW, O>;
impl<'a, const O: u8> CLRADC14IFG14_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg14_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG14_AW::CLRADC14IFG14_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg14_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG14_AW::CLRADC14IFG14_1)
    }
}
#[doc = "clear ADC14IFG15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG15_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG15_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG15_1 = 1,
}
impl From<CLRADC14IFG15_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG15` writer - clear ADC14IFG15"]
pub type CLRADC14IFG15_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG15_AW, O>;
impl<'a, const O: u8> CLRADC14IFG15_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg15_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG15_AW::CLRADC14IFG15_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg15_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG15_AW::CLRADC14IFG15_1)
    }
}
#[doc = "clear ADC14IFG16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG16_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG16_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG16_1 = 1,
}
impl From<CLRADC14IFG16_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG16` writer - clear ADC14IFG16"]
pub type CLRADC14IFG16_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG16_AW, O>;
impl<'a, const O: u8> CLRADC14IFG16_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg16_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG16_AW::CLRADC14IFG16_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg16_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG16_AW::CLRADC14IFG16_1)
    }
}
#[doc = "clear ADC14IFG17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG17_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG17_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG17_1 = 1,
}
impl From<CLRADC14IFG17_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG17` writer - clear ADC14IFG17"]
pub type CLRADC14IFG17_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG17_AW, O>;
impl<'a, const O: u8> CLRADC14IFG17_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg17_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG17_AW::CLRADC14IFG17_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg17_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG17_AW::CLRADC14IFG17_1)
    }
}
#[doc = "clear ADC14IFG18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG18_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG18_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG18_1 = 1,
}
impl From<CLRADC14IFG18_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG18` writer - clear ADC14IFG18"]
pub type CLRADC14IFG18_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG18_AW, O>;
impl<'a, const O: u8> CLRADC14IFG18_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg18_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG18_AW::CLRADC14IFG18_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg18_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG18_AW::CLRADC14IFG18_1)
    }
}
#[doc = "clear ADC14IFG19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG19_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG19_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG19_1 = 1,
}
impl From<CLRADC14IFG19_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG19` writer - clear ADC14IFG19"]
pub type CLRADC14IFG19_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG19_AW, O>;
impl<'a, const O: u8> CLRADC14IFG19_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg19_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG19_AW::CLRADC14IFG19_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg19_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG19_AW::CLRADC14IFG19_1)
    }
}
#[doc = "clear ADC14IFG20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG20_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG20_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG20_1 = 1,
}
impl From<CLRADC14IFG20_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG20` writer - clear ADC14IFG20"]
pub type CLRADC14IFG20_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG20_AW, O>;
impl<'a, const O: u8> CLRADC14IFG20_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg20_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG20_AW::CLRADC14IFG20_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg20_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG20_AW::CLRADC14IFG20_1)
    }
}
#[doc = "clear ADC14IFG21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG21_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG21_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG21_1 = 1,
}
impl From<CLRADC14IFG21_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG21` writer - clear ADC14IFG21"]
pub type CLRADC14IFG21_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG21_AW, O>;
impl<'a, const O: u8> CLRADC14IFG21_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg21_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG21_AW::CLRADC14IFG21_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg21_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG21_AW::CLRADC14IFG21_1)
    }
}
#[doc = "clear ADC14IFG22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG22_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG22_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG22_1 = 1,
}
impl From<CLRADC14IFG22_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG22` writer - clear ADC14IFG22"]
pub type CLRADC14IFG22_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG22_AW, O>;
impl<'a, const O: u8> CLRADC14IFG22_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg22_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG22_AW::CLRADC14IFG22_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg22_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG22_AW::CLRADC14IFG22_1)
    }
}
#[doc = "clear ADC14IFG23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG23_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG23_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG23_1 = 1,
}
impl From<CLRADC14IFG23_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG23` writer - clear ADC14IFG23"]
pub type CLRADC14IFG23_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG23_AW, O>;
impl<'a, const O: u8> CLRADC14IFG23_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg23_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG23_AW::CLRADC14IFG23_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg23_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG23_AW::CLRADC14IFG23_1)
    }
}
#[doc = "clear ADC14IFG24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG24_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG24_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG24_1 = 1,
}
impl From<CLRADC14IFG24_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG24` writer - clear ADC14IFG24"]
pub type CLRADC14IFG24_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG24_AW, O>;
impl<'a, const O: u8> CLRADC14IFG24_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg24_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG24_AW::CLRADC14IFG24_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg24_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG24_AW::CLRADC14IFG24_1)
    }
}
#[doc = "clear ADC14IFG25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG25_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG25_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG25_1 = 1,
}
impl From<CLRADC14IFG25_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG25` writer - clear ADC14IFG25"]
pub type CLRADC14IFG25_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG25_AW, O>;
impl<'a, const O: u8> CLRADC14IFG25_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg25_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG25_AW::CLRADC14IFG25_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg25_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG25_AW::CLRADC14IFG25_1)
    }
}
#[doc = "clear ADC14IFG26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG26_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG26_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG26_1 = 1,
}
impl From<CLRADC14IFG26_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG26` writer - clear ADC14IFG26"]
pub type CLRADC14IFG26_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG26_AW, O>;
impl<'a, const O: u8> CLRADC14IFG26_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg26_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG26_AW::CLRADC14IFG26_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg26_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG26_AW::CLRADC14IFG26_1)
    }
}
#[doc = "clear ADC14IFG27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG27_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG27_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG27_1 = 1,
}
impl From<CLRADC14IFG27_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG27` writer - clear ADC14IFG27"]
pub type CLRADC14IFG27_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG27_AW, O>;
impl<'a, const O: u8> CLRADC14IFG27_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg27_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG27_AW::CLRADC14IFG27_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg27_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG27_AW::CLRADC14IFG27_1)
    }
}
#[doc = "clear ADC14IFG28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG28_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG28_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG28_1 = 1,
}
impl From<CLRADC14IFG28_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG28` writer - clear ADC14IFG28"]
pub type CLRADC14IFG28_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG28_AW, O>;
impl<'a, const O: u8> CLRADC14IFG28_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg28_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG28_AW::CLRADC14IFG28_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg28_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG28_AW::CLRADC14IFG28_1)
    }
}
#[doc = "clear ADC14IFG29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG29_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG29_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG29_1 = 1,
}
impl From<CLRADC14IFG29_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG29` writer - clear ADC14IFG29"]
pub type CLRADC14IFG29_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG29_AW, O>;
impl<'a, const O: u8> CLRADC14IFG29_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg29_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG29_AW::CLRADC14IFG29_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg29_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG29_AW::CLRADC14IFG29_1)
    }
}
#[doc = "clear ADC14IFG30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG30_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG30_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG30_1 = 1,
}
impl From<CLRADC14IFG30_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG30` writer - clear ADC14IFG30"]
pub type CLRADC14IFG30_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG30_AW, O>;
impl<'a, const O: u8> CLRADC14IFG30_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg30_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG30_AW::CLRADC14IFG30_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg30_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG30_AW::CLRADC14IFG30_1)
    }
}
#[doc = "clear ADC14IFG31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14IFG31_AW {
    #[doc = "0: no effect"]
    CLRADC14IFG31_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14IFG31_1 = 1,
}
impl From<CLRADC14IFG31_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14IFG31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG31` writer - clear ADC14IFG31"]
pub type CLRADC14IFG31_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14CLRIFGR0_SPEC, CLRADC14IFG31_AW, O>;
impl<'a, const O: u8> CLRADC14IFG31_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg31_0(self) -> &'a mut W {
        self.variant(CLRADC14IFG31_AW::CLRADC14IFG31_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg31_1(self) -> &'a mut W {
        self.variant(CLRADC14IFG31_AW::CLRADC14IFG31_1)
    }
}
impl W {
    #[doc = "Bit 0 - clear ADC14IFG0"]
    #[inline(always)]
    pub fn clradc14ifg0(&mut self) -> CLRADC14IFG0_W<0> {
        CLRADC14IFG0_W::new(self)
    }
    #[doc = "Bit 1 - clear ADC14IFG1"]
    #[inline(always)]
    pub fn clradc14ifg1(&mut self) -> CLRADC14IFG1_W<1> {
        CLRADC14IFG1_W::new(self)
    }
    #[doc = "Bit 2 - clear ADC14IFG2"]
    #[inline(always)]
    pub fn clradc14ifg2(&mut self) -> CLRADC14IFG2_W<2> {
        CLRADC14IFG2_W::new(self)
    }
    #[doc = "Bit 3 - clear ADC14IFG3"]
    #[inline(always)]
    pub fn clradc14ifg3(&mut self) -> CLRADC14IFG3_W<3> {
        CLRADC14IFG3_W::new(self)
    }
    #[doc = "Bit 4 - clear ADC14IFG4"]
    #[inline(always)]
    pub fn clradc14ifg4(&mut self) -> CLRADC14IFG4_W<4> {
        CLRADC14IFG4_W::new(self)
    }
    #[doc = "Bit 5 - clear ADC14IFG5"]
    #[inline(always)]
    pub fn clradc14ifg5(&mut self) -> CLRADC14IFG5_W<5> {
        CLRADC14IFG5_W::new(self)
    }
    #[doc = "Bit 6 - clear ADC14IFG6"]
    #[inline(always)]
    pub fn clradc14ifg6(&mut self) -> CLRADC14IFG6_W<6> {
        CLRADC14IFG6_W::new(self)
    }
    #[doc = "Bit 7 - clear ADC14IFG7"]
    #[inline(always)]
    pub fn clradc14ifg7(&mut self) -> CLRADC14IFG7_W<7> {
        CLRADC14IFG7_W::new(self)
    }
    #[doc = "Bit 8 - clear ADC14IFG8"]
    #[inline(always)]
    pub fn clradc14ifg8(&mut self) -> CLRADC14IFG8_W<8> {
        CLRADC14IFG8_W::new(self)
    }
    #[doc = "Bit 9 - clear ADC14IFG9"]
    #[inline(always)]
    pub fn clradc14ifg9(&mut self) -> CLRADC14IFG9_W<9> {
        CLRADC14IFG9_W::new(self)
    }
    #[doc = "Bit 10 - clear ADC14IFG10"]
    #[inline(always)]
    pub fn clradc14ifg10(&mut self) -> CLRADC14IFG10_W<10> {
        CLRADC14IFG10_W::new(self)
    }
    #[doc = "Bit 11 - clear ADC14IFG11"]
    #[inline(always)]
    pub fn clradc14ifg11(&mut self) -> CLRADC14IFG11_W<11> {
        CLRADC14IFG11_W::new(self)
    }
    #[doc = "Bit 12 - clear ADC14IFG12"]
    #[inline(always)]
    pub fn clradc14ifg12(&mut self) -> CLRADC14IFG12_W<12> {
        CLRADC14IFG12_W::new(self)
    }
    #[doc = "Bit 13 - clear ADC14IFG13"]
    #[inline(always)]
    pub fn clradc14ifg13(&mut self) -> CLRADC14IFG13_W<13> {
        CLRADC14IFG13_W::new(self)
    }
    #[doc = "Bit 14 - clear ADC14IFG14"]
    #[inline(always)]
    pub fn clradc14ifg14(&mut self) -> CLRADC14IFG14_W<14> {
        CLRADC14IFG14_W::new(self)
    }
    #[doc = "Bit 15 - clear ADC14IFG15"]
    #[inline(always)]
    pub fn clradc14ifg15(&mut self) -> CLRADC14IFG15_W<15> {
        CLRADC14IFG15_W::new(self)
    }
    #[doc = "Bit 16 - clear ADC14IFG16"]
    #[inline(always)]
    pub fn clradc14ifg16(&mut self) -> CLRADC14IFG16_W<16> {
        CLRADC14IFG16_W::new(self)
    }
    #[doc = "Bit 17 - clear ADC14IFG17"]
    #[inline(always)]
    pub fn clradc14ifg17(&mut self) -> CLRADC14IFG17_W<17> {
        CLRADC14IFG17_W::new(self)
    }
    #[doc = "Bit 18 - clear ADC14IFG18"]
    #[inline(always)]
    pub fn clradc14ifg18(&mut self) -> CLRADC14IFG18_W<18> {
        CLRADC14IFG18_W::new(self)
    }
    #[doc = "Bit 19 - clear ADC14IFG19"]
    #[inline(always)]
    pub fn clradc14ifg19(&mut self) -> CLRADC14IFG19_W<19> {
        CLRADC14IFG19_W::new(self)
    }
    #[doc = "Bit 20 - clear ADC14IFG20"]
    #[inline(always)]
    pub fn clradc14ifg20(&mut self) -> CLRADC14IFG20_W<20> {
        CLRADC14IFG20_W::new(self)
    }
    #[doc = "Bit 21 - clear ADC14IFG21"]
    #[inline(always)]
    pub fn clradc14ifg21(&mut self) -> CLRADC14IFG21_W<21> {
        CLRADC14IFG21_W::new(self)
    }
    #[doc = "Bit 22 - clear ADC14IFG22"]
    #[inline(always)]
    pub fn clradc14ifg22(&mut self) -> CLRADC14IFG22_W<22> {
        CLRADC14IFG22_W::new(self)
    }
    #[doc = "Bit 23 - clear ADC14IFG23"]
    #[inline(always)]
    pub fn clradc14ifg23(&mut self) -> CLRADC14IFG23_W<23> {
        CLRADC14IFG23_W::new(self)
    }
    #[doc = "Bit 24 - clear ADC14IFG24"]
    #[inline(always)]
    pub fn clradc14ifg24(&mut self) -> CLRADC14IFG24_W<24> {
        CLRADC14IFG24_W::new(self)
    }
    #[doc = "Bit 25 - clear ADC14IFG25"]
    #[inline(always)]
    pub fn clradc14ifg25(&mut self) -> CLRADC14IFG25_W<25> {
        CLRADC14IFG25_W::new(self)
    }
    #[doc = "Bit 26 - clear ADC14IFG26"]
    #[inline(always)]
    pub fn clradc14ifg26(&mut self) -> CLRADC14IFG26_W<26> {
        CLRADC14IFG26_W::new(self)
    }
    #[doc = "Bit 27 - clear ADC14IFG27"]
    #[inline(always)]
    pub fn clradc14ifg27(&mut self) -> CLRADC14IFG27_W<27> {
        CLRADC14IFG27_W::new(self)
    }
    #[doc = "Bit 28 - clear ADC14IFG28"]
    #[inline(always)]
    pub fn clradc14ifg28(&mut self) -> CLRADC14IFG28_W<28> {
        CLRADC14IFG28_W::new(self)
    }
    #[doc = "Bit 29 - clear ADC14IFG29"]
    #[inline(always)]
    pub fn clradc14ifg29(&mut self) -> CLRADC14IFG29_W<29> {
        CLRADC14IFG29_W::new(self)
    }
    #[doc = "Bit 30 - clear ADC14IFG30"]
    #[inline(always)]
    pub fn clradc14ifg30(&mut self) -> CLRADC14IFG30_W<30> {
        CLRADC14IFG30_W::new(self)
    }
    #[doc = "Bit 31 - clear ADC14IFG31"]
    #[inline(always)]
    pub fn clradc14ifg31(&mut self) -> CLRADC14IFG31_W<31> {
        CLRADC14IFG31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Interrupt Flag 0 Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14clrifgr0](index.html) module"]
pub struct ADC14CLRIFGR0_SPEC;
impl crate::RegisterSpec for ADC14CLRIFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adc14clrifgr0::W](W) writer structure"]
impl crate::Writable for ADC14CLRIFGR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC14CLRIFGR0 to value 0"]
impl crate::Resettable for ADC14CLRIFGR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
