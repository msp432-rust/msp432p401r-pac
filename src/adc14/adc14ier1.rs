#[doc = "Register `ADC14IER1` reader"]
pub struct R(crate::R<ADC14IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14IER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC14IER1` writer"]
pub struct W(crate::W<ADC14IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC14IER1_SPEC>;
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
impl From<crate::W<ADC14IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC14IER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt enable for ADC14MEMx within comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14INIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14INIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14INIE_1 = 1,
}
impl From<ADC14INIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14INIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14INIE` reader - Interrupt enable for ADC14MEMx within comparator window"]
pub type ADC14INIE_R = crate::BitReader<ADC14INIE_A>;
impl ADC14INIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14INIE_A {
        match self.bits {
            false => ADC14INIE_A::ADC14INIE_0,
            true => ADC14INIE_A::ADC14INIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14INIE_0`"]
    #[inline(always)]
    pub fn is_adc14inie_0(&self) -> bool {
        *self == ADC14INIE_A::ADC14INIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14INIE_1`"]
    #[inline(always)]
    pub fn is_adc14inie_1(&self) -> bool {
        *self == ADC14INIE_A::ADC14INIE_1
    }
}
#[doc = "Field `ADC14INIE` writer - Interrupt enable for ADC14MEMx within comparator window"]
pub type ADC14INIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER1_SPEC, ADC14INIE_A, O>;
impl<'a, const O: u8> ADC14INIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14inie_0(self) -> &'a mut W {
        self.variant(ADC14INIE_A::ADC14INIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14inie_1(self) -> &'a mut W {
        self.variant(ADC14INIE_A::ADC14INIE_1)
    }
}
#[doc = "Interrupt enable for ADC14MEMx below comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14LOIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14LOIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14LOIE_1 = 1,
}
impl From<ADC14LOIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14LOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14LOIE` reader - Interrupt enable for ADC14MEMx below comparator window"]
pub type ADC14LOIE_R = crate::BitReader<ADC14LOIE_A>;
impl ADC14LOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14LOIE_A {
        match self.bits {
            false => ADC14LOIE_A::ADC14LOIE_0,
            true => ADC14LOIE_A::ADC14LOIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14LOIE_0`"]
    #[inline(always)]
    pub fn is_adc14loie_0(&self) -> bool {
        *self == ADC14LOIE_A::ADC14LOIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14LOIE_1`"]
    #[inline(always)]
    pub fn is_adc14loie_1(&self) -> bool {
        *self == ADC14LOIE_A::ADC14LOIE_1
    }
}
#[doc = "Field `ADC14LOIE` writer - Interrupt enable for ADC14MEMx below comparator window"]
pub type ADC14LOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER1_SPEC, ADC14LOIE_A, O>;
impl<'a, const O: u8> ADC14LOIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14loie_0(self) -> &'a mut W {
        self.variant(ADC14LOIE_A::ADC14LOIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14loie_1(self) -> &'a mut W {
        self.variant(ADC14LOIE_A::ADC14LOIE_1)
    }
}
#[doc = "Interrupt enable for ADC14MEMx above comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14HIIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14HIIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14HIIE_1 = 1,
}
impl From<ADC14HIIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14HIIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14HIIE` reader - Interrupt enable for ADC14MEMx above comparator window"]
pub type ADC14HIIE_R = crate::BitReader<ADC14HIIE_A>;
impl ADC14HIIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14HIIE_A {
        match self.bits {
            false => ADC14HIIE_A::ADC14HIIE_0,
            true => ADC14HIIE_A::ADC14HIIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14HIIE_0`"]
    #[inline(always)]
    pub fn is_adc14hiie_0(&self) -> bool {
        *self == ADC14HIIE_A::ADC14HIIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14HIIE_1`"]
    #[inline(always)]
    pub fn is_adc14hiie_1(&self) -> bool {
        *self == ADC14HIIE_A::ADC14HIIE_1
    }
}
#[doc = "Field `ADC14HIIE` writer - Interrupt enable for ADC14MEMx above comparator window"]
pub type ADC14HIIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER1_SPEC, ADC14HIIE_A, O>;
impl<'a, const O: u8> ADC14HIIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14hiie_0(self) -> &'a mut W {
        self.variant(ADC14HIIE_A::ADC14HIIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14hiie_1(self) -> &'a mut W {
        self.variant(ADC14HIIE_A::ADC14HIIE_1)
    }
}
#[doc = "ADC14MEMx overflow-interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14OVIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14OVIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14OVIE_1 = 1,
}
impl From<ADC14OVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14OVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14OVIE` reader - ADC14MEMx overflow-interrupt enable"]
pub type ADC14OVIE_R = crate::BitReader<ADC14OVIE_A>;
impl ADC14OVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14OVIE_A {
        match self.bits {
            false => ADC14OVIE_A::ADC14OVIE_0,
            true => ADC14OVIE_A::ADC14OVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14OVIE_0`"]
    #[inline(always)]
    pub fn is_adc14ovie_0(&self) -> bool {
        *self == ADC14OVIE_A::ADC14OVIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14OVIE_1`"]
    #[inline(always)]
    pub fn is_adc14ovie_1(&self) -> bool {
        *self == ADC14OVIE_A::ADC14OVIE_1
    }
}
#[doc = "Field `ADC14OVIE` writer - ADC14MEMx overflow-interrupt enable"]
pub type ADC14OVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER1_SPEC, ADC14OVIE_A, O>;
impl<'a, const O: u8> ADC14OVIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ovie_0(self) -> &'a mut W {
        self.variant(ADC14OVIE_A::ADC14OVIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ovie_1(self) -> &'a mut W {
        self.variant(ADC14OVIE_A::ADC14OVIE_1)
    }
}
#[doc = "ADC14 conversion-time-overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14TOVIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14TOVIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14TOVIE_1 = 1,
}
impl From<ADC14TOVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14TOVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14TOVIE` reader - ADC14 conversion-time-overflow interrupt enable"]
pub type ADC14TOVIE_R = crate::BitReader<ADC14TOVIE_A>;
impl ADC14TOVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14TOVIE_A {
        match self.bits {
            false => ADC14TOVIE_A::ADC14TOVIE_0,
            true => ADC14TOVIE_A::ADC14TOVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14TOVIE_0`"]
    #[inline(always)]
    pub fn is_adc14tovie_0(&self) -> bool {
        *self == ADC14TOVIE_A::ADC14TOVIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14TOVIE_1`"]
    #[inline(always)]
    pub fn is_adc14tovie_1(&self) -> bool {
        *self == ADC14TOVIE_A::ADC14TOVIE_1
    }
}
#[doc = "Field `ADC14TOVIE` writer - ADC14 conversion-time-overflow interrupt enable"]
pub type ADC14TOVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER1_SPEC, ADC14TOVIE_A, O>;
impl<'a, const O: u8> ADC14TOVIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14tovie_0(self) -> &'a mut W {
        self.variant(ADC14TOVIE_A::ADC14TOVIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14tovie_1(self) -> &'a mut W {
        self.variant(ADC14TOVIE_A::ADC14TOVIE_1)
    }
}
#[doc = "ADC14 local buffered reference ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14RDYIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC14RDYIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14RDYIE_1 = 1,
}
impl From<ADC14RDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14RDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14RDYIE` reader - ADC14 local buffered reference ready interrupt enable"]
pub type ADC14RDYIE_R = crate::BitReader<ADC14RDYIE_A>;
impl ADC14RDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14RDYIE_A {
        match self.bits {
            false => ADC14RDYIE_A::ADC14RDYIE_0,
            true => ADC14RDYIE_A::ADC14RDYIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14RDYIE_0`"]
    #[inline(always)]
    pub fn is_adc14rdyie_0(&self) -> bool {
        *self == ADC14RDYIE_A::ADC14RDYIE_0
    }
    #[doc = "Checks if the value of the field is `ADC14RDYIE_1`"]
    #[inline(always)]
    pub fn is_adc14rdyie_1(&self) -> bool {
        *self == ADC14RDYIE_A::ADC14RDYIE_1
    }
}
#[doc = "Field `ADC14RDYIE` writer - ADC14 local buffered reference ready interrupt enable"]
pub type ADC14RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER1_SPEC, ADC14RDYIE_A, O>;
impl<'a, const O: u8> ADC14RDYIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14rdyie_0(self) -> &'a mut W {
        self.variant(ADC14RDYIE_A::ADC14RDYIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14rdyie_1(self) -> &'a mut W {
        self.variant(ADC14RDYIE_A::ADC14RDYIE_1)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt enable for ADC14MEMx within comparator window"]
    #[inline(always)]
    pub fn adc14inie(&self) -> ADC14INIE_R {
        ADC14INIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for ADC14MEMx below comparator window"]
    #[inline(always)]
    pub fn adc14loie(&self) -> ADC14LOIE_R {
        ADC14LOIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for ADC14MEMx above comparator window"]
    #[inline(always)]
    pub fn adc14hiie(&self) -> ADC14HIIE_R {
        ADC14HIIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC14MEMx overflow-interrupt enable"]
    #[inline(always)]
    pub fn adc14ovie(&self) -> ADC14OVIE_R {
        ADC14OVIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC14 conversion-time-overflow interrupt enable"]
    #[inline(always)]
    pub fn adc14tovie(&self) -> ADC14TOVIE_R {
        ADC14TOVIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC14 local buffered reference ready interrupt enable"]
    #[inline(always)]
    pub fn adc14rdyie(&self) -> ADC14RDYIE_R {
        ADC14RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt enable for ADC14MEMx within comparator window"]
    #[inline(always)]
    pub fn adc14inie(&mut self) -> ADC14INIE_W<1> {
        ADC14INIE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable for ADC14MEMx below comparator window"]
    #[inline(always)]
    pub fn adc14loie(&mut self) -> ADC14LOIE_W<2> {
        ADC14LOIE_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt enable for ADC14MEMx above comparator window"]
    #[inline(always)]
    pub fn adc14hiie(&mut self) -> ADC14HIIE_W<3> {
        ADC14HIIE_W::new(self)
    }
    #[doc = "Bit 4 - ADC14MEMx overflow-interrupt enable"]
    #[inline(always)]
    pub fn adc14ovie(&mut self) -> ADC14OVIE_W<4> {
        ADC14OVIE_W::new(self)
    }
    #[doc = "Bit 5 - ADC14 conversion-time-overflow interrupt enable"]
    #[inline(always)]
    pub fn adc14tovie(&mut self) -> ADC14TOVIE_W<5> {
        ADC14TOVIE_W::new(self)
    }
    #[doc = "Bit 6 - ADC14 local buffered reference ready interrupt enable"]
    #[inline(always)]
    pub fn adc14rdyie(&mut self) -> ADC14RDYIE_W<6> {
        ADC14RDYIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14ier1](index.html) module"]
pub struct ADC14IER1_SPEC;
impl crate::RegisterSpec for ADC14IER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14ier1::R](R) reader structure"]
impl crate::Readable for ADC14IER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc14ier1::W](W) writer structure"]
impl crate::Writable for ADC14IER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC14IER1 to value 0"]
impl crate::Resettable for ADC14IER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
