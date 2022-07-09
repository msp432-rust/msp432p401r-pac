#[doc = "Register `PDREN` reader"]
pub struct R(crate::R<PDREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDREN` writer"]
pub struct W(crate::W<PDREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDREN_SPEC>;
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
impl From<crate::W<PDREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7REN` reader - Port 7 Resistor Enable"]
pub type P7REN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P7REN` writer - Port 7 Resistor Enable"]
pub type P7REN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDREN_SPEC, u8, u8, 8, O>;
#[doc = "Field `P8REN` reader - Port 8 Resistor Enable"]
pub type P8REN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P8REN` writer - Port 8 Resistor Enable"]
pub type P8REN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDREN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Resistor Enable"]
    #[inline(always)]
    pub fn p7ren(&self) -> P7REN_R {
        P7REN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Resistor Enable"]
    #[inline(always)]
    pub fn p8ren(&self) -> P8REN_R {
        P8REN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Resistor Enable"]
    #[inline(always)]
    pub fn p7ren(&mut self) -> P7REN_W<0> {
        P7REN_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 8 Resistor Enable"]
    #[inline(always)]
    pub fn p8ren(&mut self) -> P8REN_W<8> {
        P8REN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port D Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdren](index.html) module"]
pub struct PDREN_SPEC;
impl crate::RegisterSpec for PDREN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdren::R](R) reader structure"]
impl crate::Readable for PDREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdren::W](W) writer structure"]
impl crate::Writable for PDREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDREN to value 0"]
impl crate::Resettable for PDREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
