#[doc = "Register `SYS_SRAM_BANKRET` reader"]
pub struct R(crate::R<SYS_SRAM_BANKRET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BANKRET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BANKRET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BANKRET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BANKRET` writer"]
pub struct W(crate::W<SYS_SRAM_BANKRET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BANKRET_SPEC>;
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
impl From<crate::W<SYS_SRAM_BANKRET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BANKRET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNK0_RET` reader - Bank0 retention"]
pub type BNK0_RET_R = crate::BitReader<bool>;
#[doc = "Bank1 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK1_RET_A {
    #[doc = "0: Bank1 of the SRAM is not retained in LPM3 or LPM4"]
    BNK1_RET_0 = 0,
    #[doc = "1: Bank1 of the SRAM is retained in LPM3 and LPM4"]
    BNK1_RET_1 = 1,
}
impl From<BNK1_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK1_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK1_RET` reader - Bank1 retention"]
pub type BNK1_RET_R = crate::BitReader<BNK1_RET_A>;
impl BNK1_RET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK1_RET_A {
        match self.bits {
            false => BNK1_RET_A::BNK1_RET_0,
            true => BNK1_RET_A::BNK1_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK1_RET_0`"]
    #[inline(always)]
    pub fn is_bnk1_ret_0(&self) -> bool {
        *self == BNK1_RET_A::BNK1_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK1_RET_1`"]
    #[inline(always)]
    pub fn is_bnk1_ret_1(&self) -> bool {
        *self == BNK1_RET_A::BNK1_RET_1
    }
}
#[doc = "Field `BNK1_RET` writer - Bank1 retention"]
pub type BNK1_RET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKRET_SPEC, BNK1_RET_A, O>;
impl<'a, const O: u8> BNK1_RET_W<'a, O> {
    #[doc = "Bank1 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk1_ret_0(self) -> &'a mut W {
        self.variant(BNK1_RET_A::BNK1_RET_0)
    }
    #[doc = "Bank1 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk1_ret_1(self) -> &'a mut W {
        self.variant(BNK1_RET_A::BNK1_RET_1)
    }
}
#[doc = "Bank2 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK2_RET_A {
    #[doc = "0: Bank2 of the SRAM is not retained in LPM3 or LPM4"]
    BNK2_RET_0 = 0,
    #[doc = "1: Bank2 of the SRAM is retained in LPM3 and LPM4"]
    BNK2_RET_1 = 1,
}
impl From<BNK2_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK2_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK2_RET` reader - Bank2 retention"]
pub type BNK2_RET_R = crate::BitReader<BNK2_RET_A>;
impl BNK2_RET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK2_RET_A {
        match self.bits {
            false => BNK2_RET_A::BNK2_RET_0,
            true => BNK2_RET_A::BNK2_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK2_RET_0`"]
    #[inline(always)]
    pub fn is_bnk2_ret_0(&self) -> bool {
        *self == BNK2_RET_A::BNK2_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK2_RET_1`"]
    #[inline(always)]
    pub fn is_bnk2_ret_1(&self) -> bool {
        *self == BNK2_RET_A::BNK2_RET_1
    }
}
#[doc = "Field `BNK2_RET` writer - Bank2 retention"]
pub type BNK2_RET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKRET_SPEC, BNK2_RET_A, O>;
impl<'a, const O: u8> BNK2_RET_W<'a, O> {
    #[doc = "Bank2 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk2_ret_0(self) -> &'a mut W {
        self.variant(BNK2_RET_A::BNK2_RET_0)
    }
    #[doc = "Bank2 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk2_ret_1(self) -> &'a mut W {
        self.variant(BNK2_RET_A::BNK2_RET_1)
    }
}
#[doc = "Bank3 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK3_RET_A {
    #[doc = "0: Bank3 of the SRAM is not retained in LPM3 or LPM4"]
    BNK3_RET_0 = 0,
    #[doc = "1: Bank3 of the SRAM is retained in LPM3 and LPM4"]
    BNK3_RET_1 = 1,
}
impl From<BNK3_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK3_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK3_RET` reader - Bank3 retention"]
pub type BNK3_RET_R = crate::BitReader<BNK3_RET_A>;
impl BNK3_RET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK3_RET_A {
        match self.bits {
            false => BNK3_RET_A::BNK3_RET_0,
            true => BNK3_RET_A::BNK3_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK3_RET_0`"]
    #[inline(always)]
    pub fn is_bnk3_ret_0(&self) -> bool {
        *self == BNK3_RET_A::BNK3_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK3_RET_1`"]
    #[inline(always)]
    pub fn is_bnk3_ret_1(&self) -> bool {
        *self == BNK3_RET_A::BNK3_RET_1
    }
}
#[doc = "Field `BNK3_RET` writer - Bank3 retention"]
pub type BNK3_RET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKRET_SPEC, BNK3_RET_A, O>;
impl<'a, const O: u8> BNK3_RET_W<'a, O> {
    #[doc = "Bank3 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk3_ret_0(self) -> &'a mut W {
        self.variant(BNK3_RET_A::BNK3_RET_0)
    }
    #[doc = "Bank3 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk3_ret_1(self) -> &'a mut W {
        self.variant(BNK3_RET_A::BNK3_RET_1)
    }
}
#[doc = "Bank4 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK4_RET_A {
    #[doc = "0: Bank4 of the SRAM is not retained in LPM3 or LPM4"]
    BNK4_RET_0 = 0,
    #[doc = "1: Bank4 of the SRAM is retained in LPM3 and LPM4"]
    BNK4_RET_1 = 1,
}
impl From<BNK4_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK4_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK4_RET` reader - Bank4 retention"]
pub type BNK4_RET_R = crate::BitReader<BNK4_RET_A>;
impl BNK4_RET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK4_RET_A {
        match self.bits {
            false => BNK4_RET_A::BNK4_RET_0,
            true => BNK4_RET_A::BNK4_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK4_RET_0`"]
    #[inline(always)]
    pub fn is_bnk4_ret_0(&self) -> bool {
        *self == BNK4_RET_A::BNK4_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK4_RET_1`"]
    #[inline(always)]
    pub fn is_bnk4_ret_1(&self) -> bool {
        *self == BNK4_RET_A::BNK4_RET_1
    }
}
#[doc = "Field `BNK4_RET` writer - Bank4 retention"]
pub type BNK4_RET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKRET_SPEC, BNK4_RET_A, O>;
impl<'a, const O: u8> BNK4_RET_W<'a, O> {
    #[doc = "Bank4 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk4_ret_0(self) -> &'a mut W {
        self.variant(BNK4_RET_A::BNK4_RET_0)
    }
    #[doc = "Bank4 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk4_ret_1(self) -> &'a mut W {
        self.variant(BNK4_RET_A::BNK4_RET_1)
    }
}
#[doc = "Bank5 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK5_RET_A {
    #[doc = "0: Bank5 of the SRAM is not retained in LPM3 or LPM4"]
    BNK5_RET_0 = 0,
    #[doc = "1: Bank5 of the SRAM is retained in LPM3 and LPM4"]
    BNK5_RET_1 = 1,
}
impl From<BNK5_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK5_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK5_RET` reader - Bank5 retention"]
pub type BNK5_RET_R = crate::BitReader<BNK5_RET_A>;
impl BNK5_RET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK5_RET_A {
        match self.bits {
            false => BNK5_RET_A::BNK5_RET_0,
            true => BNK5_RET_A::BNK5_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK5_RET_0`"]
    #[inline(always)]
    pub fn is_bnk5_ret_0(&self) -> bool {
        *self == BNK5_RET_A::BNK5_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK5_RET_1`"]
    #[inline(always)]
    pub fn is_bnk5_ret_1(&self) -> bool {
        *self == BNK5_RET_A::BNK5_RET_1
    }
}
#[doc = "Field `BNK5_RET` writer - Bank5 retention"]
pub type BNK5_RET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKRET_SPEC, BNK5_RET_A, O>;
impl<'a, const O: u8> BNK5_RET_W<'a, O> {
    #[doc = "Bank5 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk5_ret_0(self) -> &'a mut W {
        self.variant(BNK5_RET_A::BNK5_RET_0)
    }
    #[doc = "Bank5 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk5_ret_1(self) -> &'a mut W {
        self.variant(BNK5_RET_A::BNK5_RET_1)
    }
}
#[doc = "Bank6 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK6_RET_A {
    #[doc = "0: Bank6 of the SRAM is not retained in LPM3 or LPM4"]
    BNK6_RET_0 = 0,
    #[doc = "1: Bank6 of the SRAM is retained in LPM3 and LPM4"]
    BNK6_RET_1 = 1,
}
impl From<BNK6_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK6_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK6_RET` reader - Bank6 retention"]
pub type BNK6_RET_R = crate::BitReader<BNK6_RET_A>;
impl BNK6_RET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK6_RET_A {
        match self.bits {
            false => BNK6_RET_A::BNK6_RET_0,
            true => BNK6_RET_A::BNK6_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK6_RET_0`"]
    #[inline(always)]
    pub fn is_bnk6_ret_0(&self) -> bool {
        *self == BNK6_RET_A::BNK6_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK6_RET_1`"]
    #[inline(always)]
    pub fn is_bnk6_ret_1(&self) -> bool {
        *self == BNK6_RET_A::BNK6_RET_1
    }
}
#[doc = "Field `BNK6_RET` writer - Bank6 retention"]
pub type BNK6_RET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKRET_SPEC, BNK6_RET_A, O>;
impl<'a, const O: u8> BNK6_RET_W<'a, O> {
    #[doc = "Bank6 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk6_ret_0(self) -> &'a mut W {
        self.variant(BNK6_RET_A::BNK6_RET_0)
    }
    #[doc = "Bank6 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk6_ret_1(self) -> &'a mut W {
        self.variant(BNK6_RET_A::BNK6_RET_1)
    }
}
#[doc = "Bank7 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK7_RET_A {
    #[doc = "0: Bank7 of the SRAM is not retained in LPM3 or LPM4"]
    BNK7_RET_0 = 0,
    #[doc = "1: Bank7 of the SRAM is retained in LPM3 and LPM4"]
    BNK7_RET_1 = 1,
}
impl From<BNK7_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK7_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK7_RET` reader - Bank7 retention"]
pub type BNK7_RET_R = crate::BitReader<BNK7_RET_A>;
impl BNK7_RET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK7_RET_A {
        match self.bits {
            false => BNK7_RET_A::BNK7_RET_0,
            true => BNK7_RET_A::BNK7_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK7_RET_0`"]
    #[inline(always)]
    pub fn is_bnk7_ret_0(&self) -> bool {
        *self == BNK7_RET_A::BNK7_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK7_RET_1`"]
    #[inline(always)]
    pub fn is_bnk7_ret_1(&self) -> bool {
        *self == BNK7_RET_A::BNK7_RET_1
    }
}
#[doc = "Field `BNK7_RET` writer - Bank7 retention"]
pub type BNK7_RET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKRET_SPEC, BNK7_RET_A, O>;
impl<'a, const O: u8> BNK7_RET_W<'a, O> {
    #[doc = "Bank7 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk7_ret_0(self) -> &'a mut W {
        self.variant(BNK7_RET_A::BNK7_RET_0)
    }
    #[doc = "Bank7 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk7_ret_1(self) -> &'a mut W {
        self.variant(BNK7_RET_A::BNK7_RET_1)
    }
}
#[doc = "SRAM ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_RDY_A {
    #[doc = "0: SRAM banks are being set up for retention. Entry into LPM3, LPM4 should not be attempted until this bit is set to 1"]
    SRAM_RDY_0 = 0,
    #[doc = "1: SRAM is ready for accesses. All SRAM banks are enabled/disabled for retention according to values of bits 7:0 of this register"]
    SRAM_RDY_1 = 1,
}
impl From<SRAM_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_RDY` reader - SRAM ready"]
pub type SRAM_RDY_R = crate::BitReader<SRAM_RDY_A>;
impl SRAM_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_RDY_A {
        match self.bits {
            false => SRAM_RDY_A::SRAM_RDY_0,
            true => SRAM_RDY_A::SRAM_RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM_RDY_0`"]
    #[inline(always)]
    pub fn is_sram_rdy_0(&self) -> bool {
        *self == SRAM_RDY_A::SRAM_RDY_0
    }
    #[doc = "Checks if the value of the field is `SRAM_RDY_1`"]
    #[inline(always)]
    pub fn is_sram_rdy_1(&self) -> bool {
        *self == SRAM_RDY_A::SRAM_RDY_1
    }
}
impl R {
    #[doc = "Bit 0 - Bank0 retention"]
    #[inline(always)]
    pub fn bnk0_ret(&self) -> BNK0_RET_R {
        BNK0_RET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank1 retention"]
    #[inline(always)]
    pub fn bnk1_ret(&self) -> BNK1_RET_R {
        BNK1_RET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank2 retention"]
    #[inline(always)]
    pub fn bnk2_ret(&self) -> BNK2_RET_R {
        BNK2_RET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank3 retention"]
    #[inline(always)]
    pub fn bnk3_ret(&self) -> BNK3_RET_R {
        BNK3_RET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bank4 retention"]
    #[inline(always)]
    pub fn bnk4_ret(&self) -> BNK4_RET_R {
        BNK4_RET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bank5 retention"]
    #[inline(always)]
    pub fn bnk5_ret(&self) -> BNK5_RET_R {
        BNK5_RET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bank6 retention"]
    #[inline(always)]
    pub fn bnk6_ret(&self) -> BNK6_RET_R {
        BNK6_RET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bank7 retention"]
    #[inline(always)]
    pub fn bnk7_ret(&self) -> BNK7_RET_R {
        BNK7_RET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM ready"]
    #[inline(always)]
    pub fn sram_rdy(&self) -> SRAM_RDY_R {
        SRAM_RDY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Bank1 retention"]
    #[inline(always)]
    pub fn bnk1_ret(&mut self) -> BNK1_RET_W<1> {
        BNK1_RET_W::new(self)
    }
    #[doc = "Bit 2 - Bank2 retention"]
    #[inline(always)]
    pub fn bnk2_ret(&mut self) -> BNK2_RET_W<2> {
        BNK2_RET_W::new(self)
    }
    #[doc = "Bit 3 - Bank3 retention"]
    #[inline(always)]
    pub fn bnk3_ret(&mut self) -> BNK3_RET_W<3> {
        BNK3_RET_W::new(self)
    }
    #[doc = "Bit 4 - Bank4 retention"]
    #[inline(always)]
    pub fn bnk4_ret(&mut self) -> BNK4_RET_W<4> {
        BNK4_RET_W::new(self)
    }
    #[doc = "Bit 5 - Bank5 retention"]
    #[inline(always)]
    pub fn bnk5_ret(&mut self) -> BNK5_RET_W<5> {
        BNK5_RET_W::new(self)
    }
    #[doc = "Bit 6 - Bank6 retention"]
    #[inline(always)]
    pub fn bnk6_ret(&mut self) -> BNK6_RET_W<6> {
        BNK6_RET_W::new(self)
    }
    #[doc = "Bit 7 - Bank7 retention"]
    #[inline(always)]
    pub fn bnk7_ret(&mut self) -> BNK7_RET_W<7> {
        BNK7_RET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Bank Retention Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_bankret](index.html) module"]
pub struct SYS_SRAM_BANKRET_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BANKRET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_bankret::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BANKRET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_bankret::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BANKRET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BANKRET to value 0xff"]
impl crate::Resettable for SYS_SRAM_BANKRET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
