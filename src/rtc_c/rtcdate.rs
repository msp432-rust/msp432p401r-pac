#[doc = "Register `RTCDATE` reader"]
pub struct R(crate::R<RTCDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCDATE` writer"]
pub struct W(crate::W<RTCDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCDATE_SPEC>;
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
impl From<crate::W<RTCDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Day` reader - Day of month (1 to 28, 29, 30, 31)"]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Day` writer - Day of month (1 to 28, 29, 30, 31)"]
pub type DAY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCDATE_SPEC, u8, u8, 5, O>;
#[doc = "Field `Month` reader - Month (1 to 12)"]
pub type MONTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Month` writer - Month (1 to 12)"]
pub type MONTH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCDATE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month (1 to 12)"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W<0> {
        DAY_W::new(self)
    }
    #[doc = "Bits 8:11 - Month (1 to 12)"]
    #[inline(always)]
    pub fn month(&mut self) -> MONTH_W<8> {
        MONTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCDATE - Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcdate](index.html) module"]
pub struct RTCDATE_SPEC;
impl crate::RegisterSpec for RTCDATE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcdate::R](R) reader structure"]
impl crate::Readable for RTCDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcdate::W](W) writer structure"]
impl crate::Writable for RTCDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCDATE to value 0"]
impl crate::Resettable for RTCDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
