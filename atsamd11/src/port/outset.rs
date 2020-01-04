#[doc = "Reader of register OUTSET%s"]
pub type R = crate::R<u32, super::OUTSET>;
#[doc = "Writer for register OUTSET%s"]
pub type W = crate::W<u32, super::OUTSET>;
#[doc = "Register OUTSET%s `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Output Value Set 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET0_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET0_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET0`"]
pub type OUTSET0_R = crate::R<bool, OUTSET0_A>;
impl OUTSET0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET0_A {
        match self.bits {
            false => OUTSET0_A::NOP,
            true => OUTSET0_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET0_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET0_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET0`"]
pub struct OUTSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET0_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET0_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Port Data Output Value Set 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET1_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET1_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET1`"]
pub type OUTSET1_R = crate::R<bool, OUTSET1_A>;
impl OUTSET1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET1_A {
        match self.bits {
            false => OUTSET1_A::NOP,
            true => OUTSET1_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET1_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET1_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET1`"]
pub struct OUTSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET1_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET1_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Port Data Output Value Set 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET2_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET2_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET2`"]
pub type OUTSET2_R = crate::R<bool, OUTSET2_A>;
impl OUTSET2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET2_A {
        match self.bits {
            false => OUTSET2_A::NOP,
            true => OUTSET2_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET2_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET2_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET2`"]
pub struct OUTSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET2_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET2_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Port Data Output Value Set 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET3_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET3_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET3`"]
pub type OUTSET3_R = crate::R<bool, OUTSET3_A>;
impl OUTSET3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET3_A {
        match self.bits {
            false => OUTSET3_A::NOP,
            true => OUTSET3_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET3_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET3_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET3`"]
pub struct OUTSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET3_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET3_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Port Data Output Value Set 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET4_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET4_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET4`"]
pub type OUTSET4_R = crate::R<bool, OUTSET4_A>;
impl OUTSET4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET4_A {
        match self.bits {
            false => OUTSET4_A::NOP,
            true => OUTSET4_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET4_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET4_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET4`"]
pub struct OUTSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET4_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET4_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Port Data Output Value Set 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET5_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET5_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET5`"]
pub type OUTSET5_R = crate::R<bool, OUTSET5_A>;
impl OUTSET5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET5_A {
        match self.bits {
            false => OUTSET5_A::NOP,
            true => OUTSET5_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET5_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET5_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET5`"]
pub struct OUTSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET5_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET5_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Port Data Output Value Set 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET6_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET6_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET6`"]
pub type OUTSET6_R = crate::R<bool, OUTSET6_A>;
impl OUTSET6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET6_A {
        match self.bits {
            false => OUTSET6_A::NOP,
            true => OUTSET6_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET6_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET6_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET6`"]
pub struct OUTSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET6_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET6_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Port Data Output Value Set 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET7_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET7_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET7`"]
pub type OUTSET7_R = crate::R<bool, OUTSET7_A>;
impl OUTSET7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET7_A {
        match self.bits {
            false => OUTSET7_A::NOP,
            true => OUTSET7_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET7_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET7_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET7`"]
pub struct OUTSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET7_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET7_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Port Data Output Value Set 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET8_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET8_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET8`"]
pub type OUTSET8_R = crate::R<bool, OUTSET8_A>;
impl OUTSET8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET8_A {
        match self.bits {
            false => OUTSET8_A::NOP,
            true => OUTSET8_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET8_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET8_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET8`"]
pub struct OUTSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET8_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET8_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Port Data Output Value Set 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET9_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET9_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET9`"]
pub type OUTSET9_R = crate::R<bool, OUTSET9_A>;
impl OUTSET9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET9_A {
        match self.bits {
            false => OUTSET9_A::NOP,
            true => OUTSET9_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET9_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET9_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET9`"]
pub struct OUTSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET9_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET9_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Port Data Output Value Set 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET10_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET10_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET10`"]
pub type OUTSET10_R = crate::R<bool, OUTSET10_A>;
impl OUTSET10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET10_A {
        match self.bits {
            false => OUTSET10_A::NOP,
            true => OUTSET10_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET10_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET10_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET10`"]
pub struct OUTSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET10_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET10_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Port Data Output Value Set 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET11_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET11_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET11`"]
pub type OUTSET11_R = crate::R<bool, OUTSET11_A>;
impl OUTSET11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET11_A {
        match self.bits {
            false => OUTSET11_A::NOP,
            true => OUTSET11_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET11_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET11_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET11`"]
pub struct OUTSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET11_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET11_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Port Data Output Value Set 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET12_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET12_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET12`"]
pub type OUTSET12_R = crate::R<bool, OUTSET12_A>;
impl OUTSET12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET12_A {
        match self.bits {
            false => OUTSET12_A::NOP,
            true => OUTSET12_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET12_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET12_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET12`"]
pub struct OUTSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET12_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET12_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Port Data Output Value Set 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET13_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET13_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET13`"]
pub type OUTSET13_R = crate::R<bool, OUTSET13_A>;
impl OUTSET13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET13_A {
        match self.bits {
            false => OUTSET13_A::NOP,
            true => OUTSET13_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET13_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET13_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET13`"]
pub struct OUTSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET13_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET13_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Port Data Output Value Set 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET14_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET14_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET14`"]
pub type OUTSET14_R = crate::R<bool, OUTSET14_A>;
impl OUTSET14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET14_A {
        match self.bits {
            false => OUTSET14_A::NOP,
            true => OUTSET14_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET14_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET14_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET14`"]
pub struct OUTSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET14_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET14_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Port Data Output Value Set 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET15_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET15_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET15`"]
pub type OUTSET15_R = crate::R<bool, OUTSET15_A>;
impl OUTSET15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET15_A {
        match self.bits {
            false => OUTSET15_A::NOP,
            true => OUTSET15_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET15_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET15_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET15`"]
pub struct OUTSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET15_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET15_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Port Data Output Value Set 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET16_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET16_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET16`"]
pub type OUTSET16_R = crate::R<bool, OUTSET16_A>;
impl OUTSET16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET16_A {
        match self.bits {
            false => OUTSET16_A::NOP,
            true => OUTSET16_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET16_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET16_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET16`"]
pub struct OUTSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET16_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET16_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Port Data Output Value Set 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET17_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET17_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET17`"]
pub type OUTSET17_R = crate::R<bool, OUTSET17_A>;
impl OUTSET17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET17_A {
        match self.bits {
            false => OUTSET17_A::NOP,
            true => OUTSET17_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET17_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET17_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET17`"]
pub struct OUTSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET17_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET17_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Port Data Output Value Set 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET18_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET18_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET18`"]
pub type OUTSET18_R = crate::R<bool, OUTSET18_A>;
impl OUTSET18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET18_A {
        match self.bits {
            false => OUTSET18_A::NOP,
            true => OUTSET18_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET18_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET18_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET18`"]
pub struct OUTSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET18_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET18_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Port Data Output Value Set 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET19_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET19_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET19`"]
pub type OUTSET19_R = crate::R<bool, OUTSET19_A>;
impl OUTSET19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET19_A {
        match self.bits {
            false => OUTSET19_A::NOP,
            true => OUTSET19_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET19_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET19_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET19`"]
pub struct OUTSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET19_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET19_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Port Data Output Value Set 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET20_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET20_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET20`"]
pub type OUTSET20_R = crate::R<bool, OUTSET20_A>;
impl OUTSET20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET20_A {
        match self.bits {
            false => OUTSET20_A::NOP,
            true => OUTSET20_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET20_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET20_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET20`"]
pub struct OUTSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET20_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET20_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Port Data Output Value Set 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET21_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET21_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET21`"]
pub type OUTSET21_R = crate::R<bool, OUTSET21_A>;
impl OUTSET21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET21_A {
        match self.bits {
            false => OUTSET21_A::NOP,
            true => OUTSET21_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET21_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET21_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET21`"]
pub struct OUTSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET21_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET21_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Port Data Output Value Set 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET22_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET22_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET22`"]
pub type OUTSET22_R = crate::R<bool, OUTSET22_A>;
impl OUTSET22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET22_A {
        match self.bits {
            false => OUTSET22_A::NOP,
            true => OUTSET22_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET22_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET22_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET22`"]
pub struct OUTSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET22_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET22_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Port Data Output Value Set 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET23_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET23_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET23`"]
pub type OUTSET23_R = crate::R<bool, OUTSET23_A>;
impl OUTSET23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET23_A {
        match self.bits {
            false => OUTSET23_A::NOP,
            true => OUTSET23_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET23_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET23_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET23`"]
pub struct OUTSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET23_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET23_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Port Data Output Value Set 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET24_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET24_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET24`"]
pub type OUTSET24_R = crate::R<bool, OUTSET24_A>;
impl OUTSET24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET24_A {
        match self.bits {
            false => OUTSET24_A::NOP,
            true => OUTSET24_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET24_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET24_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET24`"]
pub struct OUTSET24_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET24_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET24_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Port Data Output Value Set 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET25_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET25_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET25`"]
pub type OUTSET25_R = crate::R<bool, OUTSET25_A>;
impl OUTSET25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET25_A {
        match self.bits {
            false => OUTSET25_A::NOP,
            true => OUTSET25_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET25_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET25_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET25`"]
pub struct OUTSET25_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET25_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET25_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Port Data Output Value Set 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET26_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET26_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET26`"]
pub type OUTSET26_R = crate::R<bool, OUTSET26_A>;
impl OUTSET26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET26_A {
        match self.bits {
            false => OUTSET26_A::NOP,
            true => OUTSET26_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET26_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET26_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET26`"]
pub struct OUTSET26_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET26_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET26_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Port Data Output Value Set 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET27_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET27_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET27`"]
pub type OUTSET27_R = crate::R<bool, OUTSET27_A>;
impl OUTSET27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET27_A {
        match self.bits {
            false => OUTSET27_A::NOP,
            true => OUTSET27_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET27_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET27_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET27`"]
pub struct OUTSET27_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET27_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET27_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Port Data Output Value Set 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET28_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET28_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET28`"]
pub type OUTSET28_R = crate::R<bool, OUTSET28_A>;
impl OUTSET28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET28_A {
        match self.bits {
            false => OUTSET28_A::NOP,
            true => OUTSET28_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET28_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET28_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET28`"]
pub struct OUTSET28_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET28_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET28_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Port Data Output Value Set 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET29_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET29_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET29`"]
pub type OUTSET29_R = crate::R<bool, OUTSET29_A>;
impl OUTSET29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET29_A {
        match self.bits {
            false => OUTSET29_A::NOP,
            true => OUTSET29_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET29_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET29_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET29`"]
pub struct OUTSET29_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET29_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET29_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Port Data Output Value Set 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET30_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET30_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET30`"]
pub type OUTSET30_R = crate::R<bool, OUTSET30_A>;
impl OUTSET30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET30_A {
        match self.bits {
            false => OUTSET30_A::NOP,
            true => OUTSET30_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET30_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET30_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET30`"]
pub struct OUTSET30_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET30_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET30_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Port Data Output Value Set 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSET31_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 1"]
    OUT1 = 1,
}
impl From<OUTSET31_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSET31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTSET31`"]
pub type OUTSET31_R = crate::R<bool, OUTSET31_A>;
impl OUTSET31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSET31_A {
        match self.bits {
            false => OUTSET31_A::NOP,
            true => OUTSET31_A::OUT1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTSET31_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTSET31_A::OUT1
    }
}
#[doc = "Write proxy for field `OUTSET31`"]
pub struct OUTSET31_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSET31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTSET31_A::NOP)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTSET31_A::OUT1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port Data Output Value Set 0"]
    #[inline(always)]
    pub fn outset0(&self) -> OUTSET0_R {
        OUTSET0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Output Value Set 1"]
    #[inline(always)]
    pub fn outset1(&self) -> OUTSET1_R {
        OUTSET1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Output Value Set 2"]
    #[inline(always)]
    pub fn outset2(&self) -> OUTSET2_R {
        OUTSET2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Output Value Set 3"]
    #[inline(always)]
    pub fn outset3(&self) -> OUTSET3_R {
        OUTSET3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Output Value Set 4"]
    #[inline(always)]
    pub fn outset4(&self) -> OUTSET4_R {
        OUTSET4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Output Value Set 5"]
    #[inline(always)]
    pub fn outset5(&self) -> OUTSET5_R {
        OUTSET5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Output Value Set 6"]
    #[inline(always)]
    pub fn outset6(&self) -> OUTSET6_R {
        OUTSET6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Output Value Set 7"]
    #[inline(always)]
    pub fn outset7(&self) -> OUTSET7_R {
        OUTSET7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Output Value Set 8"]
    #[inline(always)]
    pub fn outset8(&self) -> OUTSET8_R {
        OUTSET8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Output Value Set 9"]
    #[inline(always)]
    pub fn outset9(&self) -> OUTSET9_R {
        OUTSET9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Output Value Set 10"]
    #[inline(always)]
    pub fn outset10(&self) -> OUTSET10_R {
        OUTSET10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Output Value Set 11"]
    #[inline(always)]
    pub fn outset11(&self) -> OUTSET11_R {
        OUTSET11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Output Value Set 12"]
    #[inline(always)]
    pub fn outset12(&self) -> OUTSET12_R {
        OUTSET12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Output Value Set 13"]
    #[inline(always)]
    pub fn outset13(&self) -> OUTSET13_R {
        OUTSET13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Output Value Set 14"]
    #[inline(always)]
    pub fn outset14(&self) -> OUTSET14_R {
        OUTSET14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Output Value Set 15"]
    #[inline(always)]
    pub fn outset15(&self) -> OUTSET15_R {
        OUTSET15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Output Value Set 16"]
    #[inline(always)]
    pub fn outset16(&self) -> OUTSET16_R {
        OUTSET16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Output Value Set 17"]
    #[inline(always)]
    pub fn outset17(&self) -> OUTSET17_R {
        OUTSET17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Output Value Set 18"]
    #[inline(always)]
    pub fn outset18(&self) -> OUTSET18_R {
        OUTSET18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Output Value Set 19"]
    #[inline(always)]
    pub fn outset19(&self) -> OUTSET19_R {
        OUTSET19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Output Value Set 20"]
    #[inline(always)]
    pub fn outset20(&self) -> OUTSET20_R {
        OUTSET20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Output Value Set 21"]
    #[inline(always)]
    pub fn outset21(&self) -> OUTSET21_R {
        OUTSET21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Output Value Set 22"]
    #[inline(always)]
    pub fn outset22(&self) -> OUTSET22_R {
        OUTSET22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Output Value Set 23"]
    #[inline(always)]
    pub fn outset23(&self) -> OUTSET23_R {
        OUTSET23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Output Value Set 24"]
    #[inline(always)]
    pub fn outset24(&self) -> OUTSET24_R {
        OUTSET24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Output Value Set 25"]
    #[inline(always)]
    pub fn outset25(&self) -> OUTSET25_R {
        OUTSET25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Output Value Set 26"]
    #[inline(always)]
    pub fn outset26(&self) -> OUTSET26_R {
        OUTSET26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Output Value Set 27"]
    #[inline(always)]
    pub fn outset27(&self) -> OUTSET27_R {
        OUTSET27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Output Value Set 28"]
    #[inline(always)]
    pub fn outset28(&self) -> OUTSET28_R {
        OUTSET28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Output Value Set 29"]
    #[inline(always)]
    pub fn outset29(&self) -> OUTSET29_R {
        OUTSET29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Output Value Set 30"]
    #[inline(always)]
    pub fn outset30(&self) -> OUTSET30_R {
        OUTSET30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Output Value Set 31"]
    #[inline(always)]
    pub fn outset31(&self) -> OUTSET31_R {
        OUTSET31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Output Value Set 0"]
    #[inline(always)]
    pub fn outset0(&mut self) -> OUTSET0_W {
        OUTSET0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Output Value Set 1"]
    #[inline(always)]
    pub fn outset1(&mut self) -> OUTSET1_W {
        OUTSET1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Output Value Set 2"]
    #[inline(always)]
    pub fn outset2(&mut self) -> OUTSET2_W {
        OUTSET2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Output Value Set 3"]
    #[inline(always)]
    pub fn outset3(&mut self) -> OUTSET3_W {
        OUTSET3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Output Value Set 4"]
    #[inline(always)]
    pub fn outset4(&mut self) -> OUTSET4_W {
        OUTSET4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Output Value Set 5"]
    #[inline(always)]
    pub fn outset5(&mut self) -> OUTSET5_W {
        OUTSET5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Output Value Set 6"]
    #[inline(always)]
    pub fn outset6(&mut self) -> OUTSET6_W {
        OUTSET6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Output Value Set 7"]
    #[inline(always)]
    pub fn outset7(&mut self) -> OUTSET7_W {
        OUTSET7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Output Value Set 8"]
    #[inline(always)]
    pub fn outset8(&mut self) -> OUTSET8_W {
        OUTSET8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Output Value Set 9"]
    #[inline(always)]
    pub fn outset9(&mut self) -> OUTSET9_W {
        OUTSET9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Output Value Set 10"]
    #[inline(always)]
    pub fn outset10(&mut self) -> OUTSET10_W {
        OUTSET10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Output Value Set 11"]
    #[inline(always)]
    pub fn outset11(&mut self) -> OUTSET11_W {
        OUTSET11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Output Value Set 12"]
    #[inline(always)]
    pub fn outset12(&mut self) -> OUTSET12_W {
        OUTSET12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Output Value Set 13"]
    #[inline(always)]
    pub fn outset13(&mut self) -> OUTSET13_W {
        OUTSET13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Output Value Set 14"]
    #[inline(always)]
    pub fn outset14(&mut self) -> OUTSET14_W {
        OUTSET14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Output Value Set 15"]
    #[inline(always)]
    pub fn outset15(&mut self) -> OUTSET15_W {
        OUTSET15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Output Value Set 16"]
    #[inline(always)]
    pub fn outset16(&mut self) -> OUTSET16_W {
        OUTSET16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Output Value Set 17"]
    #[inline(always)]
    pub fn outset17(&mut self) -> OUTSET17_W {
        OUTSET17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Output Value Set 18"]
    #[inline(always)]
    pub fn outset18(&mut self) -> OUTSET18_W {
        OUTSET18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Output Value Set 19"]
    #[inline(always)]
    pub fn outset19(&mut self) -> OUTSET19_W {
        OUTSET19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Output Value Set 20"]
    #[inline(always)]
    pub fn outset20(&mut self) -> OUTSET20_W {
        OUTSET20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Output Value Set 21"]
    #[inline(always)]
    pub fn outset21(&mut self) -> OUTSET21_W {
        OUTSET21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Output Value Set 22"]
    #[inline(always)]
    pub fn outset22(&mut self) -> OUTSET22_W {
        OUTSET22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Output Value Set 23"]
    #[inline(always)]
    pub fn outset23(&mut self) -> OUTSET23_W {
        OUTSET23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Output Value Set 24"]
    #[inline(always)]
    pub fn outset24(&mut self) -> OUTSET24_W {
        OUTSET24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Output Value Set 25"]
    #[inline(always)]
    pub fn outset25(&mut self) -> OUTSET25_W {
        OUTSET25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Output Value Set 26"]
    #[inline(always)]
    pub fn outset26(&mut self) -> OUTSET26_W {
        OUTSET26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Output Value Set 27"]
    #[inline(always)]
    pub fn outset27(&mut self) -> OUTSET27_W {
        OUTSET27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Output Value Set 28"]
    #[inline(always)]
    pub fn outset28(&mut self) -> OUTSET28_W {
        OUTSET28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Output Value Set 29"]
    #[inline(always)]
    pub fn outset29(&mut self) -> OUTSET29_W {
        OUTSET29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Output Value Set 30"]
    #[inline(always)]
    pub fn outset30(&mut self) -> OUTSET30_W {
        OUTSET30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Output Value Set 31"]
    #[inline(always)]
    pub fn outset31(&mut self) -> OUTSET31_W {
        OUTSET31_W { w: self }
    }
}
