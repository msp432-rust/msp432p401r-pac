#[doc = "Register `PJDS` reader"]
pub struct R(crate::R<PJDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJDS` writer"]
pub struct W(crate::W<PJDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJDS_SPEC>;
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
impl From<crate::W<PJDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJDS` reader - Port J Drive Strength"]
pub type PJDS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PJDS` writer - Port J Drive Strength"]
pub type PJDS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PJDS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Port J Drive Strength"]
    #[inline(always)]
    pub fn pjds(&self) -> PJDS_R {
        PJDS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Drive Strength"]
    #[inline(always)]
    pub fn pjds(&mut self) -> PJDS_W<0> {
        PJDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjds](index.html) module"]
pub struct PJDS_SPEC;
impl crate::RegisterSpec for PJDS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjds::R](R) reader structure"]
impl crate::Readable for PJDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjds::W](W) writer structure"]
impl crate::Writable for PJDS_SPEC {
    type Writer = W;
}
