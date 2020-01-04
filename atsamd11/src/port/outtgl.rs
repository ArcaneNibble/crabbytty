#[doc = "Reader of register OUTTGL%s"]
pub type R = crate::R<u32, super::OUTTGL>;
#[doc = "Writer for register OUTTGL%s"]
pub type W = crate::W<u32, super::OUTTGL>;
#[doc = "Register OUTTGL%s `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTTGL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Output Value Toggle 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL0_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL0_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL0`"]
pub type OUTTGL0_R = crate::R<bool, OUTTGL0_A>;
impl OUTTGL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL0_A {
        match self.bits {
            false => OUTTGL0_A::NOP,
            true => OUTTGL0_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL0_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL0_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL0`"]
pub struct OUTTGL0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL1_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL1_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL1`"]
pub type OUTTGL1_R = crate::R<bool, OUTTGL1_A>;
impl OUTTGL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL1_A {
        match self.bits {
            false => OUTTGL1_A::NOP,
            true => OUTTGL1_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL1_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL1_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL1`"]
pub struct OUTTGL1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL1_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL1_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL2_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL2_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL2`"]
pub type OUTTGL2_R = crate::R<bool, OUTTGL2_A>;
impl OUTTGL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL2_A {
        match self.bits {
            false => OUTTGL2_A::NOP,
            true => OUTTGL2_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL2_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL2_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL2`"]
pub struct OUTTGL2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL2_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL2_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL3_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL3_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL3`"]
pub type OUTTGL3_R = crate::R<bool, OUTTGL3_A>;
impl OUTTGL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL3_A {
        match self.bits {
            false => OUTTGL3_A::NOP,
            true => OUTTGL3_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL3_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL3_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL3`"]
pub struct OUTTGL3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL3_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL3_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL4_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL4_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL4`"]
pub type OUTTGL4_R = crate::R<bool, OUTTGL4_A>;
impl OUTTGL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL4_A {
        match self.bits {
            false => OUTTGL4_A::NOP,
            true => OUTTGL4_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL4_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL4_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL4`"]
pub struct OUTTGL4_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL4_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL4_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL5_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL5_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL5`"]
pub type OUTTGL5_R = crate::R<bool, OUTTGL5_A>;
impl OUTTGL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL5_A {
        match self.bits {
            false => OUTTGL5_A::NOP,
            true => OUTTGL5_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL5_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL5_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL5`"]
pub struct OUTTGL5_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL5_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL5_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL6_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL6_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL6`"]
pub type OUTTGL6_R = crate::R<bool, OUTTGL6_A>;
impl OUTTGL6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL6_A {
        match self.bits {
            false => OUTTGL6_A::NOP,
            true => OUTTGL6_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL6_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL6_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL6`"]
pub struct OUTTGL6_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL6_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL6_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL7_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL7_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL7`"]
pub type OUTTGL7_R = crate::R<bool, OUTTGL7_A>;
impl OUTTGL7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL7_A {
        match self.bits {
            false => OUTTGL7_A::NOP,
            true => OUTTGL7_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL7_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL7_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL7`"]
pub struct OUTTGL7_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL7_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL7_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL8_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL8_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL8`"]
pub type OUTTGL8_R = crate::R<bool, OUTTGL8_A>;
impl OUTTGL8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL8_A {
        match self.bits {
            false => OUTTGL8_A::NOP,
            true => OUTTGL8_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL8_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL8_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL8`"]
pub struct OUTTGL8_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL8_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL8_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL9_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL9_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL9`"]
pub type OUTTGL9_R = crate::R<bool, OUTTGL9_A>;
impl OUTTGL9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL9_A {
        match self.bits {
            false => OUTTGL9_A::NOP,
            true => OUTTGL9_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL9_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL9_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL9`"]
pub struct OUTTGL9_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL9_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL9_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL10_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL10_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL10`"]
pub type OUTTGL10_R = crate::R<bool, OUTTGL10_A>;
impl OUTTGL10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL10_A {
        match self.bits {
            false => OUTTGL10_A::NOP,
            true => OUTTGL10_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL10_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL10_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL10`"]
pub struct OUTTGL10_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL10_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL10_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL11_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL11_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL11`"]
pub type OUTTGL11_R = crate::R<bool, OUTTGL11_A>;
impl OUTTGL11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL11_A {
        match self.bits {
            false => OUTTGL11_A::NOP,
            true => OUTTGL11_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL11_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL11_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL11`"]
pub struct OUTTGL11_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL11_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL11_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL12_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL12_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL12`"]
pub type OUTTGL12_R = crate::R<bool, OUTTGL12_A>;
impl OUTTGL12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL12_A {
        match self.bits {
            false => OUTTGL12_A::NOP,
            true => OUTTGL12_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL12_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL12_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL12`"]
pub struct OUTTGL12_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL12_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL12_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL13_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL13_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL13`"]
pub type OUTTGL13_R = crate::R<bool, OUTTGL13_A>;
impl OUTTGL13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL13_A {
        match self.bits {
            false => OUTTGL13_A::NOP,
            true => OUTTGL13_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL13_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL13_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL13`"]
pub struct OUTTGL13_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL13_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL13_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL14_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL14_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL14`"]
pub type OUTTGL14_R = crate::R<bool, OUTTGL14_A>;
impl OUTTGL14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL14_A {
        match self.bits {
            false => OUTTGL14_A::NOP,
            true => OUTTGL14_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL14_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL14_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL14`"]
pub struct OUTTGL14_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL14_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL14_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL15_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL15_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL15`"]
pub type OUTTGL15_R = crate::R<bool, OUTTGL15_A>;
impl OUTTGL15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL15_A {
        match self.bits {
            false => OUTTGL15_A::NOP,
            true => OUTTGL15_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL15_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL15_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL15`"]
pub struct OUTTGL15_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL15_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL15_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL16_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL16_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL16`"]
pub type OUTTGL16_R = crate::R<bool, OUTTGL16_A>;
impl OUTTGL16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL16_A {
        match self.bits {
            false => OUTTGL16_A::NOP,
            true => OUTTGL16_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL16_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL16_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL16`"]
pub struct OUTTGL16_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL16_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL16_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL17_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL17_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL17`"]
pub type OUTTGL17_R = crate::R<bool, OUTTGL17_A>;
impl OUTTGL17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL17_A {
        match self.bits {
            false => OUTTGL17_A::NOP,
            true => OUTTGL17_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL17_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL17_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL17`"]
pub struct OUTTGL17_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL17_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL17_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL18_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL18_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL18`"]
pub type OUTTGL18_R = crate::R<bool, OUTTGL18_A>;
impl OUTTGL18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL18_A {
        match self.bits {
            false => OUTTGL18_A::NOP,
            true => OUTTGL18_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL18_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL18_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL18`"]
pub struct OUTTGL18_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL18_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL18_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL19_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL19_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL19`"]
pub type OUTTGL19_R = crate::R<bool, OUTTGL19_A>;
impl OUTTGL19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL19_A {
        match self.bits {
            false => OUTTGL19_A::NOP,
            true => OUTTGL19_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL19_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL19_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL19`"]
pub struct OUTTGL19_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL19_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL19_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL20_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL20_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL20`"]
pub type OUTTGL20_R = crate::R<bool, OUTTGL20_A>;
impl OUTTGL20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL20_A {
        match self.bits {
            false => OUTTGL20_A::NOP,
            true => OUTTGL20_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL20_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL20_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL20`"]
pub struct OUTTGL20_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL20_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL20_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL21_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL21_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL21`"]
pub type OUTTGL21_R = crate::R<bool, OUTTGL21_A>;
impl OUTTGL21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL21_A {
        match self.bits {
            false => OUTTGL21_A::NOP,
            true => OUTTGL21_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL21_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL21_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL21`"]
pub struct OUTTGL21_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL21_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL21_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL22_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL22_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL22`"]
pub type OUTTGL22_R = crate::R<bool, OUTTGL22_A>;
impl OUTTGL22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL22_A {
        match self.bits {
            false => OUTTGL22_A::NOP,
            true => OUTTGL22_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL22_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL22_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL22`"]
pub struct OUTTGL22_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL22_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL22_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL23_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL23_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL23`"]
pub type OUTTGL23_R = crate::R<bool, OUTTGL23_A>;
impl OUTTGL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL23_A {
        match self.bits {
            false => OUTTGL23_A::NOP,
            true => OUTTGL23_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL23_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL23_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL23`"]
pub struct OUTTGL23_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL23_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL23_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL24_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL24_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL24`"]
pub type OUTTGL24_R = crate::R<bool, OUTTGL24_A>;
impl OUTTGL24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL24_A {
        match self.bits {
            false => OUTTGL24_A::NOP,
            true => OUTTGL24_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL24_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL24_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL24`"]
pub struct OUTTGL24_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL24_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL24_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL25_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL25_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL25`"]
pub type OUTTGL25_R = crate::R<bool, OUTTGL25_A>;
impl OUTTGL25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL25_A {
        match self.bits {
            false => OUTTGL25_A::NOP,
            true => OUTTGL25_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL25_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL25_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL25`"]
pub struct OUTTGL25_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL25_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL25_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL26_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL26_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL26`"]
pub type OUTTGL26_R = crate::R<bool, OUTTGL26_A>;
impl OUTTGL26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL26_A {
        match self.bits {
            false => OUTTGL26_A::NOP,
            true => OUTTGL26_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL26_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL26_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL26`"]
pub struct OUTTGL26_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL26_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL26_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL27_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL27_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL27`"]
pub type OUTTGL27_R = crate::R<bool, OUTTGL27_A>;
impl OUTTGL27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL27_A {
        match self.bits {
            false => OUTTGL27_A::NOP,
            true => OUTTGL27_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL27_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL27_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL27`"]
pub struct OUTTGL27_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL27_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL27_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL28_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL28_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL28`"]
pub type OUTTGL28_R = crate::R<bool, OUTTGL28_A>;
impl OUTTGL28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL28_A {
        match self.bits {
            false => OUTTGL28_A::NOP,
            true => OUTTGL28_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL28_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL28_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL28`"]
pub struct OUTTGL28_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL28_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL28_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL29_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL29_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL29`"]
pub type OUTTGL29_R = crate::R<bool, OUTTGL29_A>;
impl OUTTGL29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL29_A {
        match self.bits {
            false => OUTTGL29_A::NOP,
            true => OUTTGL29_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL29_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL29_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL29`"]
pub struct OUTTGL29_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL29_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL29_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL30_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL30_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL30`"]
pub type OUTTGL30_R = crate::R<bool, OUTTGL30_A>;
impl OUTTGL30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL30_A {
        match self.bits {
            false => OUTTGL30_A::NOP,
            true => OUTTGL30_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL30_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL30_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL30`"]
pub struct OUTTGL30_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL30_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL30_A::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL31_A {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL31_A> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTTGL31`"]
pub type OUTTGL31_R = crate::R<bool, OUTTGL31_A>;
impl OUTTGL31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTTGL31_A {
        match self.bits {
            false => OUTTGL31_A::NOP,
            true => OUTTGL31_A::TOGGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == OUTTGL31_A::NOP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OUTTGL31_A::TOGGLE
    }
}
#[doc = "Write proxy for field `OUTTGL31`"]
pub struct OUTTGL31_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL31_A::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL31_A::TOGGLE)
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
    #[doc = "Bit 0 - Port Data Output Value Toggle 0"]
    #[inline(always)]
    pub fn outtgl0(&self) -> OUTTGL0_R {
        OUTTGL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Output Value Toggle 1"]
    #[inline(always)]
    pub fn outtgl1(&self) -> OUTTGL1_R {
        OUTTGL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Output Value Toggle 2"]
    #[inline(always)]
    pub fn outtgl2(&self) -> OUTTGL2_R {
        OUTTGL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Output Value Toggle 3"]
    #[inline(always)]
    pub fn outtgl3(&self) -> OUTTGL3_R {
        OUTTGL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Output Value Toggle 4"]
    #[inline(always)]
    pub fn outtgl4(&self) -> OUTTGL4_R {
        OUTTGL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Output Value Toggle 5"]
    #[inline(always)]
    pub fn outtgl5(&self) -> OUTTGL5_R {
        OUTTGL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Output Value Toggle 6"]
    #[inline(always)]
    pub fn outtgl6(&self) -> OUTTGL6_R {
        OUTTGL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Output Value Toggle 7"]
    #[inline(always)]
    pub fn outtgl7(&self) -> OUTTGL7_R {
        OUTTGL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Output Value Toggle 8"]
    #[inline(always)]
    pub fn outtgl8(&self) -> OUTTGL8_R {
        OUTTGL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Output Value Toggle 9"]
    #[inline(always)]
    pub fn outtgl9(&self) -> OUTTGL9_R {
        OUTTGL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Output Value Toggle 10"]
    #[inline(always)]
    pub fn outtgl10(&self) -> OUTTGL10_R {
        OUTTGL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Output Value Toggle 11"]
    #[inline(always)]
    pub fn outtgl11(&self) -> OUTTGL11_R {
        OUTTGL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Output Value Toggle 12"]
    #[inline(always)]
    pub fn outtgl12(&self) -> OUTTGL12_R {
        OUTTGL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Output Value Toggle 13"]
    #[inline(always)]
    pub fn outtgl13(&self) -> OUTTGL13_R {
        OUTTGL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Output Value Toggle 14"]
    #[inline(always)]
    pub fn outtgl14(&self) -> OUTTGL14_R {
        OUTTGL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Output Value Toggle 15"]
    #[inline(always)]
    pub fn outtgl15(&self) -> OUTTGL15_R {
        OUTTGL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Output Value Toggle 16"]
    #[inline(always)]
    pub fn outtgl16(&self) -> OUTTGL16_R {
        OUTTGL16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Output Value Toggle 17"]
    #[inline(always)]
    pub fn outtgl17(&self) -> OUTTGL17_R {
        OUTTGL17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Output Value Toggle 18"]
    #[inline(always)]
    pub fn outtgl18(&self) -> OUTTGL18_R {
        OUTTGL18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Output Value Toggle 19"]
    #[inline(always)]
    pub fn outtgl19(&self) -> OUTTGL19_R {
        OUTTGL19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Output Value Toggle 20"]
    #[inline(always)]
    pub fn outtgl20(&self) -> OUTTGL20_R {
        OUTTGL20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Output Value Toggle 21"]
    #[inline(always)]
    pub fn outtgl21(&self) -> OUTTGL21_R {
        OUTTGL21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Output Value Toggle 22"]
    #[inline(always)]
    pub fn outtgl22(&self) -> OUTTGL22_R {
        OUTTGL22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Output Value Toggle 23"]
    #[inline(always)]
    pub fn outtgl23(&self) -> OUTTGL23_R {
        OUTTGL23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Output Value Toggle 24"]
    #[inline(always)]
    pub fn outtgl24(&self) -> OUTTGL24_R {
        OUTTGL24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Output Value Toggle 25"]
    #[inline(always)]
    pub fn outtgl25(&self) -> OUTTGL25_R {
        OUTTGL25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Output Value Toggle 26"]
    #[inline(always)]
    pub fn outtgl26(&self) -> OUTTGL26_R {
        OUTTGL26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Output Value Toggle 27"]
    #[inline(always)]
    pub fn outtgl27(&self) -> OUTTGL27_R {
        OUTTGL27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Output Value Toggle 28"]
    #[inline(always)]
    pub fn outtgl28(&self) -> OUTTGL28_R {
        OUTTGL28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Output Value Toggle 29"]
    #[inline(always)]
    pub fn outtgl29(&self) -> OUTTGL29_R {
        OUTTGL29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Output Value Toggle 30"]
    #[inline(always)]
    pub fn outtgl30(&self) -> OUTTGL30_R {
        OUTTGL30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Output Value Toggle 31"]
    #[inline(always)]
    pub fn outtgl31(&self) -> OUTTGL31_R {
        OUTTGL31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Output Value Toggle 0"]
    #[inline(always)]
    pub fn outtgl0(&mut self) -> OUTTGL0_W {
        OUTTGL0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Output Value Toggle 1"]
    #[inline(always)]
    pub fn outtgl1(&mut self) -> OUTTGL1_W {
        OUTTGL1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Output Value Toggle 2"]
    #[inline(always)]
    pub fn outtgl2(&mut self) -> OUTTGL2_W {
        OUTTGL2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Output Value Toggle 3"]
    #[inline(always)]
    pub fn outtgl3(&mut self) -> OUTTGL3_W {
        OUTTGL3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Output Value Toggle 4"]
    #[inline(always)]
    pub fn outtgl4(&mut self) -> OUTTGL4_W {
        OUTTGL4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Output Value Toggle 5"]
    #[inline(always)]
    pub fn outtgl5(&mut self) -> OUTTGL5_W {
        OUTTGL5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Output Value Toggle 6"]
    #[inline(always)]
    pub fn outtgl6(&mut self) -> OUTTGL6_W {
        OUTTGL6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Output Value Toggle 7"]
    #[inline(always)]
    pub fn outtgl7(&mut self) -> OUTTGL7_W {
        OUTTGL7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Output Value Toggle 8"]
    #[inline(always)]
    pub fn outtgl8(&mut self) -> OUTTGL8_W {
        OUTTGL8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Output Value Toggle 9"]
    #[inline(always)]
    pub fn outtgl9(&mut self) -> OUTTGL9_W {
        OUTTGL9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Output Value Toggle 10"]
    #[inline(always)]
    pub fn outtgl10(&mut self) -> OUTTGL10_W {
        OUTTGL10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Output Value Toggle 11"]
    #[inline(always)]
    pub fn outtgl11(&mut self) -> OUTTGL11_W {
        OUTTGL11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Output Value Toggle 12"]
    #[inline(always)]
    pub fn outtgl12(&mut self) -> OUTTGL12_W {
        OUTTGL12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Output Value Toggle 13"]
    #[inline(always)]
    pub fn outtgl13(&mut self) -> OUTTGL13_W {
        OUTTGL13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Output Value Toggle 14"]
    #[inline(always)]
    pub fn outtgl14(&mut self) -> OUTTGL14_W {
        OUTTGL14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Output Value Toggle 15"]
    #[inline(always)]
    pub fn outtgl15(&mut self) -> OUTTGL15_W {
        OUTTGL15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Output Value Toggle 16"]
    #[inline(always)]
    pub fn outtgl16(&mut self) -> OUTTGL16_W {
        OUTTGL16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Output Value Toggle 17"]
    #[inline(always)]
    pub fn outtgl17(&mut self) -> OUTTGL17_W {
        OUTTGL17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Output Value Toggle 18"]
    #[inline(always)]
    pub fn outtgl18(&mut self) -> OUTTGL18_W {
        OUTTGL18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Output Value Toggle 19"]
    #[inline(always)]
    pub fn outtgl19(&mut self) -> OUTTGL19_W {
        OUTTGL19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Output Value Toggle 20"]
    #[inline(always)]
    pub fn outtgl20(&mut self) -> OUTTGL20_W {
        OUTTGL20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Output Value Toggle 21"]
    #[inline(always)]
    pub fn outtgl21(&mut self) -> OUTTGL21_W {
        OUTTGL21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Output Value Toggle 22"]
    #[inline(always)]
    pub fn outtgl22(&mut self) -> OUTTGL22_W {
        OUTTGL22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Output Value Toggle 23"]
    #[inline(always)]
    pub fn outtgl23(&mut self) -> OUTTGL23_W {
        OUTTGL23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Output Value Toggle 24"]
    #[inline(always)]
    pub fn outtgl24(&mut self) -> OUTTGL24_W {
        OUTTGL24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Output Value Toggle 25"]
    #[inline(always)]
    pub fn outtgl25(&mut self) -> OUTTGL25_W {
        OUTTGL25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Output Value Toggle 26"]
    #[inline(always)]
    pub fn outtgl26(&mut self) -> OUTTGL26_W {
        OUTTGL26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Output Value Toggle 27"]
    #[inline(always)]
    pub fn outtgl27(&mut self) -> OUTTGL27_W {
        OUTTGL27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Output Value Toggle 28"]
    #[inline(always)]
    pub fn outtgl28(&mut self) -> OUTTGL28_W {
        OUTTGL28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Output Value Toggle 29"]
    #[inline(always)]
    pub fn outtgl29(&mut self) -> OUTTGL29_W {
        OUTTGL29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Output Value Toggle 30"]
    #[inline(always)]
    pub fn outtgl30(&mut self) -> OUTTGL30_W {
        OUTTGL30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Output Value Toggle 31"]
    #[inline(always)]
    pub fn outtgl31(&mut self) -> OUTTGL31_W {
        OUTTGL31_W { w: self }
    }
}
