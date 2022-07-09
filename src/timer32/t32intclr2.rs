#[doc = "Register `T32INTCLR2` writer"]
pub struct W(crate::W<T32INTCLR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T32INTCLR2_SPEC>;
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
impl From<crate::W<T32INTCLR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T32INTCLR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTCLR` writer - Write clears the interrupt output"]
pub type INTCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T32INTCLR2_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Write clears the interrupt output"]
    #[inline(always)]
    pub fn intclr(&mut self) -> INTCLR_W<0> {
        INTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer 2 Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32intclr2](index.html) module"]
pub struct T32INTCLR2_SPEC;
impl crate::RegisterSpec for T32INTCLR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [t32intclr2::W](W) writer structure"]
impl crate::Writable for T32INTCLR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T32INTCLR2 to value 0"]
impl crate::Resettable for T32INTCLR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
