#[doc = "Register `P6MAP01` reader"]
pub struct R(crate::R<P6MAP01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P6MAP01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P6MAP01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P6MAP01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P6MAP01` writer"]
pub struct W(crate::W<P6MAP01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P6MAP01_SPEC>;
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
impl From<crate::W<P6MAP01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P6MAP01_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMAPx` reader - Selects secondary port function"]
pub type PMAPX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PMAPx` writer - Selects secondary port function"]
pub type PMAPX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, P6MAP01_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Selects secondary port function"]
    #[inline(always)]
    pub fn pmapx(&self) -> PMAPX_R {
        PMAPX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Selects secondary port function"]
    #[inline(always)]
    pub fn pmapx(&mut self) -> PMAPX_W<0> {
        PMAPX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port mapping register, P6.0 and P6.1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6map01](index.html) module"]
pub struct P6MAP01_SPEC;
impl crate::RegisterSpec for P6MAP01_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p6map01::R](R) reader structure"]
impl crate::Readable for P6MAP01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p6map01::W](W) writer structure"]
impl crate::Writable for P6MAP01_SPEC {
    type Writer = W;
}
