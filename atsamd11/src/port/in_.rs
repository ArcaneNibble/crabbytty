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
#[doc = "Port Data Input Value 1"]
pub type IN1_A = IN0_A;
#[doc = "Reader of field `IN1`"]
pub type IN1_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 2"]
pub type IN2_A = IN0_A;
#[doc = "Reader of field `IN2`"]
pub type IN2_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 3"]
pub type IN3_A = IN0_A;
#[doc = "Reader of field `IN3`"]
pub type IN3_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 4"]
pub type IN4_A = IN0_A;
#[doc = "Reader of field `IN4`"]
pub type IN4_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 5"]
pub type IN5_A = IN0_A;
#[doc = "Reader of field `IN5`"]
pub type IN5_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 6"]
pub type IN6_A = IN0_A;
#[doc = "Reader of field `IN6`"]
pub type IN6_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 7"]
pub type IN7_A = IN0_A;
#[doc = "Reader of field `IN7`"]
pub type IN7_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 8"]
pub type IN8_A = IN0_A;
#[doc = "Reader of field `IN8`"]
pub type IN8_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 9"]
pub type IN9_A = IN0_A;
#[doc = "Reader of field `IN9`"]
pub type IN9_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 10"]
pub type IN10_A = IN0_A;
#[doc = "Reader of field `IN10`"]
pub type IN10_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 11"]
pub type IN11_A = IN0_A;
#[doc = "Reader of field `IN11`"]
pub type IN11_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 12"]
pub type IN12_A = IN0_A;
#[doc = "Reader of field `IN12`"]
pub type IN12_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 13"]
pub type IN13_A = IN0_A;
#[doc = "Reader of field `IN13`"]
pub type IN13_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 14"]
pub type IN14_A = IN0_A;
#[doc = "Reader of field `IN14`"]
pub type IN14_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 15"]
pub type IN15_A = IN0_A;
#[doc = "Reader of field `IN15`"]
pub type IN15_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 16"]
pub type IN16_A = IN0_A;
#[doc = "Reader of field `IN16`"]
pub type IN16_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 17"]
pub type IN17_A = IN0_A;
#[doc = "Reader of field `IN17`"]
pub type IN17_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 18"]
pub type IN18_A = IN0_A;
#[doc = "Reader of field `IN18`"]
pub type IN18_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 19"]
pub type IN19_A = IN0_A;
#[doc = "Reader of field `IN19`"]
pub type IN19_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 20"]
pub type IN20_A = IN0_A;
#[doc = "Reader of field `IN20`"]
pub type IN20_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 21"]
pub type IN21_A = IN0_A;
#[doc = "Reader of field `IN21`"]
pub type IN21_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 22"]
pub type IN22_A = IN0_A;
#[doc = "Reader of field `IN22`"]
pub type IN22_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 23"]
pub type IN23_A = IN0_A;
#[doc = "Reader of field `IN23`"]
pub type IN23_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 24"]
pub type IN24_A = IN0_A;
#[doc = "Reader of field `IN24`"]
pub type IN24_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 25"]
pub type IN25_A = IN0_A;
#[doc = "Reader of field `IN25`"]
pub type IN25_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 26"]
pub type IN26_A = IN0_A;
#[doc = "Reader of field `IN26`"]
pub type IN26_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 27"]
pub type IN27_A = IN0_A;
#[doc = "Reader of field `IN27`"]
pub type IN27_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 28"]
pub type IN28_A = IN0_A;
#[doc = "Reader of field `IN28`"]
pub type IN28_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 29"]
pub type IN29_A = IN0_A;
#[doc = "Reader of field `IN29`"]
pub type IN29_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 30"]
pub type IN30_A = IN0_A;
#[doc = "Reader of field `IN30`"]
pub type IN30_R = crate::R<bool, IN0_A>;
#[doc = "Port Data Input Value 31"]
pub type IN31_A = IN0_A;
#[doc = "Reader of field `IN31`"]
pub type IN31_R = crate::R<bool, IN0_A>;
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
