#[doc = "Register `P7IV` reader"]
pub struct R(crate::R<P7IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P7IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P7IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P7IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Port 7 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P7IV_A {
    #[doc = "0: No interrupt pending"]
    P7IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 7.0 interrupt; Interrupt Flag: P7IFG0; Interrupt Priority: Highest"]
    P7IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 7.1 interrupt; Interrupt Flag: P7IFG1"]
    P7IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 7.2 interrupt; Interrupt Flag: P7IFG2"]
    P7IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 7.3 interrupt; Interrupt Flag: P7IFG3"]
    P7IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 7.4 interrupt; Interrupt Flag: P7IFG4"]
    P7IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 7.5 interrupt; Interrupt Flag: P7IFG5"]
    P7IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 7.6 interrupt; Interrupt Flag: P7IFG6"]
    P7IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 7.7 interrupt; Interrupt Flag: P7IFG7; Interrupt Priority: Lowest"]
    P7IV_16 = 16,
}
impl From<P7IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P7IV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `P7IV` reader - Port 7 interrupt vector value"]
pub type P7IV_R = crate::FieldReader<u8, P7IV_A>;
impl P7IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P7IV_A> {
        match self.bits {
            0 => Some(P7IV_A::P7IV_0),
            2 => Some(P7IV_A::P7IV_2),
            4 => Some(P7IV_A::P7IV_4),
            6 => Some(P7IV_A::P7IV_6),
            8 => Some(P7IV_A::P7IV_8),
            10 => Some(P7IV_A::P7IV_10),
            12 => Some(P7IV_A::P7IV_12),
            14 => Some(P7IV_A::P7IV_14),
            16 => Some(P7IV_A::P7IV_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P7IV_0`"]
    #[inline(always)]
    pub fn is_p7iv_0(&self) -> bool {
        *self == P7IV_A::P7IV_0
    }
    #[doc = "Checks if the value of the field is `P7IV_2`"]
    #[inline(always)]
    pub fn is_p7iv_2(&self) -> bool {
        *self == P7IV_A::P7IV_2
    }
    #[doc = "Checks if the value of the field is `P7IV_4`"]
    #[inline(always)]
    pub fn is_p7iv_4(&self) -> bool {
        *self == P7IV_A::P7IV_4
    }
    #[doc = "Checks if the value of the field is `P7IV_6`"]
    #[inline(always)]
    pub fn is_p7iv_6(&self) -> bool {
        *self == P7IV_A::P7IV_6
    }
    #[doc = "Checks if the value of the field is `P7IV_8`"]
    #[inline(always)]
    pub fn is_p7iv_8(&self) -> bool {
        *self == P7IV_A::P7IV_8
    }
    #[doc = "Checks if the value of the field is `P7IV_10`"]
    #[inline(always)]
    pub fn is_p7iv_10(&self) -> bool {
        *self == P7IV_A::P7IV_10
    }
    #[doc = "Checks if the value of the field is `P7IV_12`"]
    #[inline(always)]
    pub fn is_p7iv_12(&self) -> bool {
        *self == P7IV_A::P7IV_12
    }
    #[doc = "Checks if the value of the field is `P7IV_14`"]
    #[inline(always)]
    pub fn is_p7iv_14(&self) -> bool {
        *self == P7IV_A::P7IV_14
    }
    #[doc = "Checks if the value of the field is `P7IV_16`"]
    #[inline(always)]
    pub fn is_p7iv_16(&self) -> bool {
        *self == P7IV_A::P7IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 7 interrupt vector value"]
    #[inline(always)]
    pub fn p7iv(&self) -> P7IV_R {
        P7IV_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 7 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7iv](index.html) module"]
pub struct P7IV_SPEC;
impl crate::RegisterSpec for P7IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p7iv::R](R) reader structure"]
impl crate::Readable for P7IV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets P7IV to value 0"]
impl crate::Resettable for P7IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
