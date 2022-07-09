#[doc = "Register `RTCTCMP` reader"]
pub struct R(crate::R<RTCTCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCTCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCTCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCTCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCTCMP` writer"]
pub struct W(crate::W<RTCTCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCTCMP_SPEC>;
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
impl From<crate::W<RTCTCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCTCMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCTCMP` reader - Real-time clock temperature compensation"]
pub type RTCTCMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCTCMP` writer - Real-time clock temperature compensation"]
pub type RTCTCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCTCMP_SPEC, u8, u8, 8, O>;
#[doc = "Real-time clock temperature compensation write OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCTCOK_A {
    #[doc = "0: Write to RTCTCMPx is unsuccessful"]
    RTCTCOK_0 = 0,
    #[doc = "1: Write to RTCTCMPx is successful"]
    RTCTCOK_1 = 1,
}
impl From<RTCTCOK_A> for bool {
    #[inline(always)]
    fn from(variant: RTCTCOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCTCOK` reader - Real-time clock temperature compensation write OK"]
pub type RTCTCOK_R = crate::BitReader<RTCTCOK_A>;
impl RTCTCOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTCOK_A {
        match self.bits {
            false => RTCTCOK_A::RTCTCOK_0,
            true => RTCTCOK_A::RTCTCOK_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCTCOK_0`"]
    #[inline(always)]
    pub fn is_rtctcok_0(&self) -> bool {
        *self == RTCTCOK_A::RTCTCOK_0
    }
    #[doc = "Checks if the value of the field is `RTCTCOK_1`"]
    #[inline(always)]
    pub fn is_rtctcok_1(&self) -> bool {
        *self == RTCTCOK_A::RTCTCOK_1
    }
}
#[doc = "Field `RTCTCRDY` reader - Real-time clock temperature compensation ready"]
pub type RTCTCRDY_R = crate::BitReader<bool>;
#[doc = "Real-time clock temperature compensation sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCTCMPS_A {
    #[doc = "0: Down calibration. Frequency adjusted down"]
    RTCTCMPS_0 = 0,
    #[doc = "1: Up calibration. Frequency adjusted up"]
    RTCTCMPS_1 = 1,
}
impl From<RTCTCMPS_A> for bool {
    #[inline(always)]
    fn from(variant: RTCTCMPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCTCMPS` reader - Real-time clock temperature compensation sign"]
pub type RTCTCMPS_R = crate::BitReader<RTCTCMPS_A>;
impl RTCTCMPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTCMPS_A {
        match self.bits {
            false => RTCTCMPS_A::RTCTCMPS_0,
            true => RTCTCMPS_A::RTCTCMPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCTCMPS_0`"]
    #[inline(always)]
    pub fn is_rtctcmps_0(&self) -> bool {
        *self == RTCTCMPS_A::RTCTCMPS_0
    }
    #[doc = "Checks if the value of the field is `RTCTCMPS_1`"]
    #[inline(always)]
    pub fn is_rtctcmps_1(&self) -> bool {
        *self == RTCTCMPS_A::RTCTCMPS_1
    }
}
#[doc = "Field `RTCTCMPS` writer - Real-time clock temperature compensation sign"]
pub type RTCTCMPS_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, RTCTCMPS_A, O>;
impl<'a, const O: u8> RTCTCMPS_W<'a, O> {
    #[doc = "Down calibration. Frequency adjusted down"]
    #[inline(always)]
    pub fn rtctcmps_0(self) -> &'a mut W {
        self.variant(RTCTCMPS_A::RTCTCMPS_0)
    }
    #[doc = "Up calibration. Frequency adjusted up"]
    #[inline(always)]
    pub fn rtctcmps_1(self) -> &'a mut W {
        self.variant(RTCTCMPS_A::RTCTCMPS_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Real-time clock temperature compensation"]
    #[inline(always)]
    pub fn rtctcmp(&self) -> RTCTCMP_R {
        RTCTCMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - Real-time clock temperature compensation write OK"]
    #[inline(always)]
    pub fn rtctcok(&self) -> RTCTCOK_R {
        RTCTCOK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Real-time clock temperature compensation ready"]
    #[inline(always)]
    pub fn rtctcrdy(&self) -> RTCTCRDY_R {
        RTCTCRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Real-time clock temperature compensation sign"]
    #[inline(always)]
    pub fn rtctcmps(&self) -> RTCTCMPS_R {
        RTCTCMPS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Real-time clock temperature compensation"]
    #[inline(always)]
    pub fn rtctcmp(&mut self) -> RTCTCMP_W<0> {
        RTCTCMP_W::new(self)
    }
    #[doc = "Bit 15 - Real-time clock temperature compensation sign"]
    #[inline(always)]
    pub fn rtctcmps(&mut self) -> RTCTCMPS_W<15> {
        RTCTCMPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCTCMP Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctcmp](index.html) module"]
pub struct RTCTCMP_SPEC;
impl crate::RegisterSpec for RTCTCMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtctcmp::R](R) reader structure"]
impl crate::Readable for RTCTCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtctcmp::W](W) writer structure"]
impl crate::Writable for RTCTCMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCTCMP to value 0x4000"]
impl crate::Resettable for RTCTCMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
