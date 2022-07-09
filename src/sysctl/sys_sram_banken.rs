#[doc = "Register `SYS_SRAM_BANKEN` reader"]
pub struct R(crate::R<SYS_SRAM_BANKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BANKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BANKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BANKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BANKEN` writer"]
pub struct W(crate::W<SYS_SRAM_BANKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BANKEN_SPEC>;
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
impl From<crate::W<SYS_SRAM_BANKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BANKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNK0_EN` reader - SRAM Bank0 enable"]
pub type BNK0_EN_R = crate::BitReader<bool>;
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK1_EN_A {
    #[doc = "0: Disables Bank1 of the SRAM"]
    BNK1_EN_0 = 0,
    #[doc = "1: Enables Bank1 of the SRAM"]
    BNK1_EN_1 = 1,
}
impl From<BNK1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK1_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK1_EN` reader - SRAM Bank1 enable"]
pub type BNK1_EN_R = crate::BitReader<BNK1_EN_A>;
impl BNK1_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK1_EN_A {
        match self.bits {
            false => BNK1_EN_A::BNK1_EN_0,
            true => BNK1_EN_A::BNK1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK1_EN_0`"]
    #[inline(always)]
    pub fn is_bnk1_en_0(&self) -> bool {
        *self == BNK1_EN_A::BNK1_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK1_EN_1`"]
    #[inline(always)]
    pub fn is_bnk1_en_1(&self) -> bool {
        *self == BNK1_EN_A::BNK1_EN_1
    }
}
#[doc = "Field `BNK1_EN` writer - SRAM Bank1 enable"]
pub type BNK1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_SPEC, BNK1_EN_A, O>;
impl<'a, const O: u8> BNK1_EN_W<'a, O> {
    #[doc = "Disables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn bnk1_en_0(self) -> &'a mut W {
        self.variant(BNK1_EN_A::BNK1_EN_0)
    }
    #[doc = "Enables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn bnk1_en_1(self) -> &'a mut W {
        self.variant(BNK1_EN_A::BNK1_EN_1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK2_EN_A {
    #[doc = "0: Disables Bank2 of the SRAM"]
    BNK2_EN_0 = 0,
    #[doc = "1: Enables Bank2 of the SRAM"]
    BNK2_EN_1 = 1,
}
impl From<BNK2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK2_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK2_EN` reader - SRAM Bank1 enable"]
pub type BNK2_EN_R = crate::BitReader<BNK2_EN_A>;
impl BNK2_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK2_EN_A {
        match self.bits {
            false => BNK2_EN_A::BNK2_EN_0,
            true => BNK2_EN_A::BNK2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK2_EN_0`"]
    #[inline(always)]
    pub fn is_bnk2_en_0(&self) -> bool {
        *self == BNK2_EN_A::BNK2_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK2_EN_1`"]
    #[inline(always)]
    pub fn is_bnk2_en_1(&self) -> bool {
        *self == BNK2_EN_A::BNK2_EN_1
    }
}
#[doc = "Field `BNK2_EN` writer - SRAM Bank1 enable"]
pub type BNK2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_SPEC, BNK2_EN_A, O>;
impl<'a, const O: u8> BNK2_EN_W<'a, O> {
    #[doc = "Disables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn bnk2_en_0(self) -> &'a mut W {
        self.variant(BNK2_EN_A::BNK2_EN_0)
    }
    #[doc = "Enables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn bnk2_en_1(self) -> &'a mut W {
        self.variant(BNK2_EN_A::BNK2_EN_1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK3_EN_A {
    #[doc = "0: Disables Bank3 of the SRAM"]
    BNK3_EN_0 = 0,
    #[doc = "1: Enables Bank3 of the SRAM"]
    BNK3_EN_1 = 1,
}
impl From<BNK3_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK3_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK3_EN` reader - SRAM Bank1 enable"]
pub type BNK3_EN_R = crate::BitReader<BNK3_EN_A>;
impl BNK3_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK3_EN_A {
        match self.bits {
            false => BNK3_EN_A::BNK3_EN_0,
            true => BNK3_EN_A::BNK3_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK3_EN_0`"]
    #[inline(always)]
    pub fn is_bnk3_en_0(&self) -> bool {
        *self == BNK3_EN_A::BNK3_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK3_EN_1`"]
    #[inline(always)]
    pub fn is_bnk3_en_1(&self) -> bool {
        *self == BNK3_EN_A::BNK3_EN_1
    }
}
#[doc = "Field `BNK3_EN` writer - SRAM Bank1 enable"]
pub type BNK3_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_SPEC, BNK3_EN_A, O>;
impl<'a, const O: u8> BNK3_EN_W<'a, O> {
    #[doc = "Disables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn bnk3_en_0(self) -> &'a mut W {
        self.variant(BNK3_EN_A::BNK3_EN_0)
    }
    #[doc = "Enables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn bnk3_en_1(self) -> &'a mut W {
        self.variant(BNK3_EN_A::BNK3_EN_1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK4_EN_A {
    #[doc = "0: Disables Bank4 of the SRAM"]
    BNK4_EN_0 = 0,
    #[doc = "1: Enables Bank4 of the SRAM"]
    BNK4_EN_1 = 1,
}
impl From<BNK4_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK4_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK4_EN` reader - SRAM Bank1 enable"]
pub type BNK4_EN_R = crate::BitReader<BNK4_EN_A>;
impl BNK4_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK4_EN_A {
        match self.bits {
            false => BNK4_EN_A::BNK4_EN_0,
            true => BNK4_EN_A::BNK4_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK4_EN_0`"]
    #[inline(always)]
    pub fn is_bnk4_en_0(&self) -> bool {
        *self == BNK4_EN_A::BNK4_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK4_EN_1`"]
    #[inline(always)]
    pub fn is_bnk4_en_1(&self) -> bool {
        *self == BNK4_EN_A::BNK4_EN_1
    }
}
#[doc = "Field `BNK4_EN` writer - SRAM Bank1 enable"]
pub type BNK4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_SPEC, BNK4_EN_A, O>;
impl<'a, const O: u8> BNK4_EN_W<'a, O> {
    #[doc = "Disables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn bnk4_en_0(self) -> &'a mut W {
        self.variant(BNK4_EN_A::BNK4_EN_0)
    }
    #[doc = "Enables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn bnk4_en_1(self) -> &'a mut W {
        self.variant(BNK4_EN_A::BNK4_EN_1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK5_EN_A {
    #[doc = "0: Disables Bank5 of the SRAM"]
    BNK5_EN_0 = 0,
    #[doc = "1: Enables Bank5 of the SRAM"]
    BNK5_EN_1 = 1,
}
impl From<BNK5_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK5_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK5_EN` reader - SRAM Bank1 enable"]
pub type BNK5_EN_R = crate::BitReader<BNK5_EN_A>;
impl BNK5_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK5_EN_A {
        match self.bits {
            false => BNK5_EN_A::BNK5_EN_0,
            true => BNK5_EN_A::BNK5_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK5_EN_0`"]
    #[inline(always)]
    pub fn is_bnk5_en_0(&self) -> bool {
        *self == BNK5_EN_A::BNK5_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK5_EN_1`"]
    #[inline(always)]
    pub fn is_bnk5_en_1(&self) -> bool {
        *self == BNK5_EN_A::BNK5_EN_1
    }
}
#[doc = "Field `BNK5_EN` writer - SRAM Bank1 enable"]
pub type BNK5_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_SPEC, BNK5_EN_A, O>;
impl<'a, const O: u8> BNK5_EN_W<'a, O> {
    #[doc = "Disables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn bnk5_en_0(self) -> &'a mut W {
        self.variant(BNK5_EN_A::BNK5_EN_0)
    }
    #[doc = "Enables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn bnk5_en_1(self) -> &'a mut W {
        self.variant(BNK5_EN_A::BNK5_EN_1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK6_EN_A {
    #[doc = "0: Disables Bank6 of the SRAM"]
    BNK6_EN_0 = 0,
    #[doc = "1: Enables Bank6 of the SRAM"]
    BNK6_EN_1 = 1,
}
impl From<BNK6_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK6_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK6_EN` reader - SRAM Bank1 enable"]
pub type BNK6_EN_R = crate::BitReader<BNK6_EN_A>;
impl BNK6_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK6_EN_A {
        match self.bits {
            false => BNK6_EN_A::BNK6_EN_0,
            true => BNK6_EN_A::BNK6_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK6_EN_0`"]
    #[inline(always)]
    pub fn is_bnk6_en_0(&self) -> bool {
        *self == BNK6_EN_A::BNK6_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK6_EN_1`"]
    #[inline(always)]
    pub fn is_bnk6_en_1(&self) -> bool {
        *self == BNK6_EN_A::BNK6_EN_1
    }
}
#[doc = "Field `BNK6_EN` writer - SRAM Bank1 enable"]
pub type BNK6_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_SPEC, BNK6_EN_A, O>;
impl<'a, const O: u8> BNK6_EN_W<'a, O> {
    #[doc = "Disables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn bnk6_en_0(self) -> &'a mut W {
        self.variant(BNK6_EN_A::BNK6_EN_0)
    }
    #[doc = "Enables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn bnk6_en_1(self) -> &'a mut W {
        self.variant(BNK6_EN_A::BNK6_EN_1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK7_EN_A {
    #[doc = "0: Disables Bank7 of the SRAM"]
    BNK7_EN_0 = 0,
    #[doc = "1: Enables Bank7 of the SRAM"]
    BNK7_EN_1 = 1,
}
impl From<BNK7_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK7_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK7_EN` reader - SRAM Bank1 enable"]
pub type BNK7_EN_R = crate::BitReader<BNK7_EN_A>;
impl BNK7_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK7_EN_A {
        match self.bits {
            false => BNK7_EN_A::BNK7_EN_0,
            true => BNK7_EN_A::BNK7_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK7_EN_0`"]
    #[inline(always)]
    pub fn is_bnk7_en_0(&self) -> bool {
        *self == BNK7_EN_A::BNK7_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK7_EN_1`"]
    #[inline(always)]
    pub fn is_bnk7_en_1(&self) -> bool {
        *self == BNK7_EN_A::BNK7_EN_1
    }
}
#[doc = "Field `BNK7_EN` writer - SRAM Bank1 enable"]
pub type BNK7_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_SPEC, BNK7_EN_A, O>;
impl<'a, const O: u8> BNK7_EN_W<'a, O> {
    #[doc = "Disables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn bnk7_en_0(self) -> &'a mut W {
        self.variant(BNK7_EN_A::BNK7_EN_0)
    }
    #[doc = "Enables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn bnk7_en_1(self) -> &'a mut W {
        self.variant(BNK7_EN_A::BNK7_EN_1)
    }
}
#[doc = "SRAM ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_RDY_A {
    #[doc = "0: SRAM is not ready for accesses. Banks are undergoing an enable or disable sequence, and reads or writes to SRAM are stalled until the banks are ready"]
    SRAM_RDY_0 = 0,
    #[doc = "1: SRAM is ready for accesses. All SRAM banks are enabled/disabled according to values of bits 7:0 of this register"]
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
    #[doc = "Bit 0 - SRAM Bank0 enable"]
    #[inline(always)]
    pub fn bnk0_en(&self) -> BNK0_EN_R {
        BNK0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk1_en(&self) -> BNK1_EN_R {
        BNK1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk2_en(&self) -> BNK2_EN_R {
        BNK2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk3_en(&self) -> BNK3_EN_R {
        BNK3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk4_en(&self) -> BNK4_EN_R {
        BNK4_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk5_en(&self) -> BNK5_EN_R {
        BNK5_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk6_en(&self) -> BNK6_EN_R {
        BNK6_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk7_en(&self) -> BNK7_EN_R {
        BNK7_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM ready"]
    #[inline(always)]
    pub fn sram_rdy(&self) -> SRAM_RDY_R {
        SRAM_RDY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk1_en(&mut self) -> BNK1_EN_W<1> {
        BNK1_EN_W::new(self)
    }
    #[doc = "Bit 2 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk2_en(&mut self) -> BNK2_EN_W<2> {
        BNK2_EN_W::new(self)
    }
    #[doc = "Bit 3 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk3_en(&mut self) -> BNK3_EN_W<3> {
        BNK3_EN_W::new(self)
    }
    #[doc = "Bit 4 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk4_en(&mut self) -> BNK4_EN_W<4> {
        BNK4_EN_W::new(self)
    }
    #[doc = "Bit 5 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk5_en(&mut self) -> BNK5_EN_W<5> {
        BNK5_EN_W::new(self)
    }
    #[doc = "Bit 6 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk6_en(&mut self) -> BNK6_EN_W<6> {
        BNK6_EN_W::new(self)
    }
    #[doc = "Bit 7 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk7_en(&mut self) -> BNK7_EN_W<7> {
        BNK7_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Bank Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_banken](index.html) module"]
pub struct SYS_SRAM_BANKEN_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BANKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_banken::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BANKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_banken::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BANKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BANKEN to value 0xff"]
impl crate::Resettable for SYS_SRAM_BANKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
