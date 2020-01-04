#[doc = "Reader of register DIRCLR%s"]
pub type R = crate::R<u32, super::DIRCLR>;
#[doc = "Writer for register DIRCLR%s"]
pub type W = crate::W<u32, super::DIRCLR>;
#[doc = "Register DIRCLR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DIRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Direction Clear 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR0_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR0_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR0`"]
pub type DIRCLR0_R = crate::R<bool, DIRCLR0_A>;
impl DIRCLR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR0_A {
        match self.bits {
            false => DIRCLR0_A::NOP,
            true => DIRCLR0_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR0_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR0_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR0`"]
pub struct DIRCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR0_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR0_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR1_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR1_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR1`"]
pub type DIRCLR1_R = crate::R<bool, DIRCLR1_A>;
impl DIRCLR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR1_A {
        match self.bits {
            false => DIRCLR1_A::NOP,
            true => DIRCLR1_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR1_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR1_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR1`"]
pub struct DIRCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR1_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR1_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR2_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR2_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR2`"]
pub type DIRCLR2_R = crate::R<bool, DIRCLR2_A>;
impl DIRCLR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR2_A {
        match self.bits {
            false => DIRCLR2_A::NOP,
            true => DIRCLR2_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR2_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR2_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR2`"]
pub struct DIRCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR2_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR2_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR3_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR3_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR3`"]
pub type DIRCLR3_R = crate::R<bool, DIRCLR3_A>;
impl DIRCLR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR3_A {
        match self.bits {
            false => DIRCLR3_A::NOP,
            true => DIRCLR3_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR3_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR3_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR3`"]
pub struct DIRCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR3_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR3_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR4_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR4_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR4`"]
pub type DIRCLR4_R = crate::R<bool, DIRCLR4_A>;
impl DIRCLR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR4_A {
        match self.bits {
            false => DIRCLR4_A::NOP,
            true => DIRCLR4_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR4_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR4_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR4`"]
pub struct DIRCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR4_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR4_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR5_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR5_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR5`"]
pub type DIRCLR5_R = crate::R<bool, DIRCLR5_A>;
impl DIRCLR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR5_A {
        match self.bits {
            false => DIRCLR5_A::NOP,
            true => DIRCLR5_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR5_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR5_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR5`"]
pub struct DIRCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR5_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR5_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR6_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR6_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR6`"]
pub type DIRCLR6_R = crate::R<bool, DIRCLR6_A>;
impl DIRCLR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR6_A {
        match self.bits {
            false => DIRCLR6_A::NOP,
            true => DIRCLR6_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR6_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR6_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR6`"]
pub struct DIRCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR6_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR6_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR7_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR7_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR7`"]
pub type DIRCLR7_R = crate::R<bool, DIRCLR7_A>;
impl DIRCLR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR7_A {
        match self.bits {
            false => DIRCLR7_A::NOP,
            true => DIRCLR7_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR7_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR7_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR7`"]
pub struct DIRCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR7_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR7_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR8_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR8_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR8`"]
pub type DIRCLR8_R = crate::R<bool, DIRCLR8_A>;
impl DIRCLR8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR8_A {
        match self.bits {
            false => DIRCLR8_A::NOP,
            true => DIRCLR8_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR8_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR8_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR8`"]
pub struct DIRCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR8_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR8_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR9_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR9_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR9`"]
pub type DIRCLR9_R = crate::R<bool, DIRCLR9_A>;
impl DIRCLR9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR9_A {
        match self.bits {
            false => DIRCLR9_A::NOP,
            true => DIRCLR9_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR9_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR9_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR9`"]
pub struct DIRCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR9_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR9_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR10_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR10_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR10`"]
pub type DIRCLR10_R = crate::R<bool, DIRCLR10_A>;
impl DIRCLR10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR10_A {
        match self.bits {
            false => DIRCLR10_A::NOP,
            true => DIRCLR10_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR10_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR10_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR10`"]
pub struct DIRCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR10_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR10_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR11_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR11_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR11`"]
pub type DIRCLR11_R = crate::R<bool, DIRCLR11_A>;
impl DIRCLR11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR11_A {
        match self.bits {
            false => DIRCLR11_A::NOP,
            true => DIRCLR11_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR11_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR11_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR11`"]
pub struct DIRCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR11_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR11_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR12_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR12_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR12`"]
pub type DIRCLR12_R = crate::R<bool, DIRCLR12_A>;
impl DIRCLR12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR12_A {
        match self.bits {
            false => DIRCLR12_A::NOP,
            true => DIRCLR12_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR12_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR12_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR12`"]
pub struct DIRCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR12_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR12_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR13_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR13_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR13`"]
pub type DIRCLR13_R = crate::R<bool, DIRCLR13_A>;
impl DIRCLR13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR13_A {
        match self.bits {
            false => DIRCLR13_A::NOP,
            true => DIRCLR13_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR13_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR13_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR13`"]
pub struct DIRCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR13_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR13_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR14_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR14_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR14`"]
pub type DIRCLR14_R = crate::R<bool, DIRCLR14_A>;
impl DIRCLR14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR14_A {
        match self.bits {
            false => DIRCLR14_A::NOP,
            true => DIRCLR14_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR14_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR14_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR14`"]
pub struct DIRCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR14_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR14_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR15_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR15_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR15`"]
pub type DIRCLR15_R = crate::R<bool, DIRCLR15_A>;
impl DIRCLR15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR15_A {
        match self.bits {
            false => DIRCLR15_A::NOP,
            true => DIRCLR15_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR15_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR15_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR15`"]
pub struct DIRCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR15_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR15_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR16_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR16_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR16`"]
pub type DIRCLR16_R = crate::R<bool, DIRCLR16_A>;
impl DIRCLR16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR16_A {
        match self.bits {
            false => DIRCLR16_A::NOP,
            true => DIRCLR16_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR16_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR16_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR16`"]
pub struct DIRCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR16_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR16_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR17_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR17_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR17`"]
pub type DIRCLR17_R = crate::R<bool, DIRCLR17_A>;
impl DIRCLR17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR17_A {
        match self.bits {
            false => DIRCLR17_A::NOP,
            true => DIRCLR17_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR17_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR17_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR17`"]
pub struct DIRCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR17_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR17_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR18_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR18_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR18`"]
pub type DIRCLR18_R = crate::R<bool, DIRCLR18_A>;
impl DIRCLR18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR18_A {
        match self.bits {
            false => DIRCLR18_A::NOP,
            true => DIRCLR18_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR18_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR18_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR18`"]
pub struct DIRCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR18_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR18_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR19_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR19_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR19`"]
pub type DIRCLR19_R = crate::R<bool, DIRCLR19_A>;
impl DIRCLR19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR19_A {
        match self.bits {
            false => DIRCLR19_A::NOP,
            true => DIRCLR19_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR19_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR19_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR19`"]
pub struct DIRCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR19_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR19_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR20_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR20_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR20`"]
pub type DIRCLR20_R = crate::R<bool, DIRCLR20_A>;
impl DIRCLR20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR20_A {
        match self.bits {
            false => DIRCLR20_A::NOP,
            true => DIRCLR20_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR20_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR20_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR20`"]
pub struct DIRCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR20_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR20_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR21_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR21_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR21`"]
pub type DIRCLR21_R = crate::R<bool, DIRCLR21_A>;
impl DIRCLR21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR21_A {
        match self.bits {
            false => DIRCLR21_A::NOP,
            true => DIRCLR21_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR21_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR21_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR21`"]
pub struct DIRCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR21_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR21_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR22_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR22_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR22`"]
pub type DIRCLR22_R = crate::R<bool, DIRCLR22_A>;
impl DIRCLR22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR22_A {
        match self.bits {
            false => DIRCLR22_A::NOP,
            true => DIRCLR22_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR22_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR22_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR22`"]
pub struct DIRCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR22_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR22_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR23_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR23_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR23`"]
pub type DIRCLR23_R = crate::R<bool, DIRCLR23_A>;
impl DIRCLR23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR23_A {
        match self.bits {
            false => DIRCLR23_A::NOP,
            true => DIRCLR23_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR23_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR23_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR23`"]
pub struct DIRCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR23_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR23_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR24_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR24_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR24`"]
pub type DIRCLR24_R = crate::R<bool, DIRCLR24_A>;
impl DIRCLR24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR24_A {
        match self.bits {
            false => DIRCLR24_A::NOP,
            true => DIRCLR24_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR24_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR24_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR24`"]
pub struct DIRCLR24_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR24_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR24_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR25_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR25_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR25`"]
pub type DIRCLR25_R = crate::R<bool, DIRCLR25_A>;
impl DIRCLR25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR25_A {
        match self.bits {
            false => DIRCLR25_A::NOP,
            true => DIRCLR25_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR25_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR25_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR25`"]
pub struct DIRCLR25_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR25_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR25_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR26_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR26_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR26`"]
pub type DIRCLR26_R = crate::R<bool, DIRCLR26_A>;
impl DIRCLR26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR26_A {
        match self.bits {
            false => DIRCLR26_A::NOP,
            true => DIRCLR26_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR26_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR26_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR26`"]
pub struct DIRCLR26_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR26_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR26_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR27_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR27_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR27`"]
pub type DIRCLR27_R = crate::R<bool, DIRCLR27_A>;
impl DIRCLR27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR27_A {
        match self.bits {
            false => DIRCLR27_A::NOP,
            true => DIRCLR27_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR27_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR27_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR27`"]
pub struct DIRCLR27_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR27_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR27_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR28_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR28_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR28`"]
pub type DIRCLR28_R = crate::R<bool, DIRCLR28_A>;
impl DIRCLR28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR28_A {
        match self.bits {
            false => DIRCLR28_A::NOP,
            true => DIRCLR28_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR28_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR28_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR28`"]
pub struct DIRCLR28_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR28_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR28_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR29_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR29_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR29`"]
pub type DIRCLR29_R = crate::R<bool, DIRCLR29_A>;
impl DIRCLR29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR29_A {
        match self.bits {
            false => DIRCLR29_A::NOP,
            true => DIRCLR29_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR29_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR29_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR29`"]
pub struct DIRCLR29_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR29_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR29_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR30_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR30_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR30`"]
pub type DIRCLR30_R = crate::R<bool, DIRCLR30_A>;
impl DIRCLR30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR30_A {
        match self.bits {
            false => DIRCLR30_A::NOP,
            true => DIRCLR30_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR30_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR30_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR30`"]
pub struct DIRCLR30_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR30_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR30_A::SETINPUT)
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
#[doc = "Port Data Direction Clear 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRCLR31_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Make pin input"]
    SETINPUT = 1,
}
impl From<DIRCLR31_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLR31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRCLR31`"]
pub type DIRCLR31_R = crate::R<bool, DIRCLR31_A>;
impl DIRCLR31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLR31_A {
        match self.bits {
            false => DIRCLR31_A::NOP,
            true => DIRCLR31_A::SETINPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == DIRCLR31_A::NOP
    }
    #[doc = "Checks if the value of the field is `SETINPUT`"]
    #[inline(always)]
    pub fn is_setinput(&self) -> bool {
        *self == DIRCLR31_A::SETINPUT
    }
}
#[doc = "Write proxy for field `DIRCLR31`"]
pub struct DIRCLR31_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLR31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRCLR31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRCLR31_A::NOP)
    }
    #[doc = "Make pin input"]
    #[inline(always)]
    pub fn setinput(self) -> &'a mut W {
        self.variant(DIRCLR31_A::SETINPUT)
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
    #[doc = "Bit 0 - Port Data Direction Clear 0"]
    #[inline(always)]
    pub fn dirclr0(&self) -> DIRCLR0_R {
        DIRCLR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Direction Clear 1"]
    #[inline(always)]
    pub fn dirclr1(&self) -> DIRCLR1_R {
        DIRCLR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Direction Clear 2"]
    #[inline(always)]
    pub fn dirclr2(&self) -> DIRCLR2_R {
        DIRCLR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Direction Clear 3"]
    #[inline(always)]
    pub fn dirclr3(&self) -> DIRCLR3_R {
        DIRCLR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Direction Clear 4"]
    #[inline(always)]
    pub fn dirclr4(&self) -> DIRCLR4_R {
        DIRCLR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Direction Clear 5"]
    #[inline(always)]
    pub fn dirclr5(&self) -> DIRCLR5_R {
        DIRCLR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Direction Clear 6"]
    #[inline(always)]
    pub fn dirclr6(&self) -> DIRCLR6_R {
        DIRCLR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Direction Clear 7"]
    #[inline(always)]
    pub fn dirclr7(&self) -> DIRCLR7_R {
        DIRCLR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Direction Clear 8"]
    #[inline(always)]
    pub fn dirclr8(&self) -> DIRCLR8_R {
        DIRCLR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Direction Clear 9"]
    #[inline(always)]
    pub fn dirclr9(&self) -> DIRCLR9_R {
        DIRCLR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Direction Clear 10"]
    #[inline(always)]
    pub fn dirclr10(&self) -> DIRCLR10_R {
        DIRCLR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Direction Clear 11"]
    #[inline(always)]
    pub fn dirclr11(&self) -> DIRCLR11_R {
        DIRCLR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Direction Clear 12"]
    #[inline(always)]
    pub fn dirclr12(&self) -> DIRCLR12_R {
        DIRCLR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Direction Clear 13"]
    #[inline(always)]
    pub fn dirclr13(&self) -> DIRCLR13_R {
        DIRCLR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Direction Clear 14"]
    #[inline(always)]
    pub fn dirclr14(&self) -> DIRCLR14_R {
        DIRCLR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Direction Clear 15"]
    #[inline(always)]
    pub fn dirclr15(&self) -> DIRCLR15_R {
        DIRCLR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Direction Clear 16"]
    #[inline(always)]
    pub fn dirclr16(&self) -> DIRCLR16_R {
        DIRCLR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Direction Clear 17"]
    #[inline(always)]
    pub fn dirclr17(&self) -> DIRCLR17_R {
        DIRCLR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Direction Clear 18"]
    #[inline(always)]
    pub fn dirclr18(&self) -> DIRCLR18_R {
        DIRCLR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Direction Clear 19"]
    #[inline(always)]
    pub fn dirclr19(&self) -> DIRCLR19_R {
        DIRCLR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Direction Clear 20"]
    #[inline(always)]
    pub fn dirclr20(&self) -> DIRCLR20_R {
        DIRCLR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Direction Clear 21"]
    #[inline(always)]
    pub fn dirclr21(&self) -> DIRCLR21_R {
        DIRCLR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Direction Clear 22"]
    #[inline(always)]
    pub fn dirclr22(&self) -> DIRCLR22_R {
        DIRCLR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Direction Clear 23"]
    #[inline(always)]
    pub fn dirclr23(&self) -> DIRCLR23_R {
        DIRCLR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Direction Clear 24"]
    #[inline(always)]
    pub fn dirclr24(&self) -> DIRCLR24_R {
        DIRCLR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Direction Clear 25"]
    #[inline(always)]
    pub fn dirclr25(&self) -> DIRCLR25_R {
        DIRCLR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Direction Clear 26"]
    #[inline(always)]
    pub fn dirclr26(&self) -> DIRCLR26_R {
        DIRCLR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Direction Clear 27"]
    #[inline(always)]
    pub fn dirclr27(&self) -> DIRCLR27_R {
        DIRCLR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Direction Clear 28"]
    #[inline(always)]
    pub fn dirclr28(&self) -> DIRCLR28_R {
        DIRCLR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Direction Clear 29"]
    #[inline(always)]
    pub fn dirclr29(&self) -> DIRCLR29_R {
        DIRCLR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Direction Clear 30"]
    #[inline(always)]
    pub fn dirclr30(&self) -> DIRCLR30_R {
        DIRCLR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Direction Clear 31"]
    #[inline(always)]
    pub fn dirclr31(&self) -> DIRCLR31_R {
        DIRCLR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Direction Clear 0"]
    #[inline(always)]
    pub fn dirclr0(&mut self) -> DIRCLR0_W {
        DIRCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Direction Clear 1"]
    #[inline(always)]
    pub fn dirclr1(&mut self) -> DIRCLR1_W {
        DIRCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Direction Clear 2"]
    #[inline(always)]
    pub fn dirclr2(&mut self) -> DIRCLR2_W {
        DIRCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Direction Clear 3"]
    #[inline(always)]
    pub fn dirclr3(&mut self) -> DIRCLR3_W {
        DIRCLR3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Direction Clear 4"]
    #[inline(always)]
    pub fn dirclr4(&mut self) -> DIRCLR4_W {
        DIRCLR4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Direction Clear 5"]
    #[inline(always)]
    pub fn dirclr5(&mut self) -> DIRCLR5_W {
        DIRCLR5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Direction Clear 6"]
    #[inline(always)]
    pub fn dirclr6(&mut self) -> DIRCLR6_W {
        DIRCLR6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Direction Clear 7"]
    #[inline(always)]
    pub fn dirclr7(&mut self) -> DIRCLR7_W {
        DIRCLR7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Direction Clear 8"]
    #[inline(always)]
    pub fn dirclr8(&mut self) -> DIRCLR8_W {
        DIRCLR8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Direction Clear 9"]
    #[inline(always)]
    pub fn dirclr9(&mut self) -> DIRCLR9_W {
        DIRCLR9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Direction Clear 10"]
    #[inline(always)]
    pub fn dirclr10(&mut self) -> DIRCLR10_W {
        DIRCLR10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Direction Clear 11"]
    #[inline(always)]
    pub fn dirclr11(&mut self) -> DIRCLR11_W {
        DIRCLR11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Direction Clear 12"]
    #[inline(always)]
    pub fn dirclr12(&mut self) -> DIRCLR12_W {
        DIRCLR12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Direction Clear 13"]
    #[inline(always)]
    pub fn dirclr13(&mut self) -> DIRCLR13_W {
        DIRCLR13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Direction Clear 14"]
    #[inline(always)]
    pub fn dirclr14(&mut self) -> DIRCLR14_W {
        DIRCLR14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Direction Clear 15"]
    #[inline(always)]
    pub fn dirclr15(&mut self) -> DIRCLR15_W {
        DIRCLR15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Direction Clear 16"]
    #[inline(always)]
    pub fn dirclr16(&mut self) -> DIRCLR16_W {
        DIRCLR16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Direction Clear 17"]
    #[inline(always)]
    pub fn dirclr17(&mut self) -> DIRCLR17_W {
        DIRCLR17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Direction Clear 18"]
    #[inline(always)]
    pub fn dirclr18(&mut self) -> DIRCLR18_W {
        DIRCLR18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Direction Clear 19"]
    #[inline(always)]
    pub fn dirclr19(&mut self) -> DIRCLR19_W {
        DIRCLR19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Direction Clear 20"]
    #[inline(always)]
    pub fn dirclr20(&mut self) -> DIRCLR20_W {
        DIRCLR20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Direction Clear 21"]
    #[inline(always)]
    pub fn dirclr21(&mut self) -> DIRCLR21_W {
        DIRCLR21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Direction Clear 22"]
    #[inline(always)]
    pub fn dirclr22(&mut self) -> DIRCLR22_W {
        DIRCLR22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Direction Clear 23"]
    #[inline(always)]
    pub fn dirclr23(&mut self) -> DIRCLR23_W {
        DIRCLR23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Direction Clear 24"]
    #[inline(always)]
    pub fn dirclr24(&mut self) -> DIRCLR24_W {
        DIRCLR24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Direction Clear 25"]
    #[inline(always)]
    pub fn dirclr25(&mut self) -> DIRCLR25_W {
        DIRCLR25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Direction Clear 26"]
    #[inline(always)]
    pub fn dirclr26(&mut self) -> DIRCLR26_W {
        DIRCLR26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Direction Clear 27"]
    #[inline(always)]
    pub fn dirclr27(&mut self) -> DIRCLR27_W {
        DIRCLR27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Direction Clear 28"]
    #[inline(always)]
    pub fn dirclr28(&mut self) -> DIRCLR28_W {
        DIRCLR28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Direction Clear 29"]
    #[inline(always)]
    pub fn dirclr29(&mut self) -> DIRCLR29_W {
        DIRCLR29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Direction Clear 30"]
    #[inline(always)]
    pub fn dirclr30(&mut self) -> DIRCLR30_W {
        DIRCLR30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Direction Clear 31"]
    #[inline(always)]
    pub fn dirclr31(&mut self) -> DIRCLR31_W {
        DIRCLR31_W { w: self }
    }
}
