#[doc = "Register `CSDCOERCAL0` reader"]
pub struct R(crate::R<CSDCOERCAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSDCOERCAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSDCOERCAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSDCOERCAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSDCOERCAL0` writer"]
pub struct W(crate::W<CSDCOERCAL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSDCOERCAL0_SPEC>;
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
impl From<crate::W<CSDCOERCAL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSDCOERCAL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCO_TCCAL` reader - DCO Temperature compensation calibration"]
pub type DCO_TCCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCO_TCCAL` writer - DCO Temperature compensation calibration"]
pub type DCO_TCCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSDCOERCAL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `DCO_FCAL_RSEL04` reader - DCO frequency calibration for DCO frequency range (DCORSEL) 0 to 4."]
pub type DCO_FCAL_RSEL04_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DCO_FCAL_RSEL04` writer - DCO frequency calibration for DCO frequency range (DCORSEL) 0 to 4."]
pub type DCO_FCAL_RSEL04_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSDCOERCAL0_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:1 - DCO Temperature compensation calibration"]
    #[inline(always)]
    pub fn dco_tccal(&self) -> DCO_TCCAL_R {
        DCO_TCCAL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:25 - DCO frequency calibration for DCO frequency range (DCORSEL) 0 to 4."]
    #[inline(always)]
    pub fn dco_fcal_rsel04(&self) -> DCO_FCAL_RSEL04_R {
        DCO_FCAL_RSEL04_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - DCO Temperature compensation calibration"]
    #[inline(always)]
    pub fn dco_tccal(&mut self) -> DCO_TCCAL_W<0> {
        DCO_TCCAL_W::new(self)
    }
    #[doc = "Bits 16:25 - DCO frequency calibration for DCO frequency range (DCORSEL) 0 to 4."]
    #[inline(always)]
    pub fn dco_fcal_rsel04(&mut self) -> DCO_FCAL_RSEL04_W<16> {
        DCO_FCAL_RSEL04_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCO External Resistor Cailbration 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csdcoercal0](index.html) module"]
pub struct CSDCOERCAL0_SPEC;
impl crate::RegisterSpec for CSDCOERCAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csdcoercal0::R](R) reader structure"]
impl crate::Readable for CSDCOERCAL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csdcoercal0::W](W) writer structure"]
impl crate::Writable for CSDCOERCAL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSDCOERCAL0 to value 0x0100_0000"]
impl crate::Resettable for CSDCOERCAL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
