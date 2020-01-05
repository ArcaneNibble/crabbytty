#[doc = "Reader of register CTRL%s"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL%s"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Input Sampling Mode 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLING0_A {
    #[doc = "0: Only on reads"]
    ONREAD = 0,
    #[doc = "1: Continuously"]
    CONTINUOUS = 1,
}
impl From<SAMPLING0_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLING0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAMPLING0`"]
pub type SAMPLING0_R = crate::R<bool, SAMPLING0_A>;
impl SAMPLING0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLING0_A {
        match self.bits {
            false => SAMPLING0_A::ONREAD,
            true => SAMPLING0_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `ONREAD`"]
    #[inline(always)]
    pub fn is_onread(&self) -> bool {
        *self == SAMPLING0_A::ONREAD
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == SAMPLING0_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `SAMPLING0`"]
pub struct SAMPLING0_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 1"]
pub type SAMPLING1_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING1`"]
pub type SAMPLING1_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING1`"]
pub struct SAMPLING1_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 2"]
pub type SAMPLING2_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING2`"]
pub type SAMPLING2_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING2`"]
pub struct SAMPLING2_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 3"]
pub type SAMPLING3_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING3`"]
pub type SAMPLING3_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING3`"]
pub struct SAMPLING3_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 4"]
pub type SAMPLING4_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING4`"]
pub type SAMPLING4_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING4`"]
pub struct SAMPLING4_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 5"]
pub type SAMPLING5_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING5`"]
pub type SAMPLING5_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING5`"]
pub struct SAMPLING5_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 6"]
pub type SAMPLING6_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING6`"]
pub type SAMPLING6_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING6`"]
pub struct SAMPLING6_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 7"]
pub type SAMPLING7_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING7`"]
pub type SAMPLING7_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING7`"]
pub struct SAMPLING7_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 8"]
pub type SAMPLING8_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING8`"]
pub type SAMPLING8_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING8`"]
pub struct SAMPLING8_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 9"]
pub type SAMPLING9_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING9`"]
pub type SAMPLING9_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING9`"]
pub struct SAMPLING9_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 10"]
pub type SAMPLING10_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING10`"]
pub type SAMPLING10_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING10`"]
pub struct SAMPLING10_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 11"]
pub type SAMPLING11_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING11`"]
pub type SAMPLING11_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING11`"]
pub struct SAMPLING11_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 12"]
pub type SAMPLING12_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING12`"]
pub type SAMPLING12_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING12`"]
pub struct SAMPLING12_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 13"]
pub type SAMPLING13_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING13`"]
pub type SAMPLING13_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING13`"]
pub struct SAMPLING13_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 14"]
pub type SAMPLING14_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING14`"]
pub type SAMPLING14_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING14`"]
pub struct SAMPLING14_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 15"]
pub type SAMPLING15_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING15`"]
pub type SAMPLING15_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING15`"]
pub struct SAMPLING15_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 16"]
pub type SAMPLING16_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING16`"]
pub type SAMPLING16_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING16`"]
pub struct SAMPLING16_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 17"]
pub type SAMPLING17_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING17`"]
pub type SAMPLING17_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING17`"]
pub struct SAMPLING17_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 18"]
pub type SAMPLING18_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING18`"]
pub type SAMPLING18_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING18`"]
pub struct SAMPLING18_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 19"]
pub type SAMPLING19_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING19`"]
pub type SAMPLING19_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING19`"]
pub struct SAMPLING19_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 20"]
pub type SAMPLING20_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING20`"]
pub type SAMPLING20_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING20`"]
pub struct SAMPLING20_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 21"]
pub type SAMPLING21_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING21`"]
pub type SAMPLING21_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING21`"]
pub struct SAMPLING21_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 22"]
pub type SAMPLING22_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING22`"]
pub type SAMPLING22_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING22`"]
pub struct SAMPLING22_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 23"]
pub type SAMPLING23_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING23`"]
pub type SAMPLING23_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING23`"]
pub struct SAMPLING23_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 24"]
pub type SAMPLING24_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING24`"]
pub type SAMPLING24_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING24`"]
pub struct SAMPLING24_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 25"]
pub type SAMPLING25_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING25`"]
pub type SAMPLING25_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING25`"]
pub struct SAMPLING25_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 26"]
pub type SAMPLING26_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING26`"]
pub type SAMPLING26_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING26`"]
pub struct SAMPLING26_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 27"]
pub type SAMPLING27_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING27`"]
pub type SAMPLING27_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING27`"]
pub struct SAMPLING27_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 28"]
pub type SAMPLING28_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING28`"]
pub type SAMPLING28_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING28`"]
pub struct SAMPLING28_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 29"]
pub type SAMPLING29_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING29`"]
pub type SAMPLING29_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING29`"]
pub struct SAMPLING29_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 30"]
pub type SAMPLING30_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING30`"]
pub type SAMPLING30_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING30`"]
pub struct SAMPLING30_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
#[doc = "Input Sampling Mode 31"]
pub type SAMPLING31_A = SAMPLING0_A;
#[doc = "Reader of field `SAMPLING31`"]
pub type SAMPLING31_R = crate::R<bool, SAMPLING0_A>;
#[doc = "Write proxy for field `SAMPLING31`"]
pub struct SAMPLING31_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLING31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLING31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only on reads"]
    #[inline(always)]
    pub fn onread(self) -> &'a mut W {
        self.variant(SAMPLING0_A::ONREAD)
    }
    #[doc = "Continuously"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SAMPLING0_A::CONTINUOUS)
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
    #[doc = "Bit 0 - Input Sampling Mode 0"]
    #[inline(always)]
    pub fn sampling0(&self) -> SAMPLING0_R {
        SAMPLING0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Input Sampling Mode 1"]
    #[inline(always)]
    pub fn sampling1(&self) -> SAMPLING1_R {
        SAMPLING1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input Sampling Mode 2"]
    #[inline(always)]
    pub fn sampling2(&self) -> SAMPLING2_R {
        SAMPLING2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Input Sampling Mode 3"]
    #[inline(always)]
    pub fn sampling3(&self) -> SAMPLING3_R {
        SAMPLING3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Input Sampling Mode 4"]
    #[inline(always)]
    pub fn sampling4(&self) -> SAMPLING4_R {
        SAMPLING4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Input Sampling Mode 5"]
    #[inline(always)]
    pub fn sampling5(&self) -> SAMPLING5_R {
        SAMPLING5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Input Sampling Mode 6"]
    #[inline(always)]
    pub fn sampling6(&self) -> SAMPLING6_R {
        SAMPLING6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Input Sampling Mode 7"]
    #[inline(always)]
    pub fn sampling7(&self) -> SAMPLING7_R {
        SAMPLING7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Input Sampling Mode 8"]
    #[inline(always)]
    pub fn sampling8(&self) -> SAMPLING8_R {
        SAMPLING8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Input Sampling Mode 9"]
    #[inline(always)]
    pub fn sampling9(&self) -> SAMPLING9_R {
        SAMPLING9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Input Sampling Mode 10"]
    #[inline(always)]
    pub fn sampling10(&self) -> SAMPLING10_R {
        SAMPLING10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Input Sampling Mode 11"]
    #[inline(always)]
    pub fn sampling11(&self) -> SAMPLING11_R {
        SAMPLING11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Input Sampling Mode 12"]
    #[inline(always)]
    pub fn sampling12(&self) -> SAMPLING12_R {
        SAMPLING12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Input Sampling Mode 13"]
    #[inline(always)]
    pub fn sampling13(&self) -> SAMPLING13_R {
        SAMPLING13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Input Sampling Mode 14"]
    #[inline(always)]
    pub fn sampling14(&self) -> SAMPLING14_R {
        SAMPLING14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Input Sampling Mode 15"]
    #[inline(always)]
    pub fn sampling15(&self) -> SAMPLING15_R {
        SAMPLING15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input Sampling Mode 16"]
    #[inline(always)]
    pub fn sampling16(&self) -> SAMPLING16_R {
        SAMPLING16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Input Sampling Mode 17"]
    #[inline(always)]
    pub fn sampling17(&self) -> SAMPLING17_R {
        SAMPLING17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Input Sampling Mode 18"]
    #[inline(always)]
    pub fn sampling18(&self) -> SAMPLING18_R {
        SAMPLING18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Input Sampling Mode 19"]
    #[inline(always)]
    pub fn sampling19(&self) -> SAMPLING19_R {
        SAMPLING19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Input Sampling Mode 20"]
    #[inline(always)]
    pub fn sampling20(&self) -> SAMPLING20_R {
        SAMPLING20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Input Sampling Mode 21"]
    #[inline(always)]
    pub fn sampling21(&self) -> SAMPLING21_R {
        SAMPLING21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Input Sampling Mode 22"]
    #[inline(always)]
    pub fn sampling22(&self) -> SAMPLING22_R {
        SAMPLING22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Input Sampling Mode 23"]
    #[inline(always)]
    pub fn sampling23(&self) -> SAMPLING23_R {
        SAMPLING23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Input Sampling Mode 24"]
    #[inline(always)]
    pub fn sampling24(&self) -> SAMPLING24_R {
        SAMPLING24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Input Sampling Mode 25"]
    #[inline(always)]
    pub fn sampling25(&self) -> SAMPLING25_R {
        SAMPLING25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Input Sampling Mode 26"]
    #[inline(always)]
    pub fn sampling26(&self) -> SAMPLING26_R {
        SAMPLING26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Input Sampling Mode 27"]
    #[inline(always)]
    pub fn sampling27(&self) -> SAMPLING27_R {
        SAMPLING27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Input Sampling Mode 28"]
    #[inline(always)]
    pub fn sampling28(&self) -> SAMPLING28_R {
        SAMPLING28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Input Sampling Mode 29"]
    #[inline(always)]
    pub fn sampling29(&self) -> SAMPLING29_R {
        SAMPLING29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Input Sampling Mode 30"]
    #[inline(always)]
    pub fn sampling30(&self) -> SAMPLING30_R {
        SAMPLING30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Input Sampling Mode 31"]
    #[inline(always)]
    pub fn sampling31(&self) -> SAMPLING31_R {
        SAMPLING31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Sampling Mode 0"]
    #[inline(always)]
    pub fn sampling0(&mut self) -> SAMPLING0_W {
        SAMPLING0_W { w: self }
    }
    #[doc = "Bit 1 - Input Sampling Mode 1"]
    #[inline(always)]
    pub fn sampling1(&mut self) -> SAMPLING1_W {
        SAMPLING1_W { w: self }
    }
    #[doc = "Bit 2 - Input Sampling Mode 2"]
    #[inline(always)]
    pub fn sampling2(&mut self) -> SAMPLING2_W {
        SAMPLING2_W { w: self }
    }
    #[doc = "Bit 3 - Input Sampling Mode 3"]
    #[inline(always)]
    pub fn sampling3(&mut self) -> SAMPLING3_W {
        SAMPLING3_W { w: self }
    }
    #[doc = "Bit 4 - Input Sampling Mode 4"]
    #[inline(always)]
    pub fn sampling4(&mut self) -> SAMPLING4_W {
        SAMPLING4_W { w: self }
    }
    #[doc = "Bit 5 - Input Sampling Mode 5"]
    #[inline(always)]
    pub fn sampling5(&mut self) -> SAMPLING5_W {
        SAMPLING5_W { w: self }
    }
    #[doc = "Bit 6 - Input Sampling Mode 6"]
    #[inline(always)]
    pub fn sampling6(&mut self) -> SAMPLING6_W {
        SAMPLING6_W { w: self }
    }
    #[doc = "Bit 7 - Input Sampling Mode 7"]
    #[inline(always)]
    pub fn sampling7(&mut self) -> SAMPLING7_W {
        SAMPLING7_W { w: self }
    }
    #[doc = "Bit 8 - Input Sampling Mode 8"]
    #[inline(always)]
    pub fn sampling8(&mut self) -> SAMPLING8_W {
        SAMPLING8_W { w: self }
    }
    #[doc = "Bit 9 - Input Sampling Mode 9"]
    #[inline(always)]
    pub fn sampling9(&mut self) -> SAMPLING9_W {
        SAMPLING9_W { w: self }
    }
    #[doc = "Bit 10 - Input Sampling Mode 10"]
    #[inline(always)]
    pub fn sampling10(&mut self) -> SAMPLING10_W {
        SAMPLING10_W { w: self }
    }
    #[doc = "Bit 11 - Input Sampling Mode 11"]
    #[inline(always)]
    pub fn sampling11(&mut self) -> SAMPLING11_W {
        SAMPLING11_W { w: self }
    }
    #[doc = "Bit 12 - Input Sampling Mode 12"]
    #[inline(always)]
    pub fn sampling12(&mut self) -> SAMPLING12_W {
        SAMPLING12_W { w: self }
    }
    #[doc = "Bit 13 - Input Sampling Mode 13"]
    #[inline(always)]
    pub fn sampling13(&mut self) -> SAMPLING13_W {
        SAMPLING13_W { w: self }
    }
    #[doc = "Bit 14 - Input Sampling Mode 14"]
    #[inline(always)]
    pub fn sampling14(&mut self) -> SAMPLING14_W {
        SAMPLING14_W { w: self }
    }
    #[doc = "Bit 15 - Input Sampling Mode 15"]
    #[inline(always)]
    pub fn sampling15(&mut self) -> SAMPLING15_W {
        SAMPLING15_W { w: self }
    }
    #[doc = "Bit 16 - Input Sampling Mode 16"]
    #[inline(always)]
    pub fn sampling16(&mut self) -> SAMPLING16_W {
        SAMPLING16_W { w: self }
    }
    #[doc = "Bit 17 - Input Sampling Mode 17"]
    #[inline(always)]
    pub fn sampling17(&mut self) -> SAMPLING17_W {
        SAMPLING17_W { w: self }
    }
    #[doc = "Bit 18 - Input Sampling Mode 18"]
    #[inline(always)]
    pub fn sampling18(&mut self) -> SAMPLING18_W {
        SAMPLING18_W { w: self }
    }
    #[doc = "Bit 19 - Input Sampling Mode 19"]
    #[inline(always)]
    pub fn sampling19(&mut self) -> SAMPLING19_W {
        SAMPLING19_W { w: self }
    }
    #[doc = "Bit 20 - Input Sampling Mode 20"]
    #[inline(always)]
    pub fn sampling20(&mut self) -> SAMPLING20_W {
        SAMPLING20_W { w: self }
    }
    #[doc = "Bit 21 - Input Sampling Mode 21"]
    #[inline(always)]
    pub fn sampling21(&mut self) -> SAMPLING21_W {
        SAMPLING21_W { w: self }
    }
    #[doc = "Bit 22 - Input Sampling Mode 22"]
    #[inline(always)]
    pub fn sampling22(&mut self) -> SAMPLING22_W {
        SAMPLING22_W { w: self }
    }
    #[doc = "Bit 23 - Input Sampling Mode 23"]
    #[inline(always)]
    pub fn sampling23(&mut self) -> SAMPLING23_W {
        SAMPLING23_W { w: self }
    }
    #[doc = "Bit 24 - Input Sampling Mode 24"]
    #[inline(always)]
    pub fn sampling24(&mut self) -> SAMPLING24_W {
        SAMPLING24_W { w: self }
    }
    #[doc = "Bit 25 - Input Sampling Mode 25"]
    #[inline(always)]
    pub fn sampling25(&mut self) -> SAMPLING25_W {
        SAMPLING25_W { w: self }
    }
    #[doc = "Bit 26 - Input Sampling Mode 26"]
    #[inline(always)]
    pub fn sampling26(&mut self) -> SAMPLING26_W {
        SAMPLING26_W { w: self }
    }
    #[doc = "Bit 27 - Input Sampling Mode 27"]
    #[inline(always)]
    pub fn sampling27(&mut self) -> SAMPLING27_W {
        SAMPLING27_W { w: self }
    }
    #[doc = "Bit 28 - Input Sampling Mode 28"]
    #[inline(always)]
    pub fn sampling28(&mut self) -> SAMPLING28_W {
        SAMPLING28_W { w: self }
    }
    #[doc = "Bit 29 - Input Sampling Mode 29"]
    #[inline(always)]
    pub fn sampling29(&mut self) -> SAMPLING29_W {
        SAMPLING29_W { w: self }
    }
    #[doc = "Bit 30 - Input Sampling Mode 30"]
    #[inline(always)]
    pub fn sampling30(&mut self) -> SAMPLING30_W {
        SAMPLING30_W { w: self }
    }
    #[doc = "Bit 31 - Input Sampling Mode 31"]
    #[inline(always)]
    pub fn sampling31(&mut self) -> SAMPLING31_W {
        SAMPLING31_W { w: self }
    }
}
