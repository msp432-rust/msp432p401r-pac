#[doc = "Register `T32BGLOAD2` reader"]
pub struct R(crate::R<T32BGLOAD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T32BGLOAD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T32BGLOAD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T32BGLOAD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T32BGLOAD2` writer"]
pub struct W(crate::W<T32BGLOAD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T32BGLOAD2_SPEC>;
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
impl From<crate::W<T32BGLOAD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T32BGLOAD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BGLOAD` reader - Value from which the counter decrements"]
pub type BGLOAD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BGLOAD` writer - Value from which the counter decrements"]
pub type BGLOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T32BGLOAD2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Value from which the counter decrements"]
    #[inline(always)]
    pub fn bgload(&self) -> BGLOAD_R {
        BGLOAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value from which the counter decrements"]
    #[inline(always)]
    pub fn bgload(&mut self) -> BGLOAD_W<0> {
        BGLOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer 2 Background Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32bgload2](index.html) module"]
pub struct T32BGLOAD2_SPEC;
impl crate::RegisterSpec for T32BGLOAD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t32bgload2::R](R) reader structure"]
impl crate::Readable for T32BGLOAD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t32bgload2::W](W) writer structure"]
impl crate::Writable for T32BGLOAD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T32BGLOAD2 to value 0"]
impl crate::Resettable for T32BGLOAD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
