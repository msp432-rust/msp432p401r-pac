#[doc = "Register `UCBxI2COA1` reader"]
pub struct R(crate::R<UCBXI2COA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBXI2COA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBXI2COA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBXI2COA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCBxI2COA1` writer"]
pub struct W(crate::W<UCBXI2COA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCBXI2COA1_SPEC>;
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
impl From<crate::W<UCBXI2COA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCBXI2COA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2COA1` reader - I2C own address"]
pub type I2COA1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `I2COA1` writer - I2C own address"]
pub type I2COA1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, UCBXI2COA1_SPEC, u16, u16, 10, O>;
#[doc = "Own Address enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCOAEN_A {
    #[doc = "0: The slave address defined in I2COA1 is disabled"]
    UCOAEN_0 = 0,
    #[doc = "1: The slave address defined in I2COA1 is enabled"]
    UCOAEN_1 = 1,
}
impl From<UCOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCOAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCOAEN` reader - Own Address enable register"]
pub type UCOAEN_R = crate::BitReader<UCOAEN_A>;
impl UCOAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCOAEN_A {
        match self.bits {
            false => UCOAEN_A::UCOAEN_0,
            true => UCOAEN_A::UCOAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCOAEN_0`"]
    #[inline(always)]
    pub fn is_ucoaen_0(&self) -> bool {
        *self == UCOAEN_A::UCOAEN_0
    }
    #[doc = "Checks if the value of the field is `UCOAEN_1`"]
    #[inline(always)]
    pub fn is_ucoaen_1(&self) -> bool {
        *self == UCOAEN_A::UCOAEN_1
    }
}
#[doc = "Field `UCOAEN` writer - Own Address enable register"]
pub type UCOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCBXI2COA1_SPEC, UCOAEN_A, O>;
impl<'a, const O: u8> UCOAEN_W<'a, O> {
    #[doc = "The slave address defined in I2COA1 is disabled"]
    #[inline(always)]
    pub fn ucoaen_0(self) -> &'a mut W {
        self.variant(UCOAEN_A::UCOAEN_0)
    }
    #[doc = "The slave address defined in I2COA1 is enabled"]
    #[inline(always)]
    pub fn ucoaen_1(self) -> &'a mut W {
        self.variant(UCOAEN_A::UCOAEN_1)
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa1(&self) -> I2COA1_R {
        I2COA1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&self) -> UCOAEN_R {
        UCOAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa1(&mut self) -> I2COA1_W<0> {
        I2COA1_W::new(self)
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&mut self) -> UCOAEN_W<10> {
        UCOAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx I2C Own Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_i2coa1](index.html) module"]
pub struct UCBXI2COA1_SPEC;
impl crate::RegisterSpec for UCBXI2COA1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_i2coa1::R](R) reader structure"]
impl crate::Readable for UCBXI2COA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucbx_i2coa1::W](W) writer structure"]
impl crate::Writable for UCBXI2COA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCBxI2COA1 to value 0"]
impl crate::Resettable for UCBXI2COA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
