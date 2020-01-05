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
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
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
            false => OUTTGL0_A::_0,
            true => OUTTGL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUTTGL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUTTGL0_A::_1
    }
}
#[doc = "Port Data Output Value Toggle 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTTGL0_AW {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Output toggle"]
    TOGGLE = 1,
}
impl From<OUTTGL0_AW> for bool {
    #[inline(always)]
    fn from(variant: OUTTGL0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `OUTTGL0`"]
pub struct OUTTGL0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 1"]
pub type OUTTGL1_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL1`"]
pub type OUTTGL1_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 1"]
pub type OUTTGL1_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL1`"]
pub struct OUTTGL1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 2"]
pub type OUTTGL2_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL2`"]
pub type OUTTGL2_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 2"]
pub type OUTTGL2_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL2`"]
pub struct OUTTGL2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 3"]
pub type OUTTGL3_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL3`"]
pub type OUTTGL3_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 3"]
pub type OUTTGL3_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL3`"]
pub struct OUTTGL3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 4"]
pub type OUTTGL4_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL4`"]
pub type OUTTGL4_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 4"]
pub type OUTTGL4_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL4`"]
pub struct OUTTGL4_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 5"]
pub type OUTTGL5_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL5`"]
pub type OUTTGL5_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 5"]
pub type OUTTGL5_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL5`"]
pub struct OUTTGL5_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 6"]
pub type OUTTGL6_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL6`"]
pub type OUTTGL6_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 6"]
pub type OUTTGL6_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL6`"]
pub struct OUTTGL6_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 7"]
pub type OUTTGL7_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL7`"]
pub type OUTTGL7_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 7"]
pub type OUTTGL7_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL7`"]
pub struct OUTTGL7_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 8"]
pub type OUTTGL8_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL8`"]
pub type OUTTGL8_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 8"]
pub type OUTTGL8_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL8`"]
pub struct OUTTGL8_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL8_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 9"]
pub type OUTTGL9_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL9`"]
pub type OUTTGL9_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 9"]
pub type OUTTGL9_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL9`"]
pub struct OUTTGL9_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL9_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 10"]
pub type OUTTGL10_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL10`"]
pub type OUTTGL10_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 10"]
pub type OUTTGL10_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL10`"]
pub struct OUTTGL10_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL10_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 11"]
pub type OUTTGL11_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL11`"]
pub type OUTTGL11_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 11"]
pub type OUTTGL11_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL11`"]
pub struct OUTTGL11_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL11_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 12"]
pub type OUTTGL12_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL12`"]
pub type OUTTGL12_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 12"]
pub type OUTTGL12_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL12`"]
pub struct OUTTGL12_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL12_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 13"]
pub type OUTTGL13_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL13`"]
pub type OUTTGL13_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 13"]
pub type OUTTGL13_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL13`"]
pub struct OUTTGL13_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL13_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 14"]
pub type OUTTGL14_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL14`"]
pub type OUTTGL14_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 14"]
pub type OUTTGL14_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL14`"]
pub struct OUTTGL14_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL14_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 15"]
pub type OUTTGL15_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL15`"]
pub type OUTTGL15_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 15"]
pub type OUTTGL15_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL15`"]
pub struct OUTTGL15_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL15_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 16"]
pub type OUTTGL16_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL16`"]
pub type OUTTGL16_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 16"]
pub type OUTTGL16_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL16`"]
pub struct OUTTGL16_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL16_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 17"]
pub type OUTTGL17_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL17`"]
pub type OUTTGL17_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 17"]
pub type OUTTGL17_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL17`"]
pub struct OUTTGL17_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL17_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 18"]
pub type OUTTGL18_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL18`"]
pub type OUTTGL18_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 18"]
pub type OUTTGL18_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL18`"]
pub struct OUTTGL18_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL18_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 19"]
pub type OUTTGL19_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL19`"]
pub type OUTTGL19_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 19"]
pub type OUTTGL19_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL19`"]
pub struct OUTTGL19_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL19_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 20"]
pub type OUTTGL20_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL20`"]
pub type OUTTGL20_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 20"]
pub type OUTTGL20_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL20`"]
pub struct OUTTGL20_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL20_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 21"]
pub type OUTTGL21_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL21`"]
pub type OUTTGL21_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 21"]
pub type OUTTGL21_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL21`"]
pub struct OUTTGL21_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL21_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 22"]
pub type OUTTGL22_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL22`"]
pub type OUTTGL22_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 22"]
pub type OUTTGL22_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL22`"]
pub struct OUTTGL22_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL22_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 23"]
pub type OUTTGL23_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL23`"]
pub type OUTTGL23_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 23"]
pub type OUTTGL23_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL23`"]
pub struct OUTTGL23_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL23_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 24"]
pub type OUTTGL24_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL24`"]
pub type OUTTGL24_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 24"]
pub type OUTTGL24_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL24`"]
pub struct OUTTGL24_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL24_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 25"]
pub type OUTTGL25_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL25`"]
pub type OUTTGL25_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 25"]
pub type OUTTGL25_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL25`"]
pub struct OUTTGL25_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL25_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 26"]
pub type OUTTGL26_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL26`"]
pub type OUTTGL26_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 26"]
pub type OUTTGL26_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL26`"]
pub struct OUTTGL26_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL26_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 27"]
pub type OUTTGL27_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL27`"]
pub type OUTTGL27_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 27"]
pub type OUTTGL27_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL27`"]
pub struct OUTTGL27_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL27_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 28"]
pub type OUTTGL28_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL28`"]
pub type OUTTGL28_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 28"]
pub type OUTTGL28_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL28`"]
pub struct OUTTGL28_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL28_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 29"]
pub type OUTTGL29_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL29`"]
pub type OUTTGL29_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 29"]
pub type OUTTGL29_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL29`"]
pub struct OUTTGL29_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL29_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 30"]
pub type OUTTGL30_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL30`"]
pub type OUTTGL30_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 30"]
pub type OUTTGL30_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL30`"]
pub struct OUTTGL30_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL30_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
#[doc = "Port Data Output Value Toggle 31"]
pub type OUTTGL31_A = OUTTGL0_A;
#[doc = "Reader of field `OUTTGL31`"]
pub type OUTTGL31_R = crate::R<bool, OUTTGL0_A>;
#[doc = "Port Data Output Value Toggle 31"]
pub type OUTTGL31_AW = OUTTGL0_AW;
#[doc = "Write proxy for field `OUTTGL31`"]
pub struct OUTTGL31_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTTGL31_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::NOP)
    }
    #[doc = "Output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OUTTGL0_AW::TOGGLE)
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
