#[doc = "Reader of register DIRSET%s"]
pub type R = crate::R<u32, super::DIRSET>;
#[doc = "Writer for register DIRSET%s"]
pub type W = crate::W<u32, super::DIRSET>;
#[doc = "Register DIRSET%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DIRSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Direction Set 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET0_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET0_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET0`"]
pub type DIRSET0_R = crate::R<bool, DIRSET0_A>;
impl DIRSET0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET0_A {
        match self.bits {
            false => DIRSET0_A::NOP,
            true => DIRSET0_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET0_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET0_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET0`"]
pub struct DIRSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET0_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET0_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET1_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET1_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET1`"]
pub type DIRSET1_R = crate::R<bool, DIRSET1_A>;
impl DIRSET1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET1_A {
        match self.bits {
            false => DIRSET1_A::NOP,
            true => DIRSET1_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET1_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET1_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET1`"]
pub struct DIRSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET1_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET1_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET2_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET2_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET2`"]
pub type DIRSET2_R = crate::R<bool, DIRSET2_A>;
impl DIRSET2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET2_A {
        match self.bits {
            false => DIRSET2_A::NOP,
            true => DIRSET2_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET2_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET2_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET2`"]
pub struct DIRSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET2_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET2_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET3_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET3_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET3`"]
pub type DIRSET3_R = crate::R<bool, DIRSET3_A>;
impl DIRSET3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET3_A {
        match self.bits {
            false => DIRSET3_A::NOP,
            true => DIRSET3_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET3_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET3_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET3`"]
pub struct DIRSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET3_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET3_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET4_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET4_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET4`"]
pub type DIRSET4_R = crate::R<bool, DIRSET4_A>;
impl DIRSET4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET4_A {
        match self.bits {
            false => DIRSET4_A::NOP,
            true => DIRSET4_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET4_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET4_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET4`"]
pub struct DIRSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET4_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET4_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET5_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET5_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET5`"]
pub type DIRSET5_R = crate::R<bool, DIRSET5_A>;
impl DIRSET5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET5_A {
        match self.bits {
            false => DIRSET5_A::NOP,
            true => DIRSET5_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET5_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET5_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET5`"]
pub struct DIRSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET5_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET5_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET6_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET6_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET6`"]
pub type DIRSET6_R = crate::R<bool, DIRSET6_A>;
impl DIRSET6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET6_A {
        match self.bits {
            false => DIRSET6_A::NOP,
            true => DIRSET6_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET6_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET6_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET6`"]
pub struct DIRSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET6_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET6_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET7_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET7_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET7`"]
pub type DIRSET7_R = crate::R<bool, DIRSET7_A>;
impl DIRSET7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET7_A {
        match self.bits {
            false => DIRSET7_A::NOP,
            true => DIRSET7_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET7_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET7_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET7`"]
pub struct DIRSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET7_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET7_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET8_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET8_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET8`"]
pub type DIRSET8_R = crate::R<bool, DIRSET8_A>;
impl DIRSET8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET8_A {
        match self.bits {
            false => DIRSET8_A::NOP,
            true => DIRSET8_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET8_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET8_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET8`"]
pub struct DIRSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET8_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET8_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET9_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET9_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET9`"]
pub type DIRSET9_R = crate::R<bool, DIRSET9_A>;
impl DIRSET9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET9_A {
        match self.bits {
            false => DIRSET9_A::NOP,
            true => DIRSET9_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET9_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET9_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET9`"]
pub struct DIRSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET9_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET9_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET10_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET10_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET10`"]
pub type DIRSET10_R = crate::R<bool, DIRSET10_A>;
impl DIRSET10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET10_A {
        match self.bits {
            false => DIRSET10_A::NOP,
            true => DIRSET10_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET10_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET10_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET10`"]
pub struct DIRSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET10_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET10_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET11_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET11_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET11`"]
pub type DIRSET11_R = crate::R<bool, DIRSET11_A>;
impl DIRSET11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET11_A {
        match self.bits {
            false => DIRSET11_A::NOP,
            true => DIRSET11_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET11_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET11_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET11`"]
pub struct DIRSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET11_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET11_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET12_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET12_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET12`"]
pub type DIRSET12_R = crate::R<bool, DIRSET12_A>;
impl DIRSET12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET12_A {
        match self.bits {
            false => DIRSET12_A::NOP,
            true => DIRSET12_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET12_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET12_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET12`"]
pub struct DIRSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET12_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET12_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET13_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET13_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET13`"]
pub type DIRSET13_R = crate::R<bool, DIRSET13_A>;
impl DIRSET13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET13_A {
        match self.bits {
            false => DIRSET13_A::NOP,
            true => DIRSET13_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET13_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET13_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET13`"]
pub struct DIRSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET13_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET13_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET14_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET14_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET14`"]
pub type DIRSET14_R = crate::R<bool, DIRSET14_A>;
impl DIRSET14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET14_A {
        match self.bits {
            false => DIRSET14_A::NOP,
            true => DIRSET14_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET14_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET14_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET14`"]
pub struct DIRSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET14_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET14_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET15_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET15_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET15`"]
pub type DIRSET15_R = crate::R<bool, DIRSET15_A>;
impl DIRSET15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET15_A {
        match self.bits {
            false => DIRSET15_A::NOP,
            true => DIRSET15_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET15_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET15_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET15`"]
pub struct DIRSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET15_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET15_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET16_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET16_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET16`"]
pub type DIRSET16_R = crate::R<bool, DIRSET16_A>;
impl DIRSET16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET16_A {
        match self.bits {
            false => DIRSET16_A::NOP,
            true => DIRSET16_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET16_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET16_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET16`"]
pub struct DIRSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET16_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET16_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET17_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET17_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET17`"]
pub type DIRSET17_R = crate::R<bool, DIRSET17_A>;
impl DIRSET17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET17_A {
        match self.bits {
            false => DIRSET17_A::NOP,
            true => DIRSET17_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET17_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET17_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET17`"]
pub struct DIRSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET17_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET17_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET18_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET18_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET18`"]
pub type DIRSET18_R = crate::R<bool, DIRSET18_A>;
impl DIRSET18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET18_A {
        match self.bits {
            false => DIRSET18_A::NOP,
            true => DIRSET18_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET18_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET18_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET18`"]
pub struct DIRSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET18_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET18_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET19_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET19_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET19`"]
pub type DIRSET19_R = crate::R<bool, DIRSET19_A>;
impl DIRSET19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET19_A {
        match self.bits {
            false => DIRSET19_A::NOP,
            true => DIRSET19_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET19_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET19_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET19`"]
pub struct DIRSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET19_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET19_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET20_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET20_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET20`"]
pub type DIRSET20_R = crate::R<bool, DIRSET20_A>;
impl DIRSET20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET20_A {
        match self.bits {
            false => DIRSET20_A::NOP,
            true => DIRSET20_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET20_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET20_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET20`"]
pub struct DIRSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET20_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET20_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET21_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET21_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET21`"]
pub type DIRSET21_R = crate::R<bool, DIRSET21_A>;
impl DIRSET21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET21_A {
        match self.bits {
            false => DIRSET21_A::NOP,
            true => DIRSET21_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET21_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET21_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET21`"]
pub struct DIRSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET21_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET21_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET22_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET22_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET22`"]
pub type DIRSET22_R = crate::R<bool, DIRSET22_A>;
impl DIRSET22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET22_A {
        match self.bits {
            false => DIRSET22_A::NOP,
            true => DIRSET22_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET22_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET22_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET22`"]
pub struct DIRSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET22_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET22_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET23_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET23_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET23`"]
pub type DIRSET23_R = crate::R<bool, DIRSET23_A>;
impl DIRSET23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET23_A {
        match self.bits {
            false => DIRSET23_A::NOP,
            true => DIRSET23_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET23_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET23_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET23`"]
pub struct DIRSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET23_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET23_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET24_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET24_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET24`"]
pub type DIRSET24_R = crate::R<bool, DIRSET24_A>;
impl DIRSET24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET24_A {
        match self.bits {
            false => DIRSET24_A::NOP,
            true => DIRSET24_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET24_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET24_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET24`"]
pub struct DIRSET24_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET24_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET24_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET25_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET25_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET25`"]
pub type DIRSET25_R = crate::R<bool, DIRSET25_A>;
impl DIRSET25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET25_A {
        match self.bits {
            false => DIRSET25_A::NOP,
            true => DIRSET25_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET25_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET25_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET25`"]
pub struct DIRSET25_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET25_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET25_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET26_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET26_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET26`"]
pub type DIRSET26_R = crate::R<bool, DIRSET26_A>;
impl DIRSET26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET26_A {
        match self.bits {
            false => DIRSET26_A::NOP,
            true => DIRSET26_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET26_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET26_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET26`"]
pub struct DIRSET26_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET26_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET26_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET27_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET27_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET27`"]
pub type DIRSET27_R = crate::R<bool, DIRSET27_A>;
impl DIRSET27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET27_A {
        match self.bits {
            false => DIRSET27_A::NOP,
            true => DIRSET27_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET27_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET27_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET27`"]
pub struct DIRSET27_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET27_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET27_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET28_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET28_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET28`"]
pub type DIRSET28_R = crate::R<bool, DIRSET28_A>;
impl DIRSET28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET28_A {
        match self.bits {
            false => DIRSET28_A::NOP,
            true => DIRSET28_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET28_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET28_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET28`"]
pub struct DIRSET28_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET28_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET28_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET29_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET29_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET29`"]
pub type DIRSET29_R = crate::R<bool, DIRSET29_A>;
impl DIRSET29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET29_A {
        match self.bits {
            false => DIRSET29_A::NOP,
            true => DIRSET29_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET29_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET29_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET29`"]
pub struct DIRSET29_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET29_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET29_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET30_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET30_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET30`"]
pub type DIRSET30_R = crate::R<bool, DIRSET30_A>;
impl DIRSET30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET30_A {
        match self.bits {
            false => DIRSET30_A::NOP,
            true => DIRSET30_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET30_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET30_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET30`"]
pub struct DIRSET30_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET30_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET30_A::SETOUTPUT)
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
#[doc = "Port Data Direction Set 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSET31_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin output"]
    SETOUTPUT = 1,
}
impl From<DIRSET31_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSET31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRSET31`"]
pub type DIRSET31_R = crate::R<bool, DIRSET31_A>;
impl DIRSET31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSET31_A {
        match self.bits {
            false => DIRSET31_A::NOP,
            true => DIRSET31_A::SETOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRSET31_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETOUTPUT`"]
    #[inline(always)]
    pub fn is_setoutput(&self) -> bool {
        *self == DIRSET31_A::SETOUTPUT
    }
}
#[doc = "Write proxy for field `DIRSET31`"]
pub struct DIRSET31_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSET31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSET31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRSET31_A::NOP)
    }
    #[doc = "Make pin output"]
    #[inline(always)]
    pub fn setoutput(self) -> &'a mut W {
        self.variant(DIRSET31_A::SETOUTPUT)
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
    #[doc = "Bit 0 - Port Data Direction Set 0"]
    #[inline(always)]
    pub fn dirset0(&self) -> DIRSET0_R {
        DIRSET0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Direction Set 1"]
    #[inline(always)]
    pub fn dirset1(&self) -> DIRSET1_R {
        DIRSET1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Direction Set 2"]
    #[inline(always)]
    pub fn dirset2(&self) -> DIRSET2_R {
        DIRSET2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Direction Set 3"]
    #[inline(always)]
    pub fn dirset3(&self) -> DIRSET3_R {
        DIRSET3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Direction Set 4"]
    #[inline(always)]
    pub fn dirset4(&self) -> DIRSET4_R {
        DIRSET4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Direction Set 5"]
    #[inline(always)]
    pub fn dirset5(&self) -> DIRSET5_R {
        DIRSET5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Direction Set 6"]
    #[inline(always)]
    pub fn dirset6(&self) -> DIRSET6_R {
        DIRSET6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Direction Set 7"]
    #[inline(always)]
    pub fn dirset7(&self) -> DIRSET7_R {
        DIRSET7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Direction Set 8"]
    #[inline(always)]
    pub fn dirset8(&self) -> DIRSET8_R {
        DIRSET8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Direction Set 9"]
    #[inline(always)]
    pub fn dirset9(&self) -> DIRSET9_R {
        DIRSET9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Direction Set 10"]
    #[inline(always)]
    pub fn dirset10(&self) -> DIRSET10_R {
        DIRSET10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Direction Set 11"]
    #[inline(always)]
    pub fn dirset11(&self) -> DIRSET11_R {
        DIRSET11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Direction Set 12"]
    #[inline(always)]
    pub fn dirset12(&self) -> DIRSET12_R {
        DIRSET12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Direction Set 13"]
    #[inline(always)]
    pub fn dirset13(&self) -> DIRSET13_R {
        DIRSET13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Direction Set 14"]
    #[inline(always)]
    pub fn dirset14(&self) -> DIRSET14_R {
        DIRSET14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Direction Set 15"]
    #[inline(always)]
    pub fn dirset15(&self) -> DIRSET15_R {
        DIRSET15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Direction Set 16"]
    #[inline(always)]
    pub fn dirset16(&self) -> DIRSET16_R {
        DIRSET16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Direction Set 17"]
    #[inline(always)]
    pub fn dirset17(&self) -> DIRSET17_R {
        DIRSET17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Direction Set 18"]
    #[inline(always)]
    pub fn dirset18(&self) -> DIRSET18_R {
        DIRSET18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Direction Set 19"]
    #[inline(always)]
    pub fn dirset19(&self) -> DIRSET19_R {
        DIRSET19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Direction Set 20"]
    #[inline(always)]
    pub fn dirset20(&self) -> DIRSET20_R {
        DIRSET20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Direction Set 21"]
    #[inline(always)]
    pub fn dirset21(&self) -> DIRSET21_R {
        DIRSET21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Direction Set 22"]
    #[inline(always)]
    pub fn dirset22(&self) -> DIRSET22_R {
        DIRSET22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Direction Set 23"]
    #[inline(always)]
    pub fn dirset23(&self) -> DIRSET23_R {
        DIRSET23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Direction Set 24"]
    #[inline(always)]
    pub fn dirset24(&self) -> DIRSET24_R {
        DIRSET24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Direction Set 25"]
    #[inline(always)]
    pub fn dirset25(&self) -> DIRSET25_R {
        DIRSET25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Direction Set 26"]
    #[inline(always)]
    pub fn dirset26(&self) -> DIRSET26_R {
        DIRSET26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Direction Set 27"]
    #[inline(always)]
    pub fn dirset27(&self) -> DIRSET27_R {
        DIRSET27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Direction Set 28"]
    #[inline(always)]
    pub fn dirset28(&self) -> DIRSET28_R {
        DIRSET28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Direction Set 29"]
    #[inline(always)]
    pub fn dirset29(&self) -> DIRSET29_R {
        DIRSET29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Direction Set 30"]
    #[inline(always)]
    pub fn dirset30(&self) -> DIRSET30_R {
        DIRSET30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Direction Set 31"]
    #[inline(always)]
    pub fn dirset31(&self) -> DIRSET31_R {
        DIRSET31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Direction Set 0"]
    #[inline(always)]
    pub fn dirset0(&mut self) -> DIRSET0_W {
        DIRSET0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Direction Set 1"]
    #[inline(always)]
    pub fn dirset1(&mut self) -> DIRSET1_W {
        DIRSET1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Direction Set 2"]
    #[inline(always)]
    pub fn dirset2(&mut self) -> DIRSET2_W {
        DIRSET2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Direction Set 3"]
    #[inline(always)]
    pub fn dirset3(&mut self) -> DIRSET3_W {
        DIRSET3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Direction Set 4"]
    #[inline(always)]
    pub fn dirset4(&mut self) -> DIRSET4_W {
        DIRSET4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Direction Set 5"]
    #[inline(always)]
    pub fn dirset5(&mut self) -> DIRSET5_W {
        DIRSET5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Direction Set 6"]
    #[inline(always)]
    pub fn dirset6(&mut self) -> DIRSET6_W {
        DIRSET6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Direction Set 7"]
    #[inline(always)]
    pub fn dirset7(&mut self) -> DIRSET7_W {
        DIRSET7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Direction Set 8"]
    #[inline(always)]
    pub fn dirset8(&mut self) -> DIRSET8_W {
        DIRSET8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Direction Set 9"]
    #[inline(always)]
    pub fn dirset9(&mut self) -> DIRSET9_W {
        DIRSET9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Direction Set 10"]
    #[inline(always)]
    pub fn dirset10(&mut self) -> DIRSET10_W {
        DIRSET10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Direction Set 11"]
    #[inline(always)]
    pub fn dirset11(&mut self) -> DIRSET11_W {
        DIRSET11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Direction Set 12"]
    #[inline(always)]
    pub fn dirset12(&mut self) -> DIRSET12_W {
        DIRSET12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Direction Set 13"]
    #[inline(always)]
    pub fn dirset13(&mut self) -> DIRSET13_W {
        DIRSET13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Direction Set 14"]
    #[inline(always)]
    pub fn dirset14(&mut self) -> DIRSET14_W {
        DIRSET14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Direction Set 15"]
    #[inline(always)]
    pub fn dirset15(&mut self) -> DIRSET15_W {
        DIRSET15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Direction Set 16"]
    #[inline(always)]
    pub fn dirset16(&mut self) -> DIRSET16_W {
        DIRSET16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Direction Set 17"]
    #[inline(always)]
    pub fn dirset17(&mut self) -> DIRSET17_W {
        DIRSET17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Direction Set 18"]
    #[inline(always)]
    pub fn dirset18(&mut self) -> DIRSET18_W {
        DIRSET18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Direction Set 19"]
    #[inline(always)]
    pub fn dirset19(&mut self) -> DIRSET19_W {
        DIRSET19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Direction Set 20"]
    #[inline(always)]
    pub fn dirset20(&mut self) -> DIRSET20_W {
        DIRSET20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Direction Set 21"]
    #[inline(always)]
    pub fn dirset21(&mut self) -> DIRSET21_W {
        DIRSET21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Direction Set 22"]
    #[inline(always)]
    pub fn dirset22(&mut self) -> DIRSET22_W {
        DIRSET22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Direction Set 23"]
    #[inline(always)]
    pub fn dirset23(&mut self) -> DIRSET23_W {
        DIRSET23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Direction Set 24"]
    #[inline(always)]
    pub fn dirset24(&mut self) -> DIRSET24_W {
        DIRSET24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Direction Set 25"]
    #[inline(always)]
    pub fn dirset25(&mut self) -> DIRSET25_W {
        DIRSET25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Direction Set 26"]
    #[inline(always)]
    pub fn dirset26(&mut self) -> DIRSET26_W {
        DIRSET26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Direction Set 27"]
    #[inline(always)]
    pub fn dirset27(&mut self) -> DIRSET27_W {
        DIRSET27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Direction Set 28"]
    #[inline(always)]
    pub fn dirset28(&mut self) -> DIRSET28_W {
        DIRSET28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Direction Set 29"]
    #[inline(always)]
    pub fn dirset29(&mut self) -> DIRSET29_W {
        DIRSET29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Direction Set 30"]
    #[inline(always)]
    pub fn dirset30(&mut self) -> DIRSET30_W {
        DIRSET30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Direction Set 31"]
    #[inline(always)]
    pub fn dirset31(&mut self) -> DIRSET31_W {
        DIRSET31_W { w: self }
    }
}
