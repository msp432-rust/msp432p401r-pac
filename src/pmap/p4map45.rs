#[doc = "Register `P4MAP45` reader"]
pub struct R(crate::R<P4MAP45_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P4MAP45_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P4MAP45_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P4MAP45_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P4MAP45` writer"]
pub struct W(crate::W<P4MAP45_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P4MAP45_SPEC>;
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
impl From<crate::W<P4MAP45_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P4MAP45_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMAPx` reader - Selects secondary port function"]
pub type PMAPX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PMAPx` writer - Selects secondary port function"]
pub type PMAPX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, P4MAP45_SPEC, u16, u16, 16, O>;
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
#[doc = "Port mapping register, P4.4 and P4.5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4map45](index.html) module"]
pub struct P4MAP45_SPEC;
impl crate::RegisterSpec for P4MAP45_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p4map45::R](R) reader structure"]
impl crate::Readable for P4MAP45_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p4map45::W](W) writer structure"]
impl crate::Writable for P4MAP45_SPEC {
    type Writer = W;
}
