#[doc = "Register `ADC14LO0` reader"]
pub struct R(crate::R<ADC14LO0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14LO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14LO0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14LO0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC14LO0` writer"]
pub struct W(crate::W<ADC14LO0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC14LO0_SPEC>;
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
impl From<crate::W<ADC14LO0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC14LO0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC14LO0` reader - Low threshold 0"]
pub type ADC14LO0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC14LO0` writer - Low threshold 0"]
pub type ADC14LO0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC14LO0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Low threshold 0"]
    #[inline(always)]
    pub fn adc14lo0(&self) -> ADC14LO0_R {
        ADC14LO0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low threshold 0"]
    #[inline(always)]
    pub fn adc14lo0(&mut self) -> ADC14LO0_W<0> {
        ADC14LO0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Comparator Low Threshold 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14lo0](index.html) module"]
pub struct ADC14LO0_SPEC;
impl crate::RegisterSpec for ADC14LO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14lo0::R](R) reader structure"]
impl crate::Readable for ADC14LO0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc14lo0::W](W) writer structure"]
impl crate::Writable for ADC14LO0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC14LO0 to value 0"]
impl crate::Resettable for ADC14LO0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
