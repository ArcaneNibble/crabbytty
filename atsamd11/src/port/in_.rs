#[doc = "Reader of register IN%s"]
pub type R = crate::R<u32, super::IN>;
#[doc = "Port Data Input Value 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN0_A> for bool {
    #[inline(always)]
    fn from(variant: IN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN0`"]
pub type IN0_R = crate::R<bool, IN0_A>;
impl IN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN0_A {
        match self.bits {
            false => IN0_A::_0,
            true => IN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN0_A::_1
    }
}
#[doc = "Port Data Input Value 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN1_A> for bool {
    #[inline(always)]
    fn from(variant: IN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN1`"]
pub type IN1_R = crate::R<bool, IN1_A>;
impl IN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN1_A {
        match self.bits {
            false => IN1_A::_0,
            true => IN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN1_A::_1
    }
}
#[doc = "Port Data Input Value 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN2_A> for bool {
    #[inline(always)]
    fn from(variant: IN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN2`"]
pub type IN2_R = crate::R<bool, IN2_A>;
impl IN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN2_A {
        match self.bits {
            false => IN2_A::_0,
            true => IN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN2_A::_1
    }
}
#[doc = "Port Data Input Value 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN3_A> for bool {
    #[inline(always)]
    fn from(variant: IN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN3`"]
pub type IN3_R = crate::R<bool, IN3_A>;
impl IN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN3_A {
        match self.bits {
            false => IN3_A::_0,
            true => IN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN3_A::_1
    }
}
#[doc = "Port Data Input Value 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN4_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN4_A> for bool {
    #[inline(always)]
    fn from(variant: IN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN4`"]
pub type IN4_R = crate::R<bool, IN4_A>;
impl IN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN4_A {
        match self.bits {
            false => IN4_A::_0,
            true => IN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN4_A::_1
    }
}
#[doc = "Port Data Input Value 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN5_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN5_A> for bool {
    #[inline(always)]
    fn from(variant: IN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN5`"]
pub type IN5_R = crate::R<bool, IN5_A>;
impl IN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN5_A {
        match self.bits {
            false => IN5_A::_0,
            true => IN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN5_A::_1
    }
}
#[doc = "Port Data Input Value 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN6_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN6_A> for bool {
    #[inline(always)]
    fn from(variant: IN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN6`"]
pub type IN6_R = crate::R<bool, IN6_A>;
impl IN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN6_A {
        match self.bits {
            false => IN6_A::_0,
            true => IN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN6_A::_1
    }
}
#[doc = "Port Data Input Value 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN7_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN7_A> for bool {
    #[inline(always)]
    fn from(variant: IN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN7`"]
pub type IN7_R = crate::R<bool, IN7_A>;
impl IN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN7_A {
        match self.bits {
            false => IN7_A::_0,
            true => IN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN7_A::_1
    }
}
#[doc = "Port Data Input Value 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN8_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN8_A> for bool {
    #[inline(always)]
    fn from(variant: IN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN8`"]
pub type IN8_R = crate::R<bool, IN8_A>;
impl IN8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN8_A {
        match self.bits {
            false => IN8_A::_0,
            true => IN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN8_A::_1
    }
}
#[doc = "Port Data Input Value 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN9_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN9_A> for bool {
    #[inline(always)]
    fn from(variant: IN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN9`"]
pub type IN9_R = crate::R<bool, IN9_A>;
impl IN9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN9_A {
        match self.bits {
            false => IN9_A::_0,
            true => IN9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN9_A::_1
    }
}
#[doc = "Port Data Input Value 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN10_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN10_A> for bool {
    #[inline(always)]
    fn from(variant: IN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN10`"]
pub type IN10_R = crate::R<bool, IN10_A>;
impl IN10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN10_A {
        match self.bits {
            false => IN10_A::_0,
            true => IN10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN10_A::_1
    }
}
#[doc = "Port Data Input Value 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN11_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN11_A> for bool {
    #[inline(always)]
    fn from(variant: IN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN11`"]
pub type IN11_R = crate::R<bool, IN11_A>;
impl IN11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN11_A {
        match self.bits {
            false => IN11_A::_0,
            true => IN11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN11_A::_1
    }
}
#[doc = "Port Data Input Value 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN12_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN12_A> for bool {
    #[inline(always)]
    fn from(variant: IN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN12`"]
pub type IN12_R = crate::R<bool, IN12_A>;
impl IN12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN12_A {
        match self.bits {
            false => IN12_A::_0,
            true => IN12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN12_A::_1
    }
}
#[doc = "Port Data Input Value 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN13_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN13_A> for bool {
    #[inline(always)]
    fn from(variant: IN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN13`"]
pub type IN13_R = crate::R<bool, IN13_A>;
impl IN13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN13_A {
        match self.bits {
            false => IN13_A::_0,
            true => IN13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN13_A::_1
    }
}
#[doc = "Port Data Input Value 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN14_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN14_A> for bool {
    #[inline(always)]
    fn from(variant: IN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN14`"]
pub type IN14_R = crate::R<bool, IN14_A>;
impl IN14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN14_A {
        match self.bits {
            false => IN14_A::_0,
            true => IN14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN14_A::_1
    }
}
#[doc = "Port Data Input Value 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN15_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN15_A> for bool {
    #[inline(always)]
    fn from(variant: IN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN15`"]
pub type IN15_R = crate::R<bool, IN15_A>;
impl IN15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN15_A {
        match self.bits {
            false => IN15_A::_0,
            true => IN15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN15_A::_1
    }
}
#[doc = "Port Data Input Value 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN16_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN16_A> for bool {
    #[inline(always)]
    fn from(variant: IN16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN16`"]
pub type IN16_R = crate::R<bool, IN16_A>;
impl IN16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN16_A {
        match self.bits {
            false => IN16_A::_0,
            true => IN16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN16_A::_1
    }
}
#[doc = "Port Data Input Value 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN17_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN17_A> for bool {
    #[inline(always)]
    fn from(variant: IN17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN17`"]
pub type IN17_R = crate::R<bool, IN17_A>;
impl IN17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN17_A {
        match self.bits {
            false => IN17_A::_0,
            true => IN17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN17_A::_1
    }
}
#[doc = "Port Data Input Value 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN18_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN18_A> for bool {
    #[inline(always)]
    fn from(variant: IN18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN18`"]
pub type IN18_R = crate::R<bool, IN18_A>;
impl IN18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN18_A {
        match self.bits {
            false => IN18_A::_0,
            true => IN18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN18_A::_1
    }
}
#[doc = "Port Data Input Value 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN19_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN19_A> for bool {
    #[inline(always)]
    fn from(variant: IN19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN19`"]
pub type IN19_R = crate::R<bool, IN19_A>;
impl IN19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN19_A {
        match self.bits {
            false => IN19_A::_0,
            true => IN19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN19_A::_1
    }
}
#[doc = "Port Data Input Value 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN20_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN20_A> for bool {
    #[inline(always)]
    fn from(variant: IN20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN20`"]
pub type IN20_R = crate::R<bool, IN20_A>;
impl IN20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN20_A {
        match self.bits {
            false => IN20_A::_0,
            true => IN20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN20_A::_1
    }
}
#[doc = "Port Data Input Value 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN21_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN21_A> for bool {
    #[inline(always)]
    fn from(variant: IN21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN21`"]
pub type IN21_R = crate::R<bool, IN21_A>;
impl IN21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN21_A {
        match self.bits {
            false => IN21_A::_0,
            true => IN21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN21_A::_1
    }
}
#[doc = "Port Data Input Value 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN22_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN22_A> for bool {
    #[inline(always)]
    fn from(variant: IN22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN22`"]
pub type IN22_R = crate::R<bool, IN22_A>;
impl IN22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN22_A {
        match self.bits {
            false => IN22_A::_0,
            true => IN22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN22_A::_1
    }
}
#[doc = "Port Data Input Value 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN23_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN23_A> for bool {
    #[inline(always)]
    fn from(variant: IN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN23`"]
pub type IN23_R = crate::R<bool, IN23_A>;
impl IN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN23_A {
        match self.bits {
            false => IN23_A::_0,
            true => IN23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN23_A::_1
    }
}
#[doc = "Port Data Input Value 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN24_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN24_A> for bool {
    #[inline(always)]
    fn from(variant: IN24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN24`"]
pub type IN24_R = crate::R<bool, IN24_A>;
impl IN24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN24_A {
        match self.bits {
            false => IN24_A::_0,
            true => IN24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN24_A::_1
    }
}
#[doc = "Port Data Input Value 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN25_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN25_A> for bool {
    #[inline(always)]
    fn from(variant: IN25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN25`"]
pub type IN25_R = crate::R<bool, IN25_A>;
impl IN25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN25_A {
        match self.bits {
            false => IN25_A::_0,
            true => IN25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN25_A::_1
    }
}
#[doc = "Port Data Input Value 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN26_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN26_A> for bool {
    #[inline(always)]
    fn from(variant: IN26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN26`"]
pub type IN26_R = crate::R<bool, IN26_A>;
impl IN26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN26_A {
        match self.bits {
            false => IN26_A::_0,
            true => IN26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN26_A::_1
    }
}
#[doc = "Port Data Input Value 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN27_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN27_A> for bool {
    #[inline(always)]
    fn from(variant: IN27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN27`"]
pub type IN27_R = crate::R<bool, IN27_A>;
impl IN27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN27_A {
        match self.bits {
            false => IN27_A::_0,
            true => IN27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN27_A::_1
    }
}
#[doc = "Port Data Input Value 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN28_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN28_A> for bool {
    #[inline(always)]
    fn from(variant: IN28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN28`"]
pub type IN28_R = crate::R<bool, IN28_A>;
impl IN28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN28_A {
        match self.bits {
            false => IN28_A::_0,
            true => IN28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN28_A::_1
    }
}
#[doc = "Port Data Input Value 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN29_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN29_A> for bool {
    #[inline(always)]
    fn from(variant: IN29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN29`"]
pub type IN29_R = crate::R<bool, IN29_A>;
impl IN29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN29_A {
        match self.bits {
            false => IN29_A::_0,
            true => IN29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN29_A::_1
    }
}
#[doc = "Port Data Input Value 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN30_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN30_A> for bool {
    #[inline(always)]
    fn from(variant: IN30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN30`"]
pub type IN30_R = crate::R<bool, IN30_A>;
impl IN30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN30_A {
        match self.bits {
            false => IN30_A::_0,
            true => IN30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN30_A::_1
    }
}
#[doc = "Port Data Input Value 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN31_A {
    #[doc = "0: Input 0"]
    _0 = 0,
    #[doc = "1: Input 1"]
    _1 = 1,
}
impl From<IN31_A> for bool {
    #[inline(always)]
    fn from(variant: IN31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN31`"]
pub type IN31_R = crate::R<bool, IN31_A>;
impl IN31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN31_A {
        match self.bits {
            false => IN31_A::_0,
            true => IN31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IN31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IN31_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Port Data Input Value 0"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Input Value 1"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Input Value 2"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Input Value 3"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Input Value 4"]
    #[inline(always)]
    pub fn in4(&self) -> IN4_R {
        IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Input Value 5"]
    #[inline(always)]
    pub fn in5(&self) -> IN5_R {
        IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Input Value 6"]
    #[inline(always)]
    pub fn in6(&self) -> IN6_R {
        IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Input Value 7"]
    #[inline(always)]
    pub fn in7(&self) -> IN7_R {
        IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Input Value 8"]
    #[inline(always)]
    pub fn in8(&self) -> IN8_R {
        IN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Input Value 9"]
    #[inline(always)]
    pub fn in9(&self) -> IN9_R {
        IN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Input Value 10"]
    #[inline(always)]
    pub fn in10(&self) -> IN10_R {
        IN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Input Value 11"]
    #[inline(always)]
    pub fn in11(&self) -> IN11_R {
        IN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Input Value 12"]
    #[inline(always)]
    pub fn in12(&self) -> IN12_R {
        IN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Input Value 13"]
    #[inline(always)]
    pub fn in13(&self) -> IN13_R {
        IN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Input Value 14"]
    #[inline(always)]
    pub fn in14(&self) -> IN14_R {
        IN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Input Value 15"]
    #[inline(always)]
    pub fn in15(&self) -> IN15_R {
        IN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Input Value 16"]
    #[inline(always)]
    pub fn in16(&self) -> IN16_R {
        IN16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Input Value 17"]
    #[inline(always)]
    pub fn in17(&self) -> IN17_R {
        IN17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Input Value 18"]
    #[inline(always)]
    pub fn in18(&self) -> IN18_R {
        IN18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Input Value 19"]
    #[inline(always)]
    pub fn in19(&self) -> IN19_R {
        IN19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Input Value 20"]
    #[inline(always)]
    pub fn in20(&self) -> IN20_R {
        IN20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Input Value 21"]
    #[inline(always)]
    pub fn in21(&self) -> IN21_R {
        IN21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Input Value 22"]
    #[inline(always)]
    pub fn in22(&self) -> IN22_R {
        IN22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Input Value 23"]
    #[inline(always)]
    pub fn in23(&self) -> IN23_R {
        IN23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Input Value 24"]
    #[inline(always)]
    pub fn in24(&self) -> IN24_R {
        IN24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Input Value 25"]
    #[inline(always)]
    pub fn in25(&self) -> IN25_R {
        IN25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Input Value 26"]
    #[inline(always)]
    pub fn in26(&self) -> IN26_R {
        IN26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Input Value 27"]
    #[inline(always)]
    pub fn in27(&self) -> IN27_R {
        IN27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Input Value 28"]
    #[inline(always)]
    pub fn in28(&self) -> IN28_R {
        IN28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Input Value 29"]
    #[inline(always)]
    pub fn in29(&self) -> IN29_R {
        IN29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Input Value 30"]
    #[inline(always)]
    pub fn in30(&self) -> IN30_R {
        IN30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Input Value 31"]
    #[inline(always)]
    pub fn in31(&self) -> IN31_R {
        IN31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
