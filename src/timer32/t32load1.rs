#[doc = "Register `T32LOAD1` reader"]
pub struct R(crate::R<T32LOAD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T32LOAD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T32LOAD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T32LOAD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T32LOAD1` writer"]
pub struct W(crate::W<T32LOAD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T32LOAD1_SPEC>;
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
impl From<crate::W<T32LOAD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T32LOAD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD` reader - The value from which the Timer 1 counter decrements"]
pub type LOAD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOAD` writer - The value from which the Timer 1 counter decrements"]
pub type LOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T32LOAD1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The value from which the Timer 1 counter decrements"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The value from which the Timer 1 counter decrements"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W<0> {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer 1 Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32load1](index.html) module"]
pub struct T32LOAD1_SPEC;
impl crate::RegisterSpec for T32LOAD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t32load1::R](R) reader structure"]
impl crate::Readable for T32LOAD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t32load1::W](W) writer structure"]
impl crate::Writable for T32LOAD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T32LOAD1 to value 0"]
impl crate::Resettable for T32LOAD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
