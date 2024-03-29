#[doc = "Register `RTCYEAR` reader"]
pub struct R(crate::R<RTCYEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCYEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCYEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCYEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCYEAR` writer"]
pub struct W(crate::W<RTCYEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCYEAR_SPEC>;
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
impl From<crate::W<RTCYEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCYEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YearLowByte` reader - Year low byte. Valid values for Year are 0 to 4095."]
pub type YEARLOWBYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YearLowByte` writer - Year low byte. Valid values for Year are 0 to 4095."]
pub type YEARLOWBYTE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCYEAR_SPEC, u8, u8, 8, O>;
#[doc = "Field `YearHighByte` reader - Year high byte. Valid values for Year are 0 to 4095."]
pub type YEARHIGHBYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YearHighByte` writer - Year high byte. Valid values for Year are 0 to 4095."]
pub type YEARHIGHBYTE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCYEAR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Year low byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    pub fn year_low_byte(&self) -> YEARLOWBYTE_R {
        YEARLOWBYTE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Year high byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    pub fn year_high_byte(&self) -> YEARHIGHBYTE_R {
        YEARHIGHBYTE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Year low byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    pub fn year_low_byte(&mut self) -> YEARLOWBYTE_W<0> {
        YEARLOWBYTE_W::new(self)
    }
    #[doc = "Bits 8:11 - Year high byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    pub fn year_high_byte(&mut self) -> YEARHIGHBYTE_W<8> {
        YEARHIGHBYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCYEAR Register Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcyear](index.html) module"]
pub struct RTCYEAR_SPEC;
impl crate::RegisterSpec for RTCYEAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcyear::R](R) reader structure"]
impl crate::Readable for RTCYEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcyear::W](W) writer structure"]
impl crate::Writable for RTCYEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCYEAR to value 0"]
impl crate::Resettable for RTCYEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
