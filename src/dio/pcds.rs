#[doc = "Register `PCDS` reader"]
pub struct R(crate::R<PCDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCDS` writer"]
pub struct W(crate::W<PCDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDS_SPEC>;
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
impl From<crate::W<PCDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5DS` reader - Port 5 Drive Strength"]
pub type P5DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P5DS` writer - Port 5 Drive Strength"]
pub type P5DS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCDS_SPEC, u8, u8, 8, O>;
#[doc = "Field `P6DS` reader - Port 6 Drive Strength"]
pub type P6DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P6DS` writer - Port 6 Drive Strength"]
pub type P6DS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCDS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Drive Strength"]
    #[inline(always)]
    pub fn p5ds(&self) -> P5DS_R {
        P5DS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Drive Strength"]
    #[inline(always)]
    pub fn p6ds(&self) -> P6DS_R {
        P6DS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Drive Strength"]
    #[inline(always)]
    pub fn p5ds(&mut self) -> P5DS_W<0> {
        P5DS_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 6 Drive Strength"]
    #[inline(always)]
    pub fn p6ds(&mut self) -> P6DS_W<8> {
        P6DS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcds](index.html) module"]
pub struct PCDS_SPEC;
impl crate::RegisterSpec for PCDS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcds::R](R) reader structure"]
impl crate::Readable for PCDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcds::W](W) writer structure"]
impl crate::Writable for PCDS_SPEC {
    type Writer = W;
}
