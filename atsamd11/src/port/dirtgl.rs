#[doc = "Reader of register DIRTGL%s"]
pub type R = crate::R<u32, super::DIRTGL>;
#[doc = "Writer for register DIRTGL%s"]
pub type W = crate::W<u32, super::DIRTGL>;
#[doc = "Register DIRTGL%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DIRTGL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Direction Toggle 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRTGL0_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIRTGL0_A> for bool {
    #[inline(always)]
    fn from(variant: DIRTGL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRTGL0`"]
pub type DIRTGL0_R = crate::R<bool, DIRTGL0_A>;
impl DIRTGL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRTGL0_A {
        match self.bits {
            false => DIRTGL0_A::INPUT,
            true => DIRTGL0_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIRTGL0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIRTGL0_A::OUTPUT
    }
}
#[doc = "Port Data Direction Toggle 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRTGL0_AW {
    #[doc = "0: No effect"]
    NOP = 0,
    #[doc = "1: Toggle pin direction"]
    TOGGLE = 1,
}
impl From<DIRTGL0_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRTGL0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DIRTGL0`"]
pub struct DIRTGL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 1"]
pub type DIRTGL1_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL1`"]
pub type DIRTGL1_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 1"]
pub type DIRTGL1_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL1`"]
pub struct DIRTGL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 2"]
pub type DIRTGL2_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL2`"]
pub type DIRTGL2_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 2"]
pub type DIRTGL2_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL2`"]
pub struct DIRTGL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 3"]
pub type DIRTGL3_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL3`"]
pub type DIRTGL3_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 3"]
pub type DIRTGL3_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL3`"]
pub struct DIRTGL3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 4"]
pub type DIRTGL4_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL4`"]
pub type DIRTGL4_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 4"]
pub type DIRTGL4_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL4`"]
pub struct DIRTGL4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 5"]
pub type DIRTGL5_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL5`"]
pub type DIRTGL5_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 5"]
pub type DIRTGL5_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL5`"]
pub struct DIRTGL5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 6"]
pub type DIRTGL6_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL6`"]
pub type DIRTGL6_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 6"]
pub type DIRTGL6_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL6`"]
pub struct DIRTGL6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 7"]
pub type DIRTGL7_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL7`"]
pub type DIRTGL7_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 7"]
pub type DIRTGL7_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL7`"]
pub struct DIRTGL7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 8"]
pub type DIRTGL8_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL8`"]
pub type DIRTGL8_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 8"]
pub type DIRTGL8_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL8`"]
pub struct DIRTGL8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL8_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 9"]
pub type DIRTGL9_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL9`"]
pub type DIRTGL9_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 9"]
pub type DIRTGL9_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL9`"]
pub struct DIRTGL9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL9_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 10"]
pub type DIRTGL10_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL10`"]
pub type DIRTGL10_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 10"]
pub type DIRTGL10_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL10`"]
pub struct DIRTGL10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL10_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 11"]
pub type DIRTGL11_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL11`"]
pub type DIRTGL11_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 11"]
pub type DIRTGL11_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL11`"]
pub struct DIRTGL11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL11_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 12"]
pub type DIRTGL12_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL12`"]
pub type DIRTGL12_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 12"]
pub type DIRTGL12_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL12`"]
pub struct DIRTGL12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL12_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 13"]
pub type DIRTGL13_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL13`"]
pub type DIRTGL13_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 13"]
pub type DIRTGL13_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL13`"]
pub struct DIRTGL13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL13_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 14"]
pub type DIRTGL14_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL14`"]
pub type DIRTGL14_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 14"]
pub type DIRTGL14_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL14`"]
pub struct DIRTGL14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL14_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 15"]
pub type DIRTGL15_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL15`"]
pub type DIRTGL15_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 15"]
pub type DIRTGL15_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL15`"]
pub struct DIRTGL15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL15_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 16"]
pub type DIRTGL16_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL16`"]
pub type DIRTGL16_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 16"]
pub type DIRTGL16_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL16`"]
pub struct DIRTGL16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL16_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 17"]
pub type DIRTGL17_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL17`"]
pub type DIRTGL17_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 17"]
pub type DIRTGL17_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL17`"]
pub struct DIRTGL17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL17_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 18"]
pub type DIRTGL18_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL18`"]
pub type DIRTGL18_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 18"]
pub type DIRTGL18_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL18`"]
pub struct DIRTGL18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL18_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 19"]
pub type DIRTGL19_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL19`"]
pub type DIRTGL19_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 19"]
pub type DIRTGL19_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL19`"]
pub struct DIRTGL19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL19_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 20"]
pub type DIRTGL20_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL20`"]
pub type DIRTGL20_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 20"]
pub type DIRTGL20_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL20`"]
pub struct DIRTGL20_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL20_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 21"]
pub type DIRTGL21_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL21`"]
pub type DIRTGL21_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 21"]
pub type DIRTGL21_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL21`"]
pub struct DIRTGL21_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL21_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 22"]
pub type DIRTGL22_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL22`"]
pub type DIRTGL22_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 22"]
pub type DIRTGL22_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL22`"]
pub struct DIRTGL22_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL22_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 23"]
pub type DIRTGL23_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL23`"]
pub type DIRTGL23_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 23"]
pub type DIRTGL23_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL23`"]
pub struct DIRTGL23_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL23_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 24"]
pub type DIRTGL24_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL24`"]
pub type DIRTGL24_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 24"]
pub type DIRTGL24_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL24`"]
pub struct DIRTGL24_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL24_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 25"]
pub type DIRTGL25_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL25`"]
pub type DIRTGL25_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 25"]
pub type DIRTGL25_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL25`"]
pub struct DIRTGL25_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL25_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 26"]
pub type DIRTGL26_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL26`"]
pub type DIRTGL26_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 26"]
pub type DIRTGL26_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL26`"]
pub struct DIRTGL26_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL26_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 27"]
pub type DIRTGL27_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL27`"]
pub type DIRTGL27_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 27"]
pub type DIRTGL27_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL27`"]
pub struct DIRTGL27_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL27_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 28"]
pub type DIRTGL28_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL28`"]
pub type DIRTGL28_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 28"]
pub type DIRTGL28_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL28`"]
pub struct DIRTGL28_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL28_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 29"]
pub type DIRTGL29_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL29`"]
pub type DIRTGL29_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 29"]
pub type DIRTGL29_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL29`"]
pub struct DIRTGL29_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL29_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 30"]
pub type DIRTGL30_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL30`"]
pub type DIRTGL30_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 30"]
pub type DIRTGL30_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL30`"]
pub struct DIRTGL30_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL30_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
#[doc = "Port Data Direction Toggle 31"]
pub type DIRTGL31_A = DIRTGL0_A;
#[doc = "Reader of field `DIRTGL31`"]
pub type DIRTGL31_R = crate::R<bool, DIRTGL0_A>;
#[doc = "Port Data Direction Toggle 31"]
pub type DIRTGL31_AW = DIRTGL0_AW;
#[doc = "Write proxy for field `DIRTGL31`"]
pub struct DIRTGL31_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRTGL31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRTGL31_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::NOP)
    }
    #[doc = "Toggle pin direction"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(DIRTGL0_AW::TOGGLE)
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
    #[doc = "Bit 0 - Port Data Direction Toggle 0"]
    #[inline(always)]
    pub fn dirtgl0(&self) -> DIRTGL0_R {
        DIRTGL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Direction Toggle 1"]
    #[inline(always)]
    pub fn dirtgl1(&self) -> DIRTGL1_R {
        DIRTGL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Direction Toggle 2"]
    #[inline(always)]
    pub fn dirtgl2(&self) -> DIRTGL2_R {
        DIRTGL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Direction Toggle 3"]
    #[inline(always)]
    pub fn dirtgl3(&self) -> DIRTGL3_R {
        DIRTGL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Direction Toggle 4"]
    #[inline(always)]
    pub fn dirtgl4(&self) -> DIRTGL4_R {
        DIRTGL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Direction Toggle 5"]
    #[inline(always)]
    pub fn dirtgl5(&self) -> DIRTGL5_R {
        DIRTGL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Direction Toggle 6"]
    #[inline(always)]
    pub fn dirtgl6(&self) -> DIRTGL6_R {
        DIRTGL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Direction Toggle 7"]
    #[inline(always)]
    pub fn dirtgl7(&self) -> DIRTGL7_R {
        DIRTGL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Direction Toggle 8"]
    #[inline(always)]
    pub fn dirtgl8(&self) -> DIRTGL8_R {
        DIRTGL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Direction Toggle 9"]
    #[inline(always)]
    pub fn dirtgl9(&self) -> DIRTGL9_R {
        DIRTGL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Direction Toggle 10"]
    #[inline(always)]
    pub fn dirtgl10(&self) -> DIRTGL10_R {
        DIRTGL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Direction Toggle 11"]
    #[inline(always)]
    pub fn dirtgl11(&self) -> DIRTGL11_R {
        DIRTGL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Direction Toggle 12"]
    #[inline(always)]
    pub fn dirtgl12(&self) -> DIRTGL12_R {
        DIRTGL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Direction Toggle 13"]
    #[inline(always)]
    pub fn dirtgl13(&self) -> DIRTGL13_R {
        DIRTGL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Direction Toggle 14"]
    #[inline(always)]
    pub fn dirtgl14(&self) -> DIRTGL14_R {
        DIRTGL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Direction Toggle 15"]
    #[inline(always)]
    pub fn dirtgl15(&self) -> DIRTGL15_R {
        DIRTGL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Direction Toggle 16"]
    #[inline(always)]
    pub fn dirtgl16(&self) -> DIRTGL16_R {
        DIRTGL16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Direction Toggle 17"]
    #[inline(always)]
    pub fn dirtgl17(&self) -> DIRTGL17_R {
        DIRTGL17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Direction Toggle 18"]
    #[inline(always)]
    pub fn dirtgl18(&self) -> DIRTGL18_R {
        DIRTGL18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Direction Toggle 19"]
    #[inline(always)]
    pub fn dirtgl19(&self) -> DIRTGL19_R {
        DIRTGL19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Direction Toggle 20"]
    #[inline(always)]
    pub fn dirtgl20(&self) -> DIRTGL20_R {
        DIRTGL20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Direction Toggle 21"]
    #[inline(always)]
    pub fn dirtgl21(&self) -> DIRTGL21_R {
        DIRTGL21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Direction Toggle 22"]
    #[inline(always)]
    pub fn dirtgl22(&self) -> DIRTGL22_R {
        DIRTGL22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Direction Toggle 23"]
    #[inline(always)]
    pub fn dirtgl23(&self) -> DIRTGL23_R {
        DIRTGL23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Direction Toggle 24"]
    #[inline(always)]
    pub fn dirtgl24(&self) -> DIRTGL24_R {
        DIRTGL24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Direction Toggle 25"]
    #[inline(always)]
    pub fn dirtgl25(&self) -> DIRTGL25_R {
        DIRTGL25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Direction Toggle 26"]
    #[inline(always)]
    pub fn dirtgl26(&self) -> DIRTGL26_R {
        DIRTGL26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Direction Toggle 27"]
    #[inline(always)]
    pub fn dirtgl27(&self) -> DIRTGL27_R {
        DIRTGL27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Direction Toggle 28"]
    #[inline(always)]
    pub fn dirtgl28(&self) -> DIRTGL28_R {
        DIRTGL28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Direction Toggle 29"]
    #[inline(always)]
    pub fn dirtgl29(&self) -> DIRTGL29_R {
        DIRTGL29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Direction Toggle 30"]
    #[inline(always)]
    pub fn dirtgl30(&self) -> DIRTGL30_R {
        DIRTGL30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Direction Toggle 31"]
    #[inline(always)]
    pub fn dirtgl31(&self) -> DIRTGL31_R {
        DIRTGL31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Direction Toggle 0"]
    #[inline(always)]
    pub fn dirtgl0(&mut self) -> DIRTGL0_W {
        DIRTGL0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Direction Toggle 1"]
    #[inline(always)]
    pub fn dirtgl1(&mut self) -> DIRTGL1_W {
        DIRTGL1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Direction Toggle 2"]
    #[inline(always)]
    pub fn dirtgl2(&mut self) -> DIRTGL2_W {
        DIRTGL2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Direction Toggle 3"]
    #[inline(always)]
    pub fn dirtgl3(&mut self) -> DIRTGL3_W {
        DIRTGL3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Direction Toggle 4"]
    #[inline(always)]
    pub fn dirtgl4(&mut self) -> DIRTGL4_W {
        DIRTGL4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Direction Toggle 5"]
    #[inline(always)]
    pub fn dirtgl5(&mut self) -> DIRTGL5_W {
        DIRTGL5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Direction Toggle 6"]
    #[inline(always)]
    pub fn dirtgl6(&mut self) -> DIRTGL6_W {
        DIRTGL6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Direction Toggle 7"]
    #[inline(always)]
    pub fn dirtgl7(&mut self) -> DIRTGL7_W {
        DIRTGL7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Direction Toggle 8"]
    #[inline(always)]
    pub fn dirtgl8(&mut self) -> DIRTGL8_W {
        DIRTGL8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Direction Toggle 9"]
    #[inline(always)]
    pub fn dirtgl9(&mut self) -> DIRTGL9_W {
        DIRTGL9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Direction Toggle 10"]
    #[inline(always)]
    pub fn dirtgl10(&mut self) -> DIRTGL10_W {
        DIRTGL10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Direction Toggle 11"]
    #[inline(always)]
    pub fn dirtgl11(&mut self) -> DIRTGL11_W {
        DIRTGL11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Direction Toggle 12"]
    #[inline(always)]
    pub fn dirtgl12(&mut self) -> DIRTGL12_W {
        DIRTGL12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Direction Toggle 13"]
    #[inline(always)]
    pub fn dirtgl13(&mut self) -> DIRTGL13_W {
        DIRTGL13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Direction Toggle 14"]
    #[inline(always)]
    pub fn dirtgl14(&mut self) -> DIRTGL14_W {
        DIRTGL14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Direction Toggle 15"]
    #[inline(always)]
    pub fn dirtgl15(&mut self) -> DIRTGL15_W {
        DIRTGL15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Direction Toggle 16"]
    #[inline(always)]
    pub fn dirtgl16(&mut self) -> DIRTGL16_W {
        DIRTGL16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Direction Toggle 17"]
    #[inline(always)]
    pub fn dirtgl17(&mut self) -> DIRTGL17_W {
        DIRTGL17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Direction Toggle 18"]
    #[inline(always)]
    pub fn dirtgl18(&mut self) -> DIRTGL18_W {
        DIRTGL18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Direction Toggle 19"]
    #[inline(always)]
    pub fn dirtgl19(&mut self) -> DIRTGL19_W {
        DIRTGL19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Direction Toggle 20"]
    #[inline(always)]
    pub fn dirtgl20(&mut self) -> DIRTGL20_W {
        DIRTGL20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Direction Toggle 21"]
    #[inline(always)]
    pub fn dirtgl21(&mut self) -> DIRTGL21_W {
        DIRTGL21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Direction Toggle 22"]
    #[inline(always)]
    pub fn dirtgl22(&mut self) -> DIRTGL22_W {
        DIRTGL22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Direction Toggle 23"]
    #[inline(always)]
    pub fn dirtgl23(&mut self) -> DIRTGL23_W {
        DIRTGL23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Direction Toggle 24"]
    #[inline(always)]
    pub fn dirtgl24(&mut self) -> DIRTGL24_W {
        DIRTGL24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Direction Toggle 25"]
    #[inline(always)]
    pub fn dirtgl25(&mut self) -> DIRTGL25_W {
        DIRTGL25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Direction Toggle 26"]
    #[inline(always)]
    pub fn dirtgl26(&mut self) -> DIRTGL26_W {
        DIRTGL26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Direction Toggle 27"]
    #[inline(always)]
    pub fn dirtgl27(&mut self) -> DIRTGL27_W {
        DIRTGL27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Direction Toggle 28"]
    #[inline(always)]
    pub fn dirtgl28(&mut self) -> DIRTGL28_W {
        DIRTGL28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Direction Toggle 29"]
    #[inline(always)]
    pub fn dirtgl29(&mut self) -> DIRTGL29_W {
        DIRTGL29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Direction Toggle 30"]
    #[inline(always)]
    pub fn dirtgl30(&mut self) -> DIRTGL30_W {
        DIRTGL30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Direction Toggle 31"]
    #[inline(always)]
    pub fn dirtgl31(&mut self) -> DIRTGL31_W {
        DIRTGL31_W { w: self }
    }
}
