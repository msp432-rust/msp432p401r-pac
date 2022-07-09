#[doc = "Register `RTCPS` reader"]
pub struct R(crate::R<RTCPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCPS` writer"]
pub struct W(crate::W<RTCPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCPS_SPEC>;
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
impl From<crate::W<RTCPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RT0PS` reader - Prescale timer 0 counter value"]
pub type RT0PS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RT0PS` writer - Prescale timer 0 counter value"]
pub type RT0PS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCPS_SPEC, u8, u8, 8, O>;
#[doc = "Field `RT1PS` reader - Prescale timer 1 counter value"]
pub type RT1PS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RT1PS` writer - Prescale timer 1 counter value"]
pub type RT1PS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCPS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Prescale timer 0 counter value"]
    #[inline(always)]
    pub fn rt0ps(&self) -> RT0PS_R {
        RT0PS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Prescale timer 1 counter value"]
    #[inline(always)]
    pub fn rt1ps(&self) -> RT1PS_R {
        RT1PS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescale timer 0 counter value"]
    #[inline(always)]
    pub fn rt0ps(&mut self) -> RT0PS_W<0> {
        RT0PS_W::new(self)
    }
    #[doc = "Bits 8:15 - Prescale timer 1 counter value"]
    #[inline(always)]
    pub fn rt1ps(&mut self) -> RT1PS_W<8> {
        RT1PS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real-Time Clock Prescale Timer Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps](index.html) module"]
pub struct RTCPS_SPEC;
impl crate::RegisterSpec for RTCPS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcps::R](R) reader structure"]
impl crate::Readable for RTCPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcps::W](W) writer structure"]
impl crate::Writable for RTCPS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCPS to value 0"]
impl crate::Resettable for RTCPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
