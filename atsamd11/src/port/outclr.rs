#[doc = "Reader of register OUTCLR%s"]
pub type R = crate::R<u32, super::OUTCLR>;
#[doc = "Writer for register OUTCLR%s"]
pub type W = crate::W<u32, super::OUTCLR>;
#[doc = "Register OUTCLR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Output Value Clear 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR0_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR0_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR0`"]
pub type OUTCLR0_R = crate::R<bool, OUTCLR0_A>;
impl OUTCLR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR0_A {
        match self.bits {
            false => OUTCLR0_A::NOP,
            true => OUTCLR0_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR0_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR0_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR0`"]
pub struct OUTCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR0_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR0_A::OUT0)
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
#[doc = "Port Data Output Value Clear 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR1_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR1_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR1`"]
pub type OUTCLR1_R = crate::R<bool, OUTCLR1_A>;
impl OUTCLR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR1_A {
        match self.bits {
            false => OUTCLR1_A::NOP,
            true => OUTCLR1_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR1_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR1_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR1`"]
pub struct OUTCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR1_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR1_A::OUT0)
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
#[doc = "Port Data Output Value Clear 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR2_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR2_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR2`"]
pub type OUTCLR2_R = crate::R<bool, OUTCLR2_A>;
impl OUTCLR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR2_A {
        match self.bits {
            false => OUTCLR2_A::NOP,
            true => OUTCLR2_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR2_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR2_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR2`"]
pub struct OUTCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR2_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR2_A::OUT0)
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
#[doc = "Port Data Output Value Clear 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR3_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR3_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR3`"]
pub type OUTCLR3_R = crate::R<bool, OUTCLR3_A>;
impl OUTCLR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR3_A {
        match self.bits {
            false => OUTCLR3_A::NOP,
            true => OUTCLR3_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR3_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR3_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR3`"]
pub struct OUTCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR3_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR3_A::OUT0)
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
#[doc = "Port Data Output Value Clear 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR4_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR4_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR4`"]
pub type OUTCLR4_R = crate::R<bool, OUTCLR4_A>;
impl OUTCLR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR4_A {
        match self.bits {
            false => OUTCLR4_A::NOP,
            true => OUTCLR4_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR4_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR4_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR4`"]
pub struct OUTCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR4_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR4_A::OUT0)
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
#[doc = "Port Data Output Value Clear 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR5_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR5_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR5`"]
pub type OUTCLR5_R = crate::R<bool, OUTCLR5_A>;
impl OUTCLR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR5_A {
        match self.bits {
            false => OUTCLR5_A::NOP,
            true => OUTCLR5_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR5_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR5_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR5`"]
pub struct OUTCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR5_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR5_A::OUT0)
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
#[doc = "Port Data Output Value Clear 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR6_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR6_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR6`"]
pub type OUTCLR6_R = crate::R<bool, OUTCLR6_A>;
impl OUTCLR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR6_A {
        match self.bits {
            false => OUTCLR6_A::NOP,
            true => OUTCLR6_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR6_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR6_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR6`"]
pub struct OUTCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR6_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR6_A::OUT0)
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
#[doc = "Port Data Output Value Clear 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR7_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR7_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR7`"]
pub type OUTCLR7_R = crate::R<bool, OUTCLR7_A>;
impl OUTCLR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR7_A {
        match self.bits {
            false => OUTCLR7_A::NOP,
            true => OUTCLR7_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR7_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR7_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR7`"]
pub struct OUTCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR7_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR7_A::OUT0)
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
#[doc = "Port Data Output Value Clear 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR8_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR8_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR8`"]
pub type OUTCLR8_R = crate::R<bool, OUTCLR8_A>;
impl OUTCLR8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR8_A {
        match self.bits {
            false => OUTCLR8_A::NOP,
            true => OUTCLR8_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR8_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR8_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR8`"]
pub struct OUTCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR8_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR8_A::OUT0)
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
#[doc = "Port Data Output Value Clear 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR9_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR9_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR9`"]
pub type OUTCLR9_R = crate::R<bool, OUTCLR9_A>;
impl OUTCLR9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR9_A {
        match self.bits {
            false => OUTCLR9_A::NOP,
            true => OUTCLR9_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR9_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR9_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR9`"]
pub struct OUTCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR9_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR9_A::OUT0)
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
#[doc = "Port Data Output Value Clear 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR10_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR10_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR10`"]
pub type OUTCLR10_R = crate::R<bool, OUTCLR10_A>;
impl OUTCLR10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR10_A {
        match self.bits {
            false => OUTCLR10_A::NOP,
            true => OUTCLR10_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR10_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR10_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR10`"]
pub struct OUTCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR10_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR10_A::OUT0)
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
#[doc = "Port Data Output Value Clear 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR11_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR11_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR11`"]
pub type OUTCLR11_R = crate::R<bool, OUTCLR11_A>;
impl OUTCLR11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR11_A {
        match self.bits {
            false => OUTCLR11_A::NOP,
            true => OUTCLR11_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR11_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR11_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR11`"]
pub struct OUTCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR11_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR11_A::OUT0)
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
#[doc = "Port Data Output Value Clear 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR12_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR12_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR12`"]
pub type OUTCLR12_R = crate::R<bool, OUTCLR12_A>;
impl OUTCLR12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR12_A {
        match self.bits {
            false => OUTCLR12_A::NOP,
            true => OUTCLR12_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR12_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR12_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR12`"]
pub struct OUTCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR12_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR12_A::OUT0)
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
#[doc = "Port Data Output Value Clear 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR13_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR13_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR13`"]
pub type OUTCLR13_R = crate::R<bool, OUTCLR13_A>;
impl OUTCLR13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR13_A {
        match self.bits {
            false => OUTCLR13_A::NOP,
            true => OUTCLR13_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR13_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR13_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR13`"]
pub struct OUTCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR13_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR13_A::OUT0)
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
#[doc = "Port Data Output Value Clear 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR14_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR14_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR14`"]
pub type OUTCLR14_R = crate::R<bool, OUTCLR14_A>;
impl OUTCLR14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR14_A {
        match self.bits {
            false => OUTCLR14_A::NOP,
            true => OUTCLR14_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR14_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR14_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR14`"]
pub struct OUTCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR14_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR14_A::OUT0)
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
#[doc = "Port Data Output Value Clear 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR15_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR15_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR15`"]
pub type OUTCLR15_R = crate::R<bool, OUTCLR15_A>;
impl OUTCLR15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR15_A {
        match self.bits {
            false => OUTCLR15_A::NOP,
            true => OUTCLR15_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR15_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR15_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR15`"]
pub struct OUTCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR15_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR15_A::OUT0)
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
#[doc = "Port Data Output Value Clear 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR16_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR16_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR16`"]
pub type OUTCLR16_R = crate::R<bool, OUTCLR16_A>;
impl OUTCLR16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR16_A {
        match self.bits {
            false => OUTCLR16_A::NOP,
            true => OUTCLR16_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR16_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR16_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR16`"]
pub struct OUTCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR16_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR16_A::OUT0)
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
#[doc = "Port Data Output Value Clear 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR17_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR17_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR17`"]
pub type OUTCLR17_R = crate::R<bool, OUTCLR17_A>;
impl OUTCLR17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR17_A {
        match self.bits {
            false => OUTCLR17_A::NOP,
            true => OUTCLR17_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR17_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR17_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR17`"]
pub struct OUTCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR17_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR17_A::OUT0)
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
#[doc = "Port Data Output Value Clear 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR18_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR18_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR18`"]
pub type OUTCLR18_R = crate::R<bool, OUTCLR18_A>;
impl OUTCLR18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR18_A {
        match self.bits {
            false => OUTCLR18_A::NOP,
            true => OUTCLR18_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR18_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR18_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR18`"]
pub struct OUTCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR18_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR18_A::OUT0)
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
#[doc = "Port Data Output Value Clear 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR19_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR19_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR19`"]
pub type OUTCLR19_R = crate::R<bool, OUTCLR19_A>;
impl OUTCLR19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR19_A {
        match self.bits {
            false => OUTCLR19_A::NOP,
            true => OUTCLR19_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR19_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR19_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR19`"]
pub struct OUTCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR19_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR19_A::OUT0)
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
#[doc = "Port Data Output Value Clear 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR20_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR20_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR20`"]
pub type OUTCLR20_R = crate::R<bool, OUTCLR20_A>;
impl OUTCLR20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR20_A {
        match self.bits {
            false => OUTCLR20_A::NOP,
            true => OUTCLR20_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR20_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR20_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR20`"]
pub struct OUTCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR20_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR20_A::OUT0)
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
#[doc = "Port Data Output Value Clear 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR21_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR21_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR21`"]
pub type OUTCLR21_R = crate::R<bool, OUTCLR21_A>;
impl OUTCLR21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR21_A {
        match self.bits {
            false => OUTCLR21_A::NOP,
            true => OUTCLR21_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR21_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR21_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR21`"]
pub struct OUTCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR21_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR21_A::OUT0)
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
#[doc = "Port Data Output Value Clear 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR22_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR22_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR22`"]
pub type OUTCLR22_R = crate::R<bool, OUTCLR22_A>;
impl OUTCLR22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR22_A {
        match self.bits {
            false => OUTCLR22_A::NOP,
            true => OUTCLR22_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR22_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR22_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR22`"]
pub struct OUTCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR22_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR22_A::OUT0)
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
#[doc = "Port Data Output Value Clear 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR23_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR23_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR23`"]
pub type OUTCLR23_R = crate::R<bool, OUTCLR23_A>;
impl OUTCLR23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR23_A {
        match self.bits {
            false => OUTCLR23_A::NOP,
            true => OUTCLR23_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR23_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR23_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR23`"]
pub struct OUTCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR23_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR23_A::OUT0)
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
#[doc = "Port Data Output Value Clear 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR24_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR24_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR24`"]
pub type OUTCLR24_R = crate::R<bool, OUTCLR24_A>;
impl OUTCLR24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR24_A {
        match self.bits {
            false => OUTCLR24_A::NOP,
            true => OUTCLR24_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR24_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR24_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR24`"]
pub struct OUTCLR24_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR24_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR24_A::OUT0)
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
#[doc = "Port Data Output Value Clear 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR25_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR25_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR25`"]
pub type OUTCLR25_R = crate::R<bool, OUTCLR25_A>;
impl OUTCLR25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR25_A {
        match self.bits {
            false => OUTCLR25_A::NOP,
            true => OUTCLR25_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR25_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR25_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR25`"]
pub struct OUTCLR25_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR25_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR25_A::OUT0)
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
#[doc = "Port Data Output Value Clear 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR26_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR26_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR26`"]
pub type OUTCLR26_R = crate::R<bool, OUTCLR26_A>;
impl OUTCLR26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR26_A {
        match self.bits {
            false => OUTCLR26_A::NOP,
            true => OUTCLR26_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR26_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR26_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR26`"]
pub struct OUTCLR26_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR26_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR26_A::OUT0)
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
#[doc = "Port Data Output Value Clear 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR27_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR27_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR27`"]
pub type OUTCLR27_R = crate::R<bool, OUTCLR27_A>;
impl OUTCLR27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR27_A {
        match self.bits {
            false => OUTCLR27_A::NOP,
            true => OUTCLR27_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR27_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR27_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR27`"]
pub struct OUTCLR27_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR27_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR27_A::OUT0)
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
#[doc = "Port Data Output Value Clear 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR28_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR28_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR28`"]
pub type OUTCLR28_R = crate::R<bool, OUTCLR28_A>;
impl OUTCLR28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR28_A {
        match self.bits {
            false => OUTCLR28_A::NOP,
            true => OUTCLR28_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR28_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR28_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR28`"]
pub struct OUTCLR28_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR28_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR28_A::OUT0)
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
#[doc = "Port Data Output Value Clear 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR29_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR29_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR29`"]
pub type OUTCLR29_R = crate::R<bool, OUTCLR29_A>;
impl OUTCLR29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR29_A {
        match self.bits {
            false => OUTCLR29_A::NOP,
            true => OUTCLR29_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR29_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR29_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR29`"]
pub struct OUTCLR29_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR29_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR29_A::OUT0)
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
#[doc = "Port Data Output Value Clear 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR30_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR30_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR30`"]
pub type OUTCLR30_R = crate::R<bool, OUTCLR30_A>;
impl OUTCLR30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR30_A {
        match self.bits {
            false => OUTCLR30_A::NOP,
            true => OUTCLR30_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR30_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR30_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR30`"]
pub struct OUTCLR30_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR30_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR30_A::OUT0)
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
#[doc = "Port Data Output Value Clear 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCLR31_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output 0"]
    OUT0 = 1,
}
impl From<OUTCLR31_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCLR31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCLR31`"]
pub type OUTCLR31_R = crate::R<bool, OUTCLR31_A>;
impl OUTCLR31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCLR31_A {
        match self.bits {
            false => OUTCLR31_A::NOP,
            true => OUTCLR31_A::OUT0,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTCLR31_A::NOP
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTCLR31_A::OUT0
    }
}
#[doc = "Write proxy for field `OUTCLR31`"]
pub struct OUTCLR31_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCLR31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCLR31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTCLR31_A::NOP)
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTCLR31_A::OUT0)
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
    #[doc = "Bit 0 - Port Data Output Value Clear 0"]
    #[inline(always)]
    pub fn outclr0(&self) -> OUTCLR0_R {
        OUTCLR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Output Value Clear 1"]
    #[inline(always)]
    pub fn outclr1(&self) -> OUTCLR1_R {
        OUTCLR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Output Value Clear 2"]
    #[inline(always)]
    pub fn outclr2(&self) -> OUTCLR2_R {
        OUTCLR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Output Value Clear 3"]
    #[inline(always)]
    pub fn outclr3(&self) -> OUTCLR3_R {
        OUTCLR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Output Value Clear 4"]
    #[inline(always)]
    pub fn outclr4(&self) -> OUTCLR4_R {
        OUTCLR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Output Value Clear 5"]
    #[inline(always)]
    pub fn outclr5(&self) -> OUTCLR5_R {
        OUTCLR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Output Value Clear 6"]
    #[inline(always)]
    pub fn outclr6(&self) -> OUTCLR6_R {
        OUTCLR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Output Value Clear 7"]
    #[inline(always)]
    pub fn outclr7(&self) -> OUTCLR7_R {
        OUTCLR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Output Value Clear 8"]
    #[inline(always)]
    pub fn outclr8(&self) -> OUTCLR8_R {
        OUTCLR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Output Value Clear 9"]
    #[inline(always)]
    pub fn outclr9(&self) -> OUTCLR9_R {
        OUTCLR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Output Value Clear 10"]
    #[inline(always)]
    pub fn outclr10(&self) -> OUTCLR10_R {
        OUTCLR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Output Value Clear 11"]
    #[inline(always)]
    pub fn outclr11(&self) -> OUTCLR11_R {
        OUTCLR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Output Value Clear 12"]
    #[inline(always)]
    pub fn outclr12(&self) -> OUTCLR12_R {
        OUTCLR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Output Value Clear 13"]
    #[inline(always)]
    pub fn outclr13(&self) -> OUTCLR13_R {
        OUTCLR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Output Value Clear 14"]
    #[inline(always)]
    pub fn outclr14(&self) -> OUTCLR14_R {
        OUTCLR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Output Value Clear 15"]
    #[inline(always)]
    pub fn outclr15(&self) -> OUTCLR15_R {
        OUTCLR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Output Value Clear 16"]
    #[inline(always)]
    pub fn outclr16(&self) -> OUTCLR16_R {
        OUTCLR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Output Value Clear 17"]
    #[inline(always)]
    pub fn outclr17(&self) -> OUTCLR17_R {
        OUTCLR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Output Value Clear 18"]
    #[inline(always)]
    pub fn outclr18(&self) -> OUTCLR18_R {
        OUTCLR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Output Value Clear 19"]
    #[inline(always)]
    pub fn outclr19(&self) -> OUTCLR19_R {
        OUTCLR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Output Value Clear 20"]
    #[inline(always)]
    pub fn outclr20(&self) -> OUTCLR20_R {
        OUTCLR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Output Value Clear 21"]
    #[inline(always)]
    pub fn outclr21(&self) -> OUTCLR21_R {
        OUTCLR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Output Value Clear 22"]
    #[inline(always)]
    pub fn outclr22(&self) -> OUTCLR22_R {
        OUTCLR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Output Value Clear 23"]
    #[inline(always)]
    pub fn outclr23(&self) -> OUTCLR23_R {
        OUTCLR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Output Value Clear 24"]
    #[inline(always)]
    pub fn outclr24(&self) -> OUTCLR24_R {
        OUTCLR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Output Value Clear 25"]
    #[inline(always)]
    pub fn outclr25(&self) -> OUTCLR25_R {
        OUTCLR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Output Value Clear 26"]
    #[inline(always)]
    pub fn outclr26(&self) -> OUTCLR26_R {
        OUTCLR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Output Value Clear 27"]
    #[inline(always)]
    pub fn outclr27(&self) -> OUTCLR27_R {
        OUTCLR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Output Value Clear 28"]
    #[inline(always)]
    pub fn outclr28(&self) -> OUTCLR28_R {
        OUTCLR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Output Value Clear 29"]
    #[inline(always)]
    pub fn outclr29(&self) -> OUTCLR29_R {
        OUTCLR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Output Value Clear 30"]
    #[inline(always)]
    pub fn outclr30(&self) -> OUTCLR30_R {
        OUTCLR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Output Value Clear 31"]
    #[inline(always)]
    pub fn outclr31(&self) -> OUTCLR31_R {
        OUTCLR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Output Value Clear 0"]
    #[inline(always)]
    pub fn outclr0(&mut self) -> OUTCLR0_W {
        OUTCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Output Value Clear 1"]
    #[inline(always)]
    pub fn outclr1(&mut self) -> OUTCLR1_W {
        OUTCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Output Value Clear 2"]
    #[inline(always)]
    pub fn outclr2(&mut self) -> OUTCLR2_W {
        OUTCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Output Value Clear 3"]
    #[inline(always)]
    pub fn outclr3(&mut self) -> OUTCLR3_W {
        OUTCLR3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Output Value Clear 4"]
    #[inline(always)]
    pub fn outclr4(&mut self) -> OUTCLR4_W {
        OUTCLR4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Output Value Clear 5"]
    #[inline(always)]
    pub fn outclr5(&mut self) -> OUTCLR5_W {
        OUTCLR5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Output Value Clear 6"]
    #[inline(always)]
    pub fn outclr6(&mut self) -> OUTCLR6_W {
        OUTCLR6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Output Value Clear 7"]
    #[inline(always)]
    pub fn outclr7(&mut self) -> OUTCLR7_W {
        OUTCLR7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Output Value Clear 8"]
    #[inline(always)]
    pub fn outclr8(&mut self) -> OUTCLR8_W {
        OUTCLR8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Output Value Clear 9"]
    #[inline(always)]
    pub fn outclr9(&mut self) -> OUTCLR9_W {
        OUTCLR9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Output Value Clear 10"]
    #[inline(always)]
    pub fn outclr10(&mut self) -> OUTCLR10_W {
        OUTCLR10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Output Value Clear 11"]
    #[inline(always)]
    pub fn outclr11(&mut self) -> OUTCLR11_W {
        OUTCLR11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Output Value Clear 12"]
    #[inline(always)]
    pub fn outclr12(&mut self) -> OUTCLR12_W {
        OUTCLR12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Output Value Clear 13"]
    #[inline(always)]
    pub fn outclr13(&mut self) -> OUTCLR13_W {
        OUTCLR13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Output Value Clear 14"]
    #[inline(always)]
    pub fn outclr14(&mut self) -> OUTCLR14_W {
        OUTCLR14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Output Value Clear 15"]
    #[inline(always)]
    pub fn outclr15(&mut self) -> OUTCLR15_W {
        OUTCLR15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Output Value Clear 16"]
    #[inline(always)]
    pub fn outclr16(&mut self) -> OUTCLR16_W {
        OUTCLR16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Output Value Clear 17"]
    #[inline(always)]
    pub fn outclr17(&mut self) -> OUTCLR17_W {
        OUTCLR17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Output Value Clear 18"]
    #[inline(always)]
    pub fn outclr18(&mut self) -> OUTCLR18_W {
        OUTCLR18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Output Value Clear 19"]
    #[inline(always)]
    pub fn outclr19(&mut self) -> OUTCLR19_W {
        OUTCLR19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Output Value Clear 20"]
    #[inline(always)]
    pub fn outclr20(&mut self) -> OUTCLR20_W {
        OUTCLR20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Output Value Clear 21"]
    #[inline(always)]
    pub fn outclr21(&mut self) -> OUTCLR21_W {
        OUTCLR21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Output Value Clear 22"]
    #[inline(always)]
    pub fn outclr22(&mut self) -> OUTCLR22_W {
        OUTCLR22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Output Value Clear 23"]
    #[inline(always)]
    pub fn outclr23(&mut self) -> OUTCLR23_W {
        OUTCLR23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Output Value Clear 24"]
    #[inline(always)]
    pub fn outclr24(&mut self) -> OUTCLR24_W {
        OUTCLR24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Output Value Clear 25"]
    #[inline(always)]
    pub fn outclr25(&mut self) -> OUTCLR25_W {
        OUTCLR25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Output Value Clear 26"]
    #[inline(always)]
    pub fn outclr26(&mut self) -> OUTCLR26_W {
        OUTCLR26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Output Value Clear 27"]
    #[inline(always)]
    pub fn outclr27(&mut self) -> OUTCLR27_W {
        OUTCLR27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Output Value Clear 28"]
    #[inline(always)]
    pub fn outclr28(&mut self) -> OUTCLR28_W {
        OUTCLR28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Output Value Clear 29"]
    #[inline(always)]
    pub fn outclr29(&mut self) -> OUTCLR29_W {
        OUTCLR29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Output Value Clear 30"]
    #[inline(always)]
    pub fn outclr30(&mut self) -> OUTCLR30_W {
        OUTCLR30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Output Value Clear 31"]
    #[inline(always)]
    pub fn outclr31(&mut self) -> OUTCLR31_W {
        OUTCLR31_W { w: self }
    }
}
