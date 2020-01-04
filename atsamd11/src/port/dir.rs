#[doc = "Reader of register DIR%s"]
pub type R = crate::R<u32, super::DIR>;
#[doc = "Writer for register DIR%s"]
pub type W = crate::W<u32, super::DIR>;
#[doc = "Register DIR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Direction 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR0_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR0_A> for bool {
    #[inline(always)]
    fn from(variant: DIR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR0`"]
pub type DIR0_R = crate::R<bool, DIR0_A>;
impl DIR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR0_A {
        match self.bits {
            false => DIR0_A::INPUT,
            true => DIR0_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR0_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR0`"]
pub struct DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR0_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR0_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR1_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR1_A> for bool {
    #[inline(always)]
    fn from(variant: DIR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR1`"]
pub type DIR1_R = crate::R<bool, DIR1_A>;
impl DIR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR1_A {
        match self.bits {
            false => DIR1_A::INPUT,
            true => DIR1_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR1_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR1_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR1`"]
pub struct DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR1_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR1_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR2_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR2_A> for bool {
    #[inline(always)]
    fn from(variant: DIR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR2`"]
pub type DIR2_R = crate::R<bool, DIR2_A>;
impl DIR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR2_A {
        match self.bits {
            false => DIR2_A::INPUT,
            true => DIR2_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR2_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR2_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR2`"]
pub struct DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR2_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR2_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR3_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR3_A> for bool {
    #[inline(always)]
    fn from(variant: DIR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR3`"]
pub type DIR3_R = crate::R<bool, DIR3_A>;
impl DIR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR3_A {
        match self.bits {
            false => DIR3_A::INPUT,
            true => DIR3_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR3_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR3_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR3`"]
pub struct DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR3_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR3_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR4_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR4_A> for bool {
    #[inline(always)]
    fn from(variant: DIR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR4`"]
pub type DIR4_R = crate::R<bool, DIR4_A>;
impl DIR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR4_A {
        match self.bits {
            false => DIR4_A::INPUT,
            true => DIR4_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR4_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR4_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR4`"]
pub struct DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR4_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR4_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR5_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR5_A> for bool {
    #[inline(always)]
    fn from(variant: DIR5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR5`"]
pub type DIR5_R = crate::R<bool, DIR5_A>;
impl DIR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR5_A {
        match self.bits {
            false => DIR5_A::INPUT,
            true => DIR5_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR5_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR5_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR5`"]
pub struct DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR5_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR5_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR6_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR6_A> for bool {
    #[inline(always)]
    fn from(variant: DIR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR6`"]
pub type DIR6_R = crate::R<bool, DIR6_A>;
impl DIR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR6_A {
        match self.bits {
            false => DIR6_A::INPUT,
            true => DIR6_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR6_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR6_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR6`"]
pub struct DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR6_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR6_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR7_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR7_A> for bool {
    #[inline(always)]
    fn from(variant: DIR7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR7`"]
pub type DIR7_R = crate::R<bool, DIR7_A>;
impl DIR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR7_A {
        match self.bits {
            false => DIR7_A::INPUT,
            true => DIR7_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR7_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR7_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR7`"]
pub struct DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR7_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR7_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR8_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR8_A> for bool {
    #[inline(always)]
    fn from(variant: DIR8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR8`"]
pub type DIR8_R = crate::R<bool, DIR8_A>;
impl DIR8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR8_A {
        match self.bits {
            false => DIR8_A::INPUT,
            true => DIR8_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR8_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR8_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR8`"]
pub struct DIR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR8_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR8_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR9_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR9_A> for bool {
    #[inline(always)]
    fn from(variant: DIR9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR9`"]
pub type DIR9_R = crate::R<bool, DIR9_A>;
impl DIR9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR9_A {
        match self.bits {
            false => DIR9_A::INPUT,
            true => DIR9_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR9_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR9_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR9`"]
pub struct DIR9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR9_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR9_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR10_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR10_A> for bool {
    #[inline(always)]
    fn from(variant: DIR10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR10`"]
pub type DIR10_R = crate::R<bool, DIR10_A>;
impl DIR10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR10_A {
        match self.bits {
            false => DIR10_A::INPUT,
            true => DIR10_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR10_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR10_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR10`"]
pub struct DIR10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR10_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR10_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR11_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR11_A> for bool {
    #[inline(always)]
    fn from(variant: DIR11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR11`"]
pub type DIR11_R = crate::R<bool, DIR11_A>;
impl DIR11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR11_A {
        match self.bits {
            false => DIR11_A::INPUT,
            true => DIR11_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR11_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR11_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR11`"]
pub struct DIR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR11_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR11_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR12_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR12_A> for bool {
    #[inline(always)]
    fn from(variant: DIR12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR12`"]
pub type DIR12_R = crate::R<bool, DIR12_A>;
impl DIR12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR12_A {
        match self.bits {
            false => DIR12_A::INPUT,
            true => DIR12_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR12_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR12_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR12`"]
pub struct DIR12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR12_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR12_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR13_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR13_A> for bool {
    #[inline(always)]
    fn from(variant: DIR13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR13`"]
pub type DIR13_R = crate::R<bool, DIR13_A>;
impl DIR13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR13_A {
        match self.bits {
            false => DIR13_A::INPUT,
            true => DIR13_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR13_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR13_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR13`"]
pub struct DIR13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR13_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR13_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR14_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR14_A> for bool {
    #[inline(always)]
    fn from(variant: DIR14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR14`"]
pub type DIR14_R = crate::R<bool, DIR14_A>;
impl DIR14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR14_A {
        match self.bits {
            false => DIR14_A::INPUT,
            true => DIR14_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR14_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR14_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR14`"]
pub struct DIR14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR14_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR14_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR15_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR15_A> for bool {
    #[inline(always)]
    fn from(variant: DIR15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR15`"]
pub type DIR15_R = crate::R<bool, DIR15_A>;
impl DIR15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR15_A {
        match self.bits {
            false => DIR15_A::INPUT,
            true => DIR15_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR15_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR15_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR15`"]
pub struct DIR15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR15_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR15_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR16_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR16_A> for bool {
    #[inline(always)]
    fn from(variant: DIR16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR16`"]
pub type DIR16_R = crate::R<bool, DIR16_A>;
impl DIR16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR16_A {
        match self.bits {
            false => DIR16_A::INPUT,
            true => DIR16_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR16_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR16_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR16`"]
pub struct DIR16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR16_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR16_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR17_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR17_A> for bool {
    #[inline(always)]
    fn from(variant: DIR17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR17`"]
pub type DIR17_R = crate::R<bool, DIR17_A>;
impl DIR17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR17_A {
        match self.bits {
            false => DIR17_A::INPUT,
            true => DIR17_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR17_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR17_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR17`"]
pub struct DIR17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR17_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR17_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR18_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR18_A> for bool {
    #[inline(always)]
    fn from(variant: DIR18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR18`"]
pub type DIR18_R = crate::R<bool, DIR18_A>;
impl DIR18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR18_A {
        match self.bits {
            false => DIR18_A::INPUT,
            true => DIR18_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR18_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR18_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR18`"]
pub struct DIR18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR18_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR18_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR19_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR19_A> for bool {
    #[inline(always)]
    fn from(variant: DIR19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR19`"]
pub type DIR19_R = crate::R<bool, DIR19_A>;
impl DIR19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR19_A {
        match self.bits {
            false => DIR19_A::INPUT,
            true => DIR19_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR19_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR19_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR19`"]
pub struct DIR19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR19_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR19_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR20_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR20_A> for bool {
    #[inline(always)]
    fn from(variant: DIR20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR20`"]
pub type DIR20_R = crate::R<bool, DIR20_A>;
impl DIR20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR20_A {
        match self.bits {
            false => DIR20_A::INPUT,
            true => DIR20_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR20_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR20_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR20`"]
pub struct DIR20_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR20_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR20_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR21_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR21_A> for bool {
    #[inline(always)]
    fn from(variant: DIR21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR21`"]
pub type DIR21_R = crate::R<bool, DIR21_A>;
impl DIR21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR21_A {
        match self.bits {
            false => DIR21_A::INPUT,
            true => DIR21_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR21_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR21_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR21`"]
pub struct DIR21_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR21_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR21_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR22_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR22_A> for bool {
    #[inline(always)]
    fn from(variant: DIR22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR22`"]
pub type DIR22_R = crate::R<bool, DIR22_A>;
impl DIR22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR22_A {
        match self.bits {
            false => DIR22_A::INPUT,
            true => DIR22_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR22_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR22_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR22`"]
pub struct DIR22_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR22_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR22_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR23_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR23_A> for bool {
    #[inline(always)]
    fn from(variant: DIR23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR23`"]
pub type DIR23_R = crate::R<bool, DIR23_A>;
impl DIR23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR23_A {
        match self.bits {
            false => DIR23_A::INPUT,
            true => DIR23_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR23_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR23_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR23`"]
pub struct DIR23_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR23_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR23_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR24_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR24_A> for bool {
    #[inline(always)]
    fn from(variant: DIR24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR24`"]
pub type DIR24_R = crate::R<bool, DIR24_A>;
impl DIR24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR24_A {
        match self.bits {
            false => DIR24_A::INPUT,
            true => DIR24_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR24_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR24_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR24`"]
pub struct DIR24_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR24_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR24_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR25_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR25_A> for bool {
    #[inline(always)]
    fn from(variant: DIR25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR25`"]
pub type DIR25_R = crate::R<bool, DIR25_A>;
impl DIR25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR25_A {
        match self.bits {
            false => DIR25_A::INPUT,
            true => DIR25_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR25_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR25_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR25`"]
pub struct DIR25_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR25_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR25_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR26_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR26_A> for bool {
    #[inline(always)]
    fn from(variant: DIR26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR26`"]
pub type DIR26_R = crate::R<bool, DIR26_A>;
impl DIR26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR26_A {
        match self.bits {
            false => DIR26_A::INPUT,
            true => DIR26_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR26_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR26_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR26`"]
pub struct DIR26_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR26_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR26_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR27_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR27_A> for bool {
    #[inline(always)]
    fn from(variant: DIR27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR27`"]
pub type DIR27_R = crate::R<bool, DIR27_A>;
impl DIR27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR27_A {
        match self.bits {
            false => DIR27_A::INPUT,
            true => DIR27_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR27_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR27_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR27`"]
pub struct DIR27_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR27_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR27_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR28_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR28_A> for bool {
    #[inline(always)]
    fn from(variant: DIR28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR28`"]
pub type DIR28_R = crate::R<bool, DIR28_A>;
impl DIR28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR28_A {
        match self.bits {
            false => DIR28_A::INPUT,
            true => DIR28_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR28_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR28_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR28`"]
pub struct DIR28_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR28_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR28_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR29_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR29_A> for bool {
    #[inline(always)]
    fn from(variant: DIR29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR29`"]
pub type DIR29_R = crate::R<bool, DIR29_A>;
impl DIR29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR29_A {
        match self.bits {
            false => DIR29_A::INPUT,
            true => DIR29_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR29_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR29_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR29`"]
pub struct DIR29_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR29_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR29_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR30_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR30_A> for bool {
    #[inline(always)]
    fn from(variant: DIR30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR30`"]
pub type DIR30_R = crate::R<bool, DIR30_A>;
impl DIR30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR30_A {
        match self.bits {
            false => DIR30_A::INPUT,
            true => DIR30_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR30_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR30_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR30`"]
pub struct DIR30_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR30_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR30_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Port Data Direction 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR31_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<DIR31_A> for bool {
    #[inline(always)]
    fn from(variant: DIR31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR31`"]
pub type DIR31_R = crate::R<bool, DIR31_A>;
impl DIR31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR31_A {
        match self.bits {
            false => DIR31_A::INPUT,
            true => DIR31_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR31_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR31_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR31`"]
pub struct DIR31_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR31_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR31_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Port Data Direction 0"]
    #[inline(always)]
    pub fn dir0(&self) -> DIR0_R {
        DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Direction 1"]
    #[inline(always)]
    pub fn dir1(&self) -> DIR1_R {
        DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Direction 2"]
    #[inline(always)]
    pub fn dir2(&self) -> DIR2_R {
        DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Direction 3"]
    #[inline(always)]
    pub fn dir3(&self) -> DIR3_R {
        DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Direction 4"]
    #[inline(always)]
    pub fn dir4(&self) -> DIR4_R {
        DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Direction 5"]
    #[inline(always)]
    pub fn dir5(&self) -> DIR5_R {
        DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Direction 6"]
    #[inline(always)]
    pub fn dir6(&self) -> DIR6_R {
        DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Direction 7"]
    #[inline(always)]
    pub fn dir7(&self) -> DIR7_R {
        DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Direction 8"]
    #[inline(always)]
    pub fn dir8(&self) -> DIR8_R {
        DIR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Direction 9"]
    #[inline(always)]
    pub fn dir9(&self) -> DIR9_R {
        DIR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Direction 10"]
    #[inline(always)]
    pub fn dir10(&self) -> DIR10_R {
        DIR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Direction 11"]
    #[inline(always)]
    pub fn dir11(&self) -> DIR11_R {
        DIR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Direction 12"]
    #[inline(always)]
    pub fn dir12(&self) -> DIR12_R {
        DIR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Direction 13"]
    #[inline(always)]
    pub fn dir13(&self) -> DIR13_R {
        DIR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Direction 14"]
    #[inline(always)]
    pub fn dir14(&self) -> DIR14_R {
        DIR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Direction 15"]
    #[inline(always)]
    pub fn dir15(&self) -> DIR15_R {
        DIR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Direction 16"]
    #[inline(always)]
    pub fn dir16(&self) -> DIR16_R {
        DIR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Direction 17"]
    #[inline(always)]
    pub fn dir17(&self) -> DIR17_R {
        DIR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Direction 18"]
    #[inline(always)]
    pub fn dir18(&self) -> DIR18_R {
        DIR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Direction 19"]
    #[inline(always)]
    pub fn dir19(&self) -> DIR19_R {
        DIR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Direction 20"]
    #[inline(always)]
    pub fn dir20(&self) -> DIR20_R {
        DIR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Direction 21"]
    #[inline(always)]
    pub fn dir21(&self) -> DIR21_R {
        DIR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Direction 22"]
    #[inline(always)]
    pub fn dir22(&self) -> DIR22_R {
        DIR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Direction 23"]
    #[inline(always)]
    pub fn dir23(&self) -> DIR23_R {
        DIR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Direction 24"]
    #[inline(always)]
    pub fn dir24(&self) -> DIR24_R {
        DIR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Direction 25"]
    #[inline(always)]
    pub fn dir25(&self) -> DIR25_R {
        DIR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Direction 26"]
    #[inline(always)]
    pub fn dir26(&self) -> DIR26_R {
        DIR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Direction 27"]
    #[inline(always)]
    pub fn dir27(&self) -> DIR27_R {
        DIR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Direction 28"]
    #[inline(always)]
    pub fn dir28(&self) -> DIR28_R {
        DIR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Direction 29"]
    #[inline(always)]
    pub fn dir29(&self) -> DIR29_R {
        DIR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Direction 30"]
    #[inline(always)]
    pub fn dir30(&self) -> DIR30_R {
        DIR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Direction 31"]
    #[inline(always)]
    pub fn dir31(&self) -> DIR31_R {
        DIR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Direction 0"]
    #[inline(always)]
    pub fn dir0(&mut self) -> DIR0_W {
        DIR0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Direction 1"]
    #[inline(always)]
    pub fn dir1(&mut self) -> DIR1_W {
        DIR1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Direction 2"]
    #[inline(always)]
    pub fn dir2(&mut self) -> DIR2_W {
        DIR2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Direction 3"]
    #[inline(always)]
    pub fn dir3(&mut self) -> DIR3_W {
        DIR3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Direction 4"]
    #[inline(always)]
    pub fn dir4(&mut self) -> DIR4_W {
        DIR4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Direction 5"]
    #[inline(always)]
    pub fn dir5(&mut self) -> DIR5_W {
        DIR5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Direction 6"]
    #[inline(always)]
    pub fn dir6(&mut self) -> DIR6_W {
        DIR6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Direction 7"]
    #[inline(always)]
    pub fn dir7(&mut self) -> DIR7_W {
        DIR7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Direction 8"]
    #[inline(always)]
    pub fn dir8(&mut self) -> DIR8_W {
        DIR8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Direction 9"]
    #[inline(always)]
    pub fn dir9(&mut self) -> DIR9_W {
        DIR9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Direction 10"]
    #[inline(always)]
    pub fn dir10(&mut self) -> DIR10_W {
        DIR10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Direction 11"]
    #[inline(always)]
    pub fn dir11(&mut self) -> DIR11_W {
        DIR11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Direction 12"]
    #[inline(always)]
    pub fn dir12(&mut self) -> DIR12_W {
        DIR12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Direction 13"]
    #[inline(always)]
    pub fn dir13(&mut self) -> DIR13_W {
        DIR13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Direction 14"]
    #[inline(always)]
    pub fn dir14(&mut self) -> DIR14_W {
        DIR14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Direction 15"]
    #[inline(always)]
    pub fn dir15(&mut self) -> DIR15_W {
        DIR15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Direction 16"]
    #[inline(always)]
    pub fn dir16(&mut self) -> DIR16_W {
        DIR16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Direction 17"]
    #[inline(always)]
    pub fn dir17(&mut self) -> DIR17_W {
        DIR17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Direction 18"]
    #[inline(always)]
    pub fn dir18(&mut self) -> DIR18_W {
        DIR18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Direction 19"]
    #[inline(always)]
    pub fn dir19(&mut self) -> DIR19_W {
        DIR19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Direction 20"]
    #[inline(always)]
    pub fn dir20(&mut self) -> DIR20_W {
        DIR20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Direction 21"]
    #[inline(always)]
    pub fn dir21(&mut self) -> DIR21_W {
        DIR21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Direction 22"]
    #[inline(always)]
    pub fn dir22(&mut self) -> DIR22_W {
        DIR22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Direction 23"]
    #[inline(always)]
    pub fn dir23(&mut self) -> DIR23_W {
        DIR23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Direction 24"]
    #[inline(always)]
    pub fn dir24(&mut self) -> DIR24_W {
        DIR24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Direction 25"]
    #[inline(always)]
    pub fn dir25(&mut self) -> DIR25_W {
        DIR25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Direction 26"]
    #[inline(always)]
    pub fn dir26(&mut self) -> DIR26_W {
        DIR26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Direction 27"]
    #[inline(always)]
    pub fn dir27(&mut self) -> DIR27_W {
        DIR27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Direction 28"]
    #[inline(always)]
    pub fn dir28(&mut self) -> DIR28_W {
        DIR28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Direction 29"]
    #[inline(always)]
    pub fn dir29(&mut self) -> DIR29_W {
        DIR29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Direction 30"]
    #[inline(always)]
    pub fn dir30(&mut self) -> DIR30_W {
        DIR30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Direction 31"]
    #[inline(always)]
    pub fn dir31(&mut self) -> DIR31_W {
        DIR31_W { w: self }
    }
}
