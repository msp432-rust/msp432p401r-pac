#[doc = "Register `FLCTL_RDBRST_CTLSTAT` reader"]
pub struct R(crate::R<FLCTL_RDBRST_CTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_RDBRST_CTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_RDBRST_CTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_RDBRST_CTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_RDBRST_CTLSTAT` writer"]
pub struct W(crate::W<FLCTL_RDBRST_CTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_RDBRST_CTLSTAT_SPEC>;
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
impl From<crate::W<FLCTL_RDBRST_CTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_RDBRST_CTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` writer - Start of burst/compare operation"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_RDBRST_CTLSTAT_SPEC, bool, O>;
#[doc = "Type of memory that burst is carried out on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MEM_TYPE_A {
    #[doc = "0: Main Memory"]
    MEM_TYPE_0 = 0,
    #[doc = "1: Information Memory"]
    MEM_TYPE_1 = 1,
    #[doc = "3: Engineering Memory"]
    MEM_TYPE_3 = 3,
}
impl From<MEM_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MEM_TYPE` reader - Type of memory that burst is carried out on"]
pub type MEM_TYPE_R = crate::FieldReader<u8, MEM_TYPE_A>;
impl MEM_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MEM_TYPE_A> {
        match self.bits {
            0 => Some(MEM_TYPE_A::MEM_TYPE_0),
            1 => Some(MEM_TYPE_A::MEM_TYPE_1),
            3 => Some(MEM_TYPE_A::MEM_TYPE_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MEM_TYPE_0`"]
    #[inline(always)]
    pub fn is_mem_type_0(&self) -> bool {
        *self == MEM_TYPE_A::MEM_TYPE_0
    }
    #[doc = "Checks if the value of the field is `MEM_TYPE_1`"]
    #[inline(always)]
    pub fn is_mem_type_1(&self) -> bool {
        *self == MEM_TYPE_A::MEM_TYPE_1
    }
    #[doc = "Checks if the value of the field is `MEM_TYPE_3`"]
    #[inline(always)]
    pub fn is_mem_type_3(&self) -> bool {
        *self == MEM_TYPE_A::MEM_TYPE_3
    }
}
#[doc = "Field `MEM_TYPE` writer - Type of memory that burst is carried out on"]
pub type MEM_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLCTL_RDBRST_CTLSTAT_SPEC, u8, MEM_TYPE_A, 2, O>;
impl<'a, const O: u8> MEM_TYPE_W<'a, O> {
    #[doc = "Main Memory"]
    #[inline(always)]
    pub fn mem_type_0(self) -> &'a mut W {
        self.variant(MEM_TYPE_A::MEM_TYPE_0)
    }
    #[doc = "Information Memory"]
    #[inline(always)]
    pub fn mem_type_1(self) -> &'a mut W {
        self.variant(MEM_TYPE_A::MEM_TYPE_1)
    }
    #[doc = "Engineering Memory"]
    #[inline(always)]
    pub fn mem_type_3(self) -> &'a mut W {
        self.variant(MEM_TYPE_A::MEM_TYPE_3)
    }
}
#[doc = "Field `STOP_FAIL` reader - Terminate burst/compare operation"]
pub type STOP_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `STOP_FAIL` writer - Terminate burst/compare operation"]
pub type STOP_FAIL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_RDBRST_CTLSTAT_SPEC, bool, O>;
#[doc = "Data pattern used for comparison against memory read data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_CMP_A {
    #[doc = "0: 0000_0000_0000_0000_0000_0000_0000_0000"]
    DATA_CMP_0 = 0,
    #[doc = "1: FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF"]
    DATA_CMP_1 = 1,
}
impl From<DATA_CMP_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_CMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_CMP` reader - Data pattern used for comparison against memory read data"]
pub type DATA_CMP_R = crate::BitReader<DATA_CMP_A>;
impl DATA_CMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_CMP_A {
        match self.bits {
            false => DATA_CMP_A::DATA_CMP_0,
            true => DATA_CMP_A::DATA_CMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_CMP_0`"]
    #[inline(always)]
    pub fn is_data_cmp_0(&self) -> bool {
        *self == DATA_CMP_A::DATA_CMP_0
    }
    #[doc = "Checks if the value of the field is `DATA_CMP_1`"]
    #[inline(always)]
    pub fn is_data_cmp_1(&self) -> bool {
        *self == DATA_CMP_A::DATA_CMP_1
    }
}
#[doc = "Field `DATA_CMP` writer - Data pattern used for comparison against memory read data"]
pub type DATA_CMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_RDBRST_CTLSTAT_SPEC, DATA_CMP_A, O>;
impl<'a, const O: u8> DATA_CMP_W<'a, O> {
    #[doc = "0000_0000_0000_0000_0000_0000_0000_0000"]
    #[inline(always)]
    pub fn data_cmp_0(self) -> &'a mut W {
        self.variant(DATA_CMP_A::DATA_CMP_0)
    }
    #[doc = "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF"]
    #[inline(always)]
    pub fn data_cmp_1(self) -> &'a mut W {
        self.variant(DATA_CMP_A::DATA_CMP_1)
    }
}
#[doc = "Field `TEST_EN` reader - Enable comparison against test data compare registers"]
pub type TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TEST_EN` writer - Enable comparison against test data compare registers"]
pub type TEST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_RDBRST_CTLSTAT_SPEC, bool, O>;
#[doc = "Status of Burst/Compare operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRST_STAT_A {
    #[doc = "0: Idle"]
    BRST_STAT_0 = 0,
    #[doc = "1: Burst/Compare START bit written, but operation pending"]
    BRST_STAT_1 = 1,
    #[doc = "2: Burst/Compare in progress"]
    BRST_STAT_2 = 2,
    #[doc = "3: Burst complete (status of completed burst remains in this state unless explicitly cleared by SW)"]
    BRST_STAT_3 = 3,
}
impl From<BRST_STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: BRST_STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BRST_STAT` reader - Status of Burst/Compare operation"]
pub type BRST_STAT_R = crate::FieldReader<u8, BRST_STAT_A>;
impl BRST_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRST_STAT_A {
        match self.bits {
            0 => BRST_STAT_A::BRST_STAT_0,
            1 => BRST_STAT_A::BRST_STAT_1,
            2 => BRST_STAT_A::BRST_STAT_2,
            3 => BRST_STAT_A::BRST_STAT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BRST_STAT_0`"]
    #[inline(always)]
    pub fn is_brst_stat_0(&self) -> bool {
        *self == BRST_STAT_A::BRST_STAT_0
    }
    #[doc = "Checks if the value of the field is `BRST_STAT_1`"]
    #[inline(always)]
    pub fn is_brst_stat_1(&self) -> bool {
        *self == BRST_STAT_A::BRST_STAT_1
    }
    #[doc = "Checks if the value of the field is `BRST_STAT_2`"]
    #[inline(always)]
    pub fn is_brst_stat_2(&self) -> bool {
        *self == BRST_STAT_A::BRST_STAT_2
    }
    #[doc = "Checks if the value of the field is `BRST_STAT_3`"]
    #[inline(always)]
    pub fn is_brst_stat_3(&self) -> bool {
        *self == BRST_STAT_A::BRST_STAT_3
    }
}
#[doc = "Field `CMP_ERR` reader - Burst/Compare Operation encountered atleast one data"]
pub type CMP_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_ERR` reader - Burst/Compare Operation was terminated due to access to"]
pub type ADDR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_STAT` writer - Clear status bits 19-16 of this register"]
pub type CLR_STAT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_RDBRST_CTLSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 1:2 - Type of memory that burst is carried out on"]
    #[inline(always)]
    pub fn mem_type(&self) -> MEM_TYPE_R {
        MEM_TYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Terminate burst/compare operation"]
    #[inline(always)]
    pub fn stop_fail(&self) -> STOP_FAIL_R {
        STOP_FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data pattern used for comparison against memory read data"]
    #[inline(always)]
    pub fn data_cmp(&self) -> DATA_CMP_R {
        DATA_CMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable comparison against test data compare registers"]
    #[inline(always)]
    pub fn test_en(&self) -> TEST_EN_R {
        TEST_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Status of Burst/Compare operation"]
    #[inline(always)]
    pub fn brst_stat(&self) -> BRST_STAT_R {
        BRST_STAT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Burst/Compare Operation encountered atleast one data"]
    #[inline(always)]
    pub fn cmp_err(&self) -> CMP_ERR_R {
        CMP_ERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Burst/Compare Operation was terminated due to access to"]
    #[inline(always)]
    pub fn addr_err(&self) -> ADDR_ERR_R {
        ADDR_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of burst/compare operation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bits 1:2 - Type of memory that burst is carried out on"]
    #[inline(always)]
    pub fn mem_type(&mut self) -> MEM_TYPE_W<1> {
        MEM_TYPE_W::new(self)
    }
    #[doc = "Bit 3 - Terminate burst/compare operation"]
    #[inline(always)]
    pub fn stop_fail(&mut self) -> STOP_FAIL_W<3> {
        STOP_FAIL_W::new(self)
    }
    #[doc = "Bit 4 - Data pattern used for comparison against memory read data"]
    #[inline(always)]
    pub fn data_cmp(&mut self) -> DATA_CMP_W<4> {
        DATA_CMP_W::new(self)
    }
    #[doc = "Bit 6 - Enable comparison against test data compare registers"]
    #[inline(always)]
    pub fn test_en(&mut self) -> TEST_EN_W<6> {
        TEST_EN_W::new(self)
    }
    #[doc = "Bit 23 - Clear status bits 19-16 of this register"]
    #[inline(always)]
    pub fn clr_stat(&mut self) -> CLR_STAT_W<23> {
        CLR_STAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Burst/Compare Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_rdbrst_ctlstat](index.html) module"]
pub struct FLCTL_RDBRST_CTLSTAT_SPEC;
impl crate::RegisterSpec for FLCTL_RDBRST_CTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_rdbrst_ctlstat::R](R) reader structure"]
impl crate::Readable for FLCTL_RDBRST_CTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_rdbrst_ctlstat::W](W) writer structure"]
impl crate::Writable for FLCTL_RDBRST_CTLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_RDBRST_CTLSTAT to value 0"]
impl crate::Resettable for FLCTL_RDBRST_CTLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
