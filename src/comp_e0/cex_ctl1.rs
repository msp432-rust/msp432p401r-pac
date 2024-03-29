#[doc = "Register `CExCTL1` reader"]
pub struct R(crate::R<CEXCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEXCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEXCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEXCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CExCTL1` writer"]
pub struct W(crate::W<CEXCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEXCTL1_SPEC>;
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
impl From<crate::W<CEXCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEXCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEOUT` reader - Comparator output value"]
pub type CEOUT_R = crate::BitReader<bool>;
#[doc = "Field `CEOUT` writer - Comparator output value"]
pub type CEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL1_SPEC, bool, O>;
#[doc = "Comparator output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEOUTPOL_A {
    #[doc = "0: Noninverted"]
    CEOUTPOL_0 = 0,
    #[doc = "1: Inverted"]
    CEOUTPOL_1 = 1,
}
impl From<CEOUTPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CEOUTPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEOUTPOL` reader - Comparator output polarity"]
pub type CEOUTPOL_R = crate::BitReader<CEOUTPOL_A>;
impl CEOUTPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEOUTPOL_A {
        match self.bits {
            false => CEOUTPOL_A::CEOUTPOL_0,
            true => CEOUTPOL_A::CEOUTPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEOUTPOL_0`"]
    #[inline(always)]
    pub fn is_ceoutpol_0(&self) -> bool {
        *self == CEOUTPOL_A::CEOUTPOL_0
    }
    #[doc = "Checks if the value of the field is `CEOUTPOL_1`"]
    #[inline(always)]
    pub fn is_ceoutpol_1(&self) -> bool {
        *self == CEOUTPOL_A::CEOUTPOL_1
    }
}
#[doc = "Field `CEOUTPOL` writer - Comparator output polarity"]
pub type CEOUTPOL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL1_SPEC, CEOUTPOL_A, O>;
impl<'a, const O: u8> CEOUTPOL_W<'a, O> {
    #[doc = "Noninverted"]
    #[inline(always)]
    pub fn ceoutpol_0(self) -> &'a mut W {
        self.variant(CEOUTPOL_A::CEOUTPOL_0)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn ceoutpol_1(self) -> &'a mut W {
        self.variant(CEOUTPOL_A::CEOUTPOL_1)
    }
}
#[doc = "Comparator output filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEF_A {
    #[doc = "0: Comparator output is not filtered"]
    CEF_0 = 0,
    #[doc = "1: Comparator output is filtered"]
    CEF_1 = 1,
}
impl From<CEF_A> for bool {
    #[inline(always)]
    fn from(variant: CEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEF` reader - Comparator output filter"]
pub type CEF_R = crate::BitReader<CEF_A>;
impl CEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEF_A {
        match self.bits {
            false => CEF_A::CEF_0,
            true => CEF_A::CEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEF_0`"]
    #[inline(always)]
    pub fn is_cef_0(&self) -> bool {
        *self == CEF_A::CEF_0
    }
    #[doc = "Checks if the value of the field is `CEF_1`"]
    #[inline(always)]
    pub fn is_cef_1(&self) -> bool {
        *self == CEF_A::CEF_1
    }
}
#[doc = "Field `CEF` writer - Comparator output filter"]
pub type CEF_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL1_SPEC, CEF_A, O>;
impl<'a, const O: u8> CEF_W<'a, O> {
    #[doc = "Comparator output is not filtered"]
    #[inline(always)]
    pub fn cef_0(self) -> &'a mut W {
        self.variant(CEF_A::CEF_0)
    }
    #[doc = "Comparator output is filtered"]
    #[inline(always)]
    pub fn cef_1(self) -> &'a mut W {
        self.variant(CEF_A::CEF_1)
    }
}
#[doc = "Interrupt edge select for CEIIFG and CEIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEIES_A {
    #[doc = "0: Rising edge for CEIFG, falling edge for CEIIFG"]
    CEIES_0 = 0,
    #[doc = "1: Falling edge for CEIFG, rising edge for CEIIFG"]
    CEIES_1 = 1,
}
impl From<CEIES_A> for bool {
    #[inline(always)]
    fn from(variant: CEIES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIES` reader - Interrupt edge select for CEIIFG and CEIFG"]
pub type CEIES_R = crate::BitReader<CEIES_A>;
impl CEIES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIES_A {
        match self.bits {
            false => CEIES_A::CEIES_0,
            true => CEIES_A::CEIES_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEIES_0`"]
    #[inline(always)]
    pub fn is_ceies_0(&self) -> bool {
        *self == CEIES_A::CEIES_0
    }
    #[doc = "Checks if the value of the field is `CEIES_1`"]
    #[inline(always)]
    pub fn is_ceies_1(&self) -> bool {
        *self == CEIES_A::CEIES_1
    }
}
#[doc = "Field `CEIES` writer - Interrupt edge select for CEIIFG and CEIFG"]
pub type CEIES_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL1_SPEC, CEIES_A, O>;
impl<'a, const O: u8> CEIES_W<'a, O> {
    #[doc = "Rising edge for CEIFG, falling edge for CEIIFG"]
    #[inline(always)]
    pub fn ceies_0(self) -> &'a mut W {
        self.variant(CEIES_A::CEIES_0)
    }
    #[doc = "Falling edge for CEIFG, rising edge for CEIIFG"]
    #[inline(always)]
    pub fn ceies_1(self) -> &'a mut W {
        self.variant(CEIES_A::CEIES_1)
    }
}
#[doc = "Input short\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CESHORT_A {
    #[doc = "0: Inputs not shorted"]
    CESHORT_0 = 0,
    #[doc = "1: Inputs shorted"]
    CESHORT_1 = 1,
}
impl From<CESHORT_A> for bool {
    #[inline(always)]
    fn from(variant: CESHORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CESHORT` reader - Input short"]
pub type CESHORT_R = crate::BitReader<CESHORT_A>;
impl CESHORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CESHORT_A {
        match self.bits {
            false => CESHORT_A::CESHORT_0,
            true => CESHORT_A::CESHORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CESHORT_0`"]
    #[inline(always)]
    pub fn is_ceshort_0(&self) -> bool {
        *self == CESHORT_A::CESHORT_0
    }
    #[doc = "Checks if the value of the field is `CESHORT_1`"]
    #[inline(always)]
    pub fn is_ceshort_1(&self) -> bool {
        *self == CESHORT_A::CESHORT_1
    }
}
#[doc = "Field `CESHORT` writer - Input short"]
pub type CESHORT_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL1_SPEC, CESHORT_A, O>;
impl<'a, const O: u8> CESHORT_W<'a, O> {
    #[doc = "Inputs not shorted"]
    #[inline(always)]
    pub fn ceshort_0(self) -> &'a mut W {
        self.variant(CESHORT_A::CESHORT_0)
    }
    #[doc = "Inputs shorted"]
    #[inline(always)]
    pub fn ceshort_1(self) -> &'a mut W {
        self.variant(CESHORT_A::CESHORT_1)
    }
}
#[doc = "Field `CEEX` reader - Exchange"]
pub type CEEX_R = crate::BitReader<bool>;
#[doc = "Field `CEEX` writer - Exchange"]
pub type CEEX_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL1_SPEC, bool, O>;
#[doc = "Filter delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEFDLY_A {
    #[doc = "0: Typical filter delay of TBD (450) ns"]
    CEFDLY_0 = 0,
    #[doc = "1: Typical filter delay of TBD (900) ns"]
    CEFDLY_1 = 1,
    #[doc = "2: Typical filter delay of TBD (1800) ns"]
    CEFDLY_2 = 2,
    #[doc = "3: Typical filter delay of TBD (3600) ns"]
    CEFDLY_3 = 3,
}
impl From<CEFDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: CEFDLY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CEFDLY` reader - Filter delay"]
pub type CEFDLY_R = crate::FieldReader<u8, CEFDLY_A>;
impl CEFDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEFDLY_A {
        match self.bits {
            0 => CEFDLY_A::CEFDLY_0,
            1 => CEFDLY_A::CEFDLY_1,
            2 => CEFDLY_A::CEFDLY_2,
            3 => CEFDLY_A::CEFDLY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEFDLY_0`"]
    #[inline(always)]
    pub fn is_cefdly_0(&self) -> bool {
        *self == CEFDLY_A::CEFDLY_0
    }
    #[doc = "Checks if the value of the field is `CEFDLY_1`"]
    #[inline(always)]
    pub fn is_cefdly_1(&self) -> bool {
        *self == CEFDLY_A::CEFDLY_1
    }
    #[doc = "Checks if the value of the field is `CEFDLY_2`"]
    #[inline(always)]
    pub fn is_cefdly_2(&self) -> bool {
        *self == CEFDLY_A::CEFDLY_2
    }
    #[doc = "Checks if the value of the field is `CEFDLY_3`"]
    #[inline(always)]
    pub fn is_cefdly_3(&self) -> bool {
        *self == CEFDLY_A::CEFDLY_3
    }
}
#[doc = "Field `CEFDLY` writer - Filter delay"]
pub type CEFDLY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CEXCTL1_SPEC, u8, CEFDLY_A, 2, O>;
impl<'a, const O: u8> CEFDLY_W<'a, O> {
    #[doc = "Typical filter delay of TBD (450) ns"]
    #[inline(always)]
    pub fn cefdly_0(self) -> &'a mut W {
        self.variant(CEFDLY_A::CEFDLY_0)
    }
    #[doc = "Typical filter delay of TBD (900) ns"]
    #[inline(always)]
    pub fn cefdly_1(self) -> &'a mut W {
        self.variant(CEFDLY_A::CEFDLY_1)
    }
    #[doc = "Typical filter delay of TBD (1800) ns"]
    #[inline(always)]
    pub fn cefdly_2(self) -> &'a mut W {
        self.variant(CEFDLY_A::CEFDLY_2)
    }
    #[doc = "Typical filter delay of TBD (3600) ns"]
    #[inline(always)]
    pub fn cefdly_3(self) -> &'a mut W {
        self.variant(CEFDLY_A::CEFDLY_3)
    }
}
#[doc = "Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEPWRMD_A {
    #[doc = "0: High-speed mode"]
    CEPWRMD_0 = 0,
    #[doc = "1: Normal mode"]
    CEPWRMD_1 = 1,
    #[doc = "2: Ultra-low power mode"]
    CEPWRMD_2 = 2,
}
impl From<CEPWRMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CEPWRMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CEPWRMD` reader - Power Mode"]
pub type CEPWRMD_R = crate::FieldReader<u8, CEPWRMD_A>;
impl CEPWRMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CEPWRMD_A> {
        match self.bits {
            0 => Some(CEPWRMD_A::CEPWRMD_0),
            1 => Some(CEPWRMD_A::CEPWRMD_1),
            2 => Some(CEPWRMD_A::CEPWRMD_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CEPWRMD_0`"]
    #[inline(always)]
    pub fn is_cepwrmd_0(&self) -> bool {
        *self == CEPWRMD_A::CEPWRMD_0
    }
    #[doc = "Checks if the value of the field is `CEPWRMD_1`"]
    #[inline(always)]
    pub fn is_cepwrmd_1(&self) -> bool {
        *self == CEPWRMD_A::CEPWRMD_1
    }
    #[doc = "Checks if the value of the field is `CEPWRMD_2`"]
    #[inline(always)]
    pub fn is_cepwrmd_2(&self) -> bool {
        *self == CEPWRMD_A::CEPWRMD_2
    }
}
#[doc = "Field `CEPWRMD` writer - Power Mode"]
pub type CEPWRMD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CEXCTL1_SPEC, u8, CEPWRMD_A, 2, O>;
impl<'a, const O: u8> CEPWRMD_W<'a, O> {
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn cepwrmd_0(self) -> &'a mut W {
        self.variant(CEPWRMD_A::CEPWRMD_0)
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn cepwrmd_1(self) -> &'a mut W {
        self.variant(CEPWRMD_A::CEPWRMD_1)
    }
    #[doc = "Ultra-low power mode"]
    #[inline(always)]
    pub fn cepwrmd_2(self) -> &'a mut W {
        self.variant(CEPWRMD_A::CEPWRMD_2)
    }
}
#[doc = "Comparator On\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEON_A {
    #[doc = "0: Off"]
    CEON_0 = 0,
    #[doc = "1: On"]
    CEON_1 = 1,
}
impl From<CEON_A> for bool {
    #[inline(always)]
    fn from(variant: CEON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEON` reader - Comparator On"]
pub type CEON_R = crate::BitReader<CEON_A>;
impl CEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEON_A {
        match self.bits {
            false => CEON_A::CEON_0,
            true => CEON_A::CEON_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEON_0`"]
    #[inline(always)]
    pub fn is_ceon_0(&self) -> bool {
        *self == CEON_A::CEON_0
    }
    #[doc = "Checks if the value of the field is `CEON_1`"]
    #[inline(always)]
    pub fn is_ceon_1(&self) -> bool {
        *self == CEON_A::CEON_1
    }
}
#[doc = "Field `CEON` writer - Comparator On"]
pub type CEON_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL1_SPEC, CEON_A, O>;
impl<'a, const O: u8> CEON_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn ceon_0(self) -> &'a mut W {
        self.variant(CEON_A::CEON_0)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ceon_1(self) -> &'a mut W {
        self.variant(CEON_A::CEON_1)
    }
}
#[doc = "This bit is valid of CEMRVS is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEMRVL_A {
    #[doc = "0: VREF0 is selected if CERS = 00, 01, or 10"]
    CEMRVL_0 = 0,
    #[doc = "1: VREF1 is selected if CERS = 00, 01, or 10"]
    CEMRVL_1 = 1,
}
impl From<CEMRVL_A> for bool {
    #[inline(always)]
    fn from(variant: CEMRVL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEMRVL` reader - This bit is valid of CEMRVS is set to 1"]
pub type CEMRVL_R = crate::BitReader<CEMRVL_A>;
impl CEMRVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEMRVL_A {
        match self.bits {
            false => CEMRVL_A::CEMRVL_0,
            true => CEMRVL_A::CEMRVL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEMRVL_0`"]
    #[inline(always)]
    pub fn is_cemrvl_0(&self) -> bool {
        *self == CEMRVL_A::CEMRVL_0
    }
    #[doc = "Checks if the value of the field is `CEMRVL_1`"]
    #[inline(always)]
    pub fn is_cemrvl_1(&self) -> bool {
        *self == CEMRVL_A::CEMRVL_1
    }
}
#[doc = "Field `CEMRVL` writer - This bit is valid of CEMRVS is set to 1"]
pub type CEMRVL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL1_SPEC, CEMRVL_A, O>;
impl<'a, const O: u8> CEMRVL_W<'a, O> {
    #[doc = "VREF0 is selected if CERS = 00, 01, or 10"]
    #[inline(always)]
    pub fn cemrvl_0(self) -> &'a mut W {
        self.variant(CEMRVL_A::CEMRVL_0)
    }
    #[doc = "VREF1 is selected if CERS = 00, 01, or 10"]
    #[inline(always)]
    pub fn cemrvl_1(self) -> &'a mut W {
        self.variant(CEMRVL_A::CEMRVL_1)
    }
}
#[doc = "This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEMRVS_A {
    #[doc = "0: Comparator output state selects between VREF0 or VREF1"]
    CEMRVS_0 = 0,
    #[doc = "1: CEMRVL selects between VREF0 or VREF1"]
    CEMRVS_1 = 1,
}
impl From<CEMRVS_A> for bool {
    #[inline(always)]
    fn from(variant: CEMRVS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEMRVS` reader - This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10."]
pub type CEMRVS_R = crate::BitReader<CEMRVS_A>;
impl CEMRVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEMRVS_A {
        match self.bits {
            false => CEMRVS_A::CEMRVS_0,
            true => CEMRVS_A::CEMRVS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEMRVS_0`"]
    #[inline(always)]
    pub fn is_cemrvs_0(&self) -> bool {
        *self == CEMRVS_A::CEMRVS_0
    }
    #[doc = "Checks if the value of the field is `CEMRVS_1`"]
    #[inline(always)]
    pub fn is_cemrvs_1(&self) -> bool {
        *self == CEMRVS_A::CEMRVS_1
    }
}
#[doc = "Field `CEMRVS` writer - This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10."]
pub type CEMRVS_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEXCTL1_SPEC, CEMRVS_A, O>;
impl<'a, const O: u8> CEMRVS_W<'a, O> {
    #[doc = "Comparator output state selects between VREF0 or VREF1"]
    #[inline(always)]
    pub fn cemrvs_0(self) -> &'a mut W {
        self.variant(CEMRVS_A::CEMRVS_0)
    }
    #[doc = "CEMRVL selects between VREF0 or VREF1"]
    #[inline(always)]
    pub fn cemrvs_1(self) -> &'a mut W {
        self.variant(CEMRVS_A::CEMRVS_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator output value"]
    #[inline(always)]
    pub fn ceout(&self) -> CEOUT_R {
        CEOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator output polarity"]
    #[inline(always)]
    pub fn ceoutpol(&self) -> CEOUTPOL_R {
        CEOUTPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator output filter"]
    #[inline(always)]
    pub fn cef(&self) -> CEF_R {
        CEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt edge select for CEIIFG and CEIFG"]
    #[inline(always)]
    pub fn ceies(&self) -> CEIES_R {
        CEIES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input short"]
    #[inline(always)]
    pub fn ceshort(&self) -> CESHORT_R {
        CESHORT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Exchange"]
    #[inline(always)]
    pub fn ceex(&self) -> CEEX_R {
        CEEX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Filter delay"]
    #[inline(always)]
    pub fn cefdly(&self) -> CEFDLY_R {
        CEFDLY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Power Mode"]
    #[inline(always)]
    pub fn cepwrmd(&self) -> CEPWRMD_R {
        CEPWRMD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Comparator On"]
    #[inline(always)]
    pub fn ceon(&self) -> CEON_R {
        CEON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit is valid of CEMRVS is set to 1"]
    #[inline(always)]
    pub fn cemrvl(&self) -> CEMRVL_R {
        CEMRVL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10."]
    #[inline(always)]
    pub fn cemrvs(&self) -> CEMRVS_R {
        CEMRVS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator output value"]
    #[inline(always)]
    pub fn ceout(&mut self) -> CEOUT_W<0> {
        CEOUT_W::new(self)
    }
    #[doc = "Bit 1 - Comparator output polarity"]
    #[inline(always)]
    pub fn ceoutpol(&mut self) -> CEOUTPOL_W<1> {
        CEOUTPOL_W::new(self)
    }
    #[doc = "Bit 2 - Comparator output filter"]
    #[inline(always)]
    pub fn cef(&mut self) -> CEF_W<2> {
        CEF_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt edge select for CEIIFG and CEIFG"]
    #[inline(always)]
    pub fn ceies(&mut self) -> CEIES_W<3> {
        CEIES_W::new(self)
    }
    #[doc = "Bit 4 - Input short"]
    #[inline(always)]
    pub fn ceshort(&mut self) -> CESHORT_W<4> {
        CESHORT_W::new(self)
    }
    #[doc = "Bit 5 - Exchange"]
    #[inline(always)]
    pub fn ceex(&mut self) -> CEEX_W<5> {
        CEEX_W::new(self)
    }
    #[doc = "Bits 6:7 - Filter delay"]
    #[inline(always)]
    pub fn cefdly(&mut self) -> CEFDLY_W<6> {
        CEFDLY_W::new(self)
    }
    #[doc = "Bits 8:9 - Power Mode"]
    #[inline(always)]
    pub fn cepwrmd(&mut self) -> CEPWRMD_W<8> {
        CEPWRMD_W::new(self)
    }
    #[doc = "Bit 10 - Comparator On"]
    #[inline(always)]
    pub fn ceon(&mut self) -> CEON_W<10> {
        CEON_W::new(self)
    }
    #[doc = "Bit 11 - This bit is valid of CEMRVS is set to 1"]
    #[inline(always)]
    pub fn cemrvl(&mut self) -> CEMRVL_W<11> {
        CEMRVL_W::new(self)
    }
    #[doc = "Bit 12 - This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10."]
    #[inline(always)]
    pub fn cemrvs(&mut self) -> CEMRVS_W<12> {
        CEMRVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_ctl1](index.html) module"]
pub struct CEXCTL1_SPEC;
impl crate::RegisterSpec for CEXCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cex_ctl1::R](R) reader structure"]
impl crate::Readable for CEXCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cex_ctl1::W](W) writer structure"]
impl crate::Writable for CEXCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CExCTL1 to value 0"]
impl crate::Resettable for CEXCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
