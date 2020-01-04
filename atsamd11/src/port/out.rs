#[doc = "Reader of register OUT%s"]
pub type R = crate::R<u32, super::OUT>;
#[doc = "Writer for register OUT%s"]
pub type W = crate::W<u32, super::OUT>;
#[doc = "Register OUT%s `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Output Value 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT0_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT0_A> for bool {
    #[inline(always)]
    fn from(variant: OUT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT0`"]
pub type OUT0_R = crate::R<bool, OUT0_A>;
impl OUT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT0_A {
        match self.bits {
            false => OUT0_A::_0,
            true => OUT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT0_A::_1
    }
}
#[doc = "Write proxy for field `OUT0`"]
pub struct OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT0_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT0_A::_1)
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
#[doc = "Port Data Output Value 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT1_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT1_A> for bool {
    #[inline(always)]
    fn from(variant: OUT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT1`"]
pub type OUT1_R = crate::R<bool, OUT1_A>;
impl OUT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT1_A {
        match self.bits {
            false => OUT1_A::_0,
            true => OUT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT1_A::_1
    }
}
#[doc = "Write proxy for field `OUT1`"]
pub struct OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT1_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT1_A::_1)
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
#[doc = "Port Data Output Value 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT2_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT2_A> for bool {
    #[inline(always)]
    fn from(variant: OUT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT2`"]
pub type OUT2_R = crate::R<bool, OUT2_A>;
impl OUT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT2_A {
        match self.bits {
            false => OUT2_A::_0,
            true => OUT2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT2_A::_1
    }
}
#[doc = "Write proxy for field `OUT2`"]
pub struct OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT2_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT2_A::_1)
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
#[doc = "Port Data Output Value 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT3_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT3_A> for bool {
    #[inline(always)]
    fn from(variant: OUT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT3`"]
pub type OUT3_R = crate::R<bool, OUT3_A>;
impl OUT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT3_A {
        match self.bits {
            false => OUT3_A::_0,
            true => OUT3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT3_A::_1
    }
}
#[doc = "Write proxy for field `OUT3`"]
pub struct OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT3_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT3_A::_1)
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
#[doc = "Port Data Output Value 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT4_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT4_A> for bool {
    #[inline(always)]
    fn from(variant: OUT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT4`"]
pub type OUT4_R = crate::R<bool, OUT4_A>;
impl OUT4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT4_A {
        match self.bits {
            false => OUT4_A::_0,
            true => OUT4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT4_A::_1
    }
}
#[doc = "Write proxy for field `OUT4`"]
pub struct OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT4_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT4_A::_1)
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
#[doc = "Port Data Output Value 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT5_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT5_A> for bool {
    #[inline(always)]
    fn from(variant: OUT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT5`"]
pub type OUT5_R = crate::R<bool, OUT5_A>;
impl OUT5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT5_A {
        match self.bits {
            false => OUT5_A::_0,
            true => OUT5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT5_A::_1
    }
}
#[doc = "Write proxy for field `OUT5`"]
pub struct OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT5_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT5_A::_1)
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
#[doc = "Port Data Output Value 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT6_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT6_A> for bool {
    #[inline(always)]
    fn from(variant: OUT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT6`"]
pub type OUT6_R = crate::R<bool, OUT6_A>;
impl OUT6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT6_A {
        match self.bits {
            false => OUT6_A::_0,
            true => OUT6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT6_A::_1
    }
}
#[doc = "Write proxy for field `OUT6`"]
pub struct OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT6_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT6_A::_1)
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
#[doc = "Port Data Output Value 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT7_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT7_A> for bool {
    #[inline(always)]
    fn from(variant: OUT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT7`"]
pub type OUT7_R = crate::R<bool, OUT7_A>;
impl OUT7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT7_A {
        match self.bits {
            false => OUT7_A::_0,
            true => OUT7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT7_A::_1
    }
}
#[doc = "Write proxy for field `OUT7`"]
pub struct OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT7_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT7_A::_1)
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
#[doc = "Port Data Output Value 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT8_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT8_A> for bool {
    #[inline(always)]
    fn from(variant: OUT8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT8`"]
pub type OUT8_R = crate::R<bool, OUT8_A>;
impl OUT8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT8_A {
        match self.bits {
            false => OUT8_A::_0,
            true => OUT8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT8_A::_1
    }
}
#[doc = "Write proxy for field `OUT8`"]
pub struct OUT8_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT8_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT8_A::_1)
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
#[doc = "Port Data Output Value 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT9_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT9_A> for bool {
    #[inline(always)]
    fn from(variant: OUT9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT9`"]
pub type OUT9_R = crate::R<bool, OUT9_A>;
impl OUT9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT9_A {
        match self.bits {
            false => OUT9_A::_0,
            true => OUT9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT9_A::_1
    }
}
#[doc = "Write proxy for field `OUT9`"]
pub struct OUT9_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT9_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT9_A::_1)
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
#[doc = "Port Data Output Value 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT10_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT10_A> for bool {
    #[inline(always)]
    fn from(variant: OUT10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT10`"]
pub type OUT10_R = crate::R<bool, OUT10_A>;
impl OUT10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT10_A {
        match self.bits {
            false => OUT10_A::_0,
            true => OUT10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT10_A::_1
    }
}
#[doc = "Write proxy for field `OUT10`"]
pub struct OUT10_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT10_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT10_A::_1)
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
#[doc = "Port Data Output Value 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT11_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT11_A> for bool {
    #[inline(always)]
    fn from(variant: OUT11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT11`"]
pub type OUT11_R = crate::R<bool, OUT11_A>;
impl OUT11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT11_A {
        match self.bits {
            false => OUT11_A::_0,
            true => OUT11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT11_A::_1
    }
}
#[doc = "Write proxy for field `OUT11`"]
pub struct OUT11_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT11_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT11_A::_1)
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
#[doc = "Port Data Output Value 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT12_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT12_A> for bool {
    #[inline(always)]
    fn from(variant: OUT12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT12`"]
pub type OUT12_R = crate::R<bool, OUT12_A>;
impl OUT12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT12_A {
        match self.bits {
            false => OUT12_A::_0,
            true => OUT12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT12_A::_1
    }
}
#[doc = "Write proxy for field `OUT12`"]
pub struct OUT12_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT12_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT12_A::_1)
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
#[doc = "Port Data Output Value 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT13_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT13_A> for bool {
    #[inline(always)]
    fn from(variant: OUT13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT13`"]
pub type OUT13_R = crate::R<bool, OUT13_A>;
impl OUT13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT13_A {
        match self.bits {
            false => OUT13_A::_0,
            true => OUT13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT13_A::_1
    }
}
#[doc = "Write proxy for field `OUT13`"]
pub struct OUT13_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT13_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT13_A::_1)
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
#[doc = "Port Data Output Value 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT14_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT14_A> for bool {
    #[inline(always)]
    fn from(variant: OUT14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT14`"]
pub type OUT14_R = crate::R<bool, OUT14_A>;
impl OUT14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT14_A {
        match self.bits {
            false => OUT14_A::_0,
            true => OUT14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT14_A::_1
    }
}
#[doc = "Write proxy for field `OUT14`"]
pub struct OUT14_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT14_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT14_A::_1)
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
#[doc = "Port Data Output Value 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT15_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT15_A> for bool {
    #[inline(always)]
    fn from(variant: OUT15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT15`"]
pub type OUT15_R = crate::R<bool, OUT15_A>;
impl OUT15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT15_A {
        match self.bits {
            false => OUT15_A::_0,
            true => OUT15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT15_A::_1
    }
}
#[doc = "Write proxy for field `OUT15`"]
pub struct OUT15_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT15_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT15_A::_1)
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
#[doc = "Port Data Output Value 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT16_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT16_A> for bool {
    #[inline(always)]
    fn from(variant: OUT16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT16`"]
pub type OUT16_R = crate::R<bool, OUT16_A>;
impl OUT16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT16_A {
        match self.bits {
            false => OUT16_A::_0,
            true => OUT16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT16_A::_1
    }
}
#[doc = "Write proxy for field `OUT16`"]
pub struct OUT16_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT16_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT16_A::_1)
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
#[doc = "Port Data Output Value 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT17_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT17_A> for bool {
    #[inline(always)]
    fn from(variant: OUT17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT17`"]
pub type OUT17_R = crate::R<bool, OUT17_A>;
impl OUT17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT17_A {
        match self.bits {
            false => OUT17_A::_0,
            true => OUT17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT17_A::_1
    }
}
#[doc = "Write proxy for field `OUT17`"]
pub struct OUT17_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT17_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT17_A::_1)
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
#[doc = "Port Data Output Value 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT18_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT18_A> for bool {
    #[inline(always)]
    fn from(variant: OUT18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT18`"]
pub type OUT18_R = crate::R<bool, OUT18_A>;
impl OUT18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT18_A {
        match self.bits {
            false => OUT18_A::_0,
            true => OUT18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT18_A::_1
    }
}
#[doc = "Write proxy for field `OUT18`"]
pub struct OUT18_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT18_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT18_A::_1)
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
#[doc = "Port Data Output Value 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT19_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT19_A> for bool {
    #[inline(always)]
    fn from(variant: OUT19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT19`"]
pub type OUT19_R = crate::R<bool, OUT19_A>;
impl OUT19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT19_A {
        match self.bits {
            false => OUT19_A::_0,
            true => OUT19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT19_A::_1
    }
}
#[doc = "Write proxy for field `OUT19`"]
pub struct OUT19_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT19_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT19_A::_1)
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
#[doc = "Port Data Output Value 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT20_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT20_A> for bool {
    #[inline(always)]
    fn from(variant: OUT20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT20`"]
pub type OUT20_R = crate::R<bool, OUT20_A>;
impl OUT20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT20_A {
        match self.bits {
            false => OUT20_A::_0,
            true => OUT20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT20_A::_1
    }
}
#[doc = "Write proxy for field `OUT20`"]
pub struct OUT20_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT20_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT20_A::_1)
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
#[doc = "Port Data Output Value 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT21_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT21_A> for bool {
    #[inline(always)]
    fn from(variant: OUT21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT21`"]
pub type OUT21_R = crate::R<bool, OUT21_A>;
impl OUT21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT21_A {
        match self.bits {
            false => OUT21_A::_0,
            true => OUT21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT21_A::_1
    }
}
#[doc = "Write proxy for field `OUT21`"]
pub struct OUT21_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT21_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT21_A::_1)
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
#[doc = "Port Data Output Value 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT22_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT22_A> for bool {
    #[inline(always)]
    fn from(variant: OUT22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT22`"]
pub type OUT22_R = crate::R<bool, OUT22_A>;
impl OUT22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT22_A {
        match self.bits {
            false => OUT22_A::_0,
            true => OUT22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT22_A::_1
    }
}
#[doc = "Write proxy for field `OUT22`"]
pub struct OUT22_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT22_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT22_A::_1)
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
#[doc = "Port Data Output Value 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT23_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: OUT23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT23`"]
pub type OUT23_R = crate::R<bool, OUT23_A>;
impl OUT23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT23_A {
        match self.bits {
            false => OUT23_A::_0,
            true => OUT23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT23_A::_1
    }
}
#[doc = "Write proxy for field `OUT23`"]
pub struct OUT23_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT23_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT23_A::_1)
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
#[doc = "Port Data Output Value 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT24_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT24_A> for bool {
    #[inline(always)]
    fn from(variant: OUT24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT24`"]
pub type OUT24_R = crate::R<bool, OUT24_A>;
impl OUT24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT24_A {
        match self.bits {
            false => OUT24_A::_0,
            true => OUT24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT24_A::_1
    }
}
#[doc = "Write proxy for field `OUT24`"]
pub struct OUT24_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT24_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT24_A::_1)
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
#[doc = "Port Data Output Value 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT25_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT25_A> for bool {
    #[inline(always)]
    fn from(variant: OUT25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT25`"]
pub type OUT25_R = crate::R<bool, OUT25_A>;
impl OUT25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT25_A {
        match self.bits {
            false => OUT25_A::_0,
            true => OUT25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT25_A::_1
    }
}
#[doc = "Write proxy for field `OUT25`"]
pub struct OUT25_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT25_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT25_A::_1)
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
#[doc = "Port Data Output Value 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT26_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT26_A> for bool {
    #[inline(always)]
    fn from(variant: OUT26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT26`"]
pub type OUT26_R = crate::R<bool, OUT26_A>;
impl OUT26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT26_A {
        match self.bits {
            false => OUT26_A::_0,
            true => OUT26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT26_A::_1
    }
}
#[doc = "Write proxy for field `OUT26`"]
pub struct OUT26_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT26_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT26_A::_1)
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
#[doc = "Port Data Output Value 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT27_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT27_A> for bool {
    #[inline(always)]
    fn from(variant: OUT27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT27`"]
pub type OUT27_R = crate::R<bool, OUT27_A>;
impl OUT27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT27_A {
        match self.bits {
            false => OUT27_A::_0,
            true => OUT27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT27_A::_1
    }
}
#[doc = "Write proxy for field `OUT27`"]
pub struct OUT27_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT27_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT27_A::_1)
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
#[doc = "Port Data Output Value 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT28_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT28_A> for bool {
    #[inline(always)]
    fn from(variant: OUT28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT28`"]
pub type OUT28_R = crate::R<bool, OUT28_A>;
impl OUT28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT28_A {
        match self.bits {
            false => OUT28_A::_0,
            true => OUT28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT28_A::_1
    }
}
#[doc = "Write proxy for field `OUT28`"]
pub struct OUT28_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT28_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT28_A::_1)
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
#[doc = "Port Data Output Value 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT29_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT29_A> for bool {
    #[inline(always)]
    fn from(variant: OUT29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT29`"]
pub type OUT29_R = crate::R<bool, OUT29_A>;
impl OUT29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT29_A {
        match self.bits {
            false => OUT29_A::_0,
            true => OUT29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT29_A::_1
    }
}
#[doc = "Write proxy for field `OUT29`"]
pub struct OUT29_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT29_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT29_A::_1)
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
#[doc = "Port Data Output Value 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT30_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT30_A> for bool {
    #[inline(always)]
    fn from(variant: OUT30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT30`"]
pub type OUT30_R = crate::R<bool, OUT30_A>;
impl OUT30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT30_A {
        match self.bits {
            false => OUT30_A::_0,
            true => OUT30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT30_A::_1
    }
}
#[doc = "Write proxy for field `OUT30`"]
pub struct OUT30_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT30_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT30_A::_1)
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
#[doc = "Port Data Output Value 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT31_A {
    #[doc = "0: Output 0"]
    _0 = 0,
    #[doc = "1: Output 1"]
    _1 = 1,
}
impl From<OUT31_A> for bool {
    #[inline(always)]
    fn from(variant: OUT31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT31`"]
pub type OUT31_R = crate::R<bool, OUT31_A>;
impl OUT31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT31_A {
        match self.bits {
            false => OUT31_A::_0,
            true => OUT31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUT31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUT31_A::_1
    }
}
#[doc = "Write proxy for field `OUT31`"]
pub struct OUT31_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUT31_A::_0)
    }
    #[doc = "Output 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUT31_A::_1)
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
    #[doc = "Bit 0 - Port Data Output Value 0"]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Output Value 1"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Output Value 2"]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Output Value 3"]
    #[inline(always)]
    pub fn out3(&self) -> OUT3_R {
        OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Output Value 4"]
    #[inline(always)]
    pub fn out4(&self) -> OUT4_R {
        OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Output Value 5"]
    #[inline(always)]
    pub fn out5(&self) -> OUT5_R {
        OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Output Value 6"]
    #[inline(always)]
    pub fn out6(&self) -> OUT6_R {
        OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Output Value 7"]
    #[inline(always)]
    pub fn out7(&self) -> OUT7_R {
        OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Output Value 8"]
    #[inline(always)]
    pub fn out8(&self) -> OUT8_R {
        OUT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Output Value 9"]
    #[inline(always)]
    pub fn out9(&self) -> OUT9_R {
        OUT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Output Value 10"]
    #[inline(always)]
    pub fn out10(&self) -> OUT10_R {
        OUT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Output Value 11"]
    #[inline(always)]
    pub fn out11(&self) -> OUT11_R {
        OUT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Output Value 12"]
    #[inline(always)]
    pub fn out12(&self) -> OUT12_R {
        OUT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Output Value 13"]
    #[inline(always)]
    pub fn out13(&self) -> OUT13_R {
        OUT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Output Value 14"]
    #[inline(always)]
    pub fn out14(&self) -> OUT14_R {
        OUT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Output Value 15"]
    #[inline(always)]
    pub fn out15(&self) -> OUT15_R {
        OUT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Output Value 16"]
    #[inline(always)]
    pub fn out16(&self) -> OUT16_R {
        OUT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Output Value 17"]
    #[inline(always)]
    pub fn out17(&self) -> OUT17_R {
        OUT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Output Value 18"]
    #[inline(always)]
    pub fn out18(&self) -> OUT18_R {
        OUT18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Output Value 19"]
    #[inline(always)]
    pub fn out19(&self) -> OUT19_R {
        OUT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Output Value 20"]
    #[inline(always)]
    pub fn out20(&self) -> OUT20_R {
        OUT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Output Value 21"]
    #[inline(always)]
    pub fn out21(&self) -> OUT21_R {
        OUT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Output Value 22"]
    #[inline(always)]
    pub fn out22(&self) -> OUT22_R {
        OUT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Output Value 23"]
    #[inline(always)]
    pub fn out23(&self) -> OUT23_R {
        OUT23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Output Value 24"]
    #[inline(always)]
    pub fn out24(&self) -> OUT24_R {
        OUT24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Output Value 25"]
    #[inline(always)]
    pub fn out25(&self) -> OUT25_R {
        OUT25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Output Value 26"]
    #[inline(always)]
    pub fn out26(&self) -> OUT26_R {
        OUT26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Output Value 27"]
    #[inline(always)]
    pub fn out27(&self) -> OUT27_R {
        OUT27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Output Value 28"]
    #[inline(always)]
    pub fn out28(&self) -> OUT28_R {
        OUT28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Output Value 29"]
    #[inline(always)]
    pub fn out29(&self) -> OUT29_R {
        OUT29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Output Value 30"]
    #[inline(always)]
    pub fn out30(&self) -> OUT30_R {
        OUT30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Output Value 31"]
    #[inline(always)]
    pub fn out31(&self) -> OUT31_R {
        OUT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Output Value 0"]
    #[inline(always)]
    pub fn out0(&mut self) -> OUT0_W {
        OUT0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Output Value 1"]
    #[inline(always)]
    pub fn out1(&mut self) -> OUT1_W {
        OUT1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Output Value 2"]
    #[inline(always)]
    pub fn out2(&mut self) -> OUT2_W {
        OUT2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Output Value 3"]
    #[inline(always)]
    pub fn out3(&mut self) -> OUT3_W {
        OUT3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Output Value 4"]
    #[inline(always)]
    pub fn out4(&mut self) -> OUT4_W {
        OUT4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Output Value 5"]
    #[inline(always)]
    pub fn out5(&mut self) -> OUT5_W {
        OUT5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Output Value 6"]
    #[inline(always)]
    pub fn out6(&mut self) -> OUT6_W {
        OUT6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Output Value 7"]
    #[inline(always)]
    pub fn out7(&mut self) -> OUT7_W {
        OUT7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Output Value 8"]
    #[inline(always)]
    pub fn out8(&mut self) -> OUT8_W {
        OUT8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Output Value 9"]
    #[inline(always)]
    pub fn out9(&mut self) -> OUT9_W {
        OUT9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Output Value 10"]
    #[inline(always)]
    pub fn out10(&mut self) -> OUT10_W {
        OUT10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Output Value 11"]
    #[inline(always)]
    pub fn out11(&mut self) -> OUT11_W {
        OUT11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Output Value 12"]
    #[inline(always)]
    pub fn out12(&mut self) -> OUT12_W {
        OUT12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Output Value 13"]
    #[inline(always)]
    pub fn out13(&mut self) -> OUT13_W {
        OUT13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Output Value 14"]
    #[inline(always)]
    pub fn out14(&mut self) -> OUT14_W {
        OUT14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Output Value 15"]
    #[inline(always)]
    pub fn out15(&mut self) -> OUT15_W {
        OUT15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Output Value 16"]
    #[inline(always)]
    pub fn out16(&mut self) -> OUT16_W {
        OUT16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Output Value 17"]
    #[inline(always)]
    pub fn out17(&mut self) -> OUT17_W {
        OUT17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Output Value 18"]
    #[inline(always)]
    pub fn out18(&mut self) -> OUT18_W {
        OUT18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Output Value 19"]
    #[inline(always)]
    pub fn out19(&mut self) -> OUT19_W {
        OUT19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Output Value 20"]
    #[inline(always)]
    pub fn out20(&mut self) -> OUT20_W {
        OUT20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Output Value 21"]
    #[inline(always)]
    pub fn out21(&mut self) -> OUT21_W {
        OUT21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Output Value 22"]
    #[inline(always)]
    pub fn out22(&mut self) -> OUT22_W {
        OUT22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Output Value 23"]
    #[inline(always)]
    pub fn out23(&mut self) -> OUT23_W {
        OUT23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Output Value 24"]
    #[inline(always)]
    pub fn out24(&mut self) -> OUT24_W {
        OUT24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Output Value 25"]
    #[inline(always)]
    pub fn out25(&mut self) -> OUT25_W {
        OUT25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Output Value 26"]
    #[inline(always)]
    pub fn out26(&mut self) -> OUT26_W {
        OUT26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Output Value 27"]
    #[inline(always)]
    pub fn out27(&mut self) -> OUT27_W {
        OUT27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Output Value 28"]
    #[inline(always)]
    pub fn out28(&mut self) -> OUT28_W {
        OUT28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Output Value 29"]
    #[inline(always)]
    pub fn out29(&mut self) -> OUT29_W {
        OUT29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Output Value 30"]
    #[inline(always)]
    pub fn out30(&mut self) -> OUT30_W {
        OUT30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Output Value 31"]
    #[inline(always)]
    pub fn out31(&mut self) -> OUT31_W {
        OUT31_W { w: self }
    }
}
