#[doc = "Reader of register EPSTATUS%s"]
pub type R = crate::R<u8, super::EPSTATUS>;
#[doc = "Data Toggle Out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTGLOUT_A {
    #[doc = "0: Next is DATA0"]
    DATA0 = 0,
    #[doc = "1: Next is DATA1"]
    DATA1 = 1,
}
impl From<DTGLOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DTGLOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTGLOUT`"]
pub type DTGLOUT_R = crate::R<bool, DTGLOUT_A>;
impl DTGLOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTGLOUT_A {
        match self.bits {
            false => DTGLOUT_A::DATA0,
            true => DTGLOUT_A::DATA1,
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DTGLOUT_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DTGLOUT_A::DATA1
    }
}
#[doc = "Data Toggle In\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTGLIN_A {
    #[doc = "0: Next is DATA0"]
    DATA0 = 0,
    #[doc = "1: Next is DATA1"]
    DATA1 = 1,
}
impl From<DTGLIN_A> for bool {
    #[inline(always)]
    fn from(variant: DTGLIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTGLIN`"]
pub type DTGLIN_R = crate::R<bool, DTGLIN_A>;
impl DTGLIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTGLIN_A {
        match self.bits {
            false => DTGLIN_A::DATA0,
            true => DTGLIN_A::DATA1,
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DTGLIN_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DTGLIN_A::DATA1
    }
}
#[doc = "Current Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURBK_A {
    #[doc = "0: Next is Bank0"]
    BANK0 = 0,
    #[doc = "1: Next is Bank1"]
    BANK1 = 1,
}
impl From<CURBK_A> for bool {
    #[inline(always)]
    fn from(variant: CURBK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CURBK`"]
pub type CURBK_R = crate::R<bool, CURBK_A>;
impl CURBK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURBK_A {
        match self.bits {
            false => CURBK_A::BANK0,
            true => CURBK_A::BANK1,
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == CURBK_A::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == CURBK_A::BANK1
    }
}
#[doc = "Stall 0 Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALLRQ0_A {
    #[doc = "0: No stall reply"]
    NOSTALL = 0,
    #[doc = "1: Stall reply"]
    STALL = 1,
}
impl From<STALLRQ0_A> for bool {
    #[inline(always)]
    fn from(variant: STALLRQ0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STALLRQ0`"]
pub type STALLRQ0_R = crate::R<bool, STALLRQ0_A>;
impl STALLRQ0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALLRQ0_A {
        match self.bits {
            false => STALLRQ0_A::NOSTALL,
            true => STALLRQ0_A::STALL,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTALL`"]
    #[inline(always)]
    pub fn is_nostall(&self) -> bool {
        *self == STALLRQ0_A::NOSTALL
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STALLRQ0_A::STALL
    }
}
#[doc = "Stall 1 Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALLRQ1_A {
    #[doc = "0: No stall reply"]
    NOSTALL = 0,
    #[doc = "1: Stall reply"]
    STALL = 1,
}
impl From<STALLRQ1_A> for bool {
    #[inline(always)]
    fn from(variant: STALLRQ1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STALLRQ1`"]
pub type STALLRQ1_R = crate::R<bool, STALLRQ1_A>;
impl STALLRQ1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALLRQ1_A {
        match self.bits {
            false => STALLRQ1_A::NOSTALL,
            true => STALLRQ1_A::STALL,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTALL`"]
    #[inline(always)]
    pub fn is_nostall(&self) -> bool {
        *self == STALLRQ1_A::NOSTALL
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STALLRQ1_A::STALL
    }
}
#[doc = "Bank 0 ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK0RDY_A {
    #[doc = "0: Not ready"]
    NOTREADY = 0,
    #[doc = "1: Ready"]
    READY = 1,
}
impl From<BK0RDY_A> for bool {
    #[inline(always)]
    fn from(variant: BK0RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BK0RDY`"]
pub type BK0RDY_R = crate::R<bool, BK0RDY_A>;
impl BK0RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK0RDY_A {
        match self.bits {
            false => BK0RDY_A::NOTREADY,
            true => BK0RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == BK0RDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == BK0RDY_A::READY
    }
}
#[doc = "Bank 1 ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK1RDY_A {
    #[doc = "0: Not ready"]
    NOTREADY = 0,
    #[doc = "1: Ready"]
    READY = 1,
}
impl From<BK1RDY_A> for bool {
    #[inline(always)]
    fn from(variant: BK1RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BK1RDY`"]
pub type BK1RDY_R = crate::R<bool, BK1RDY_A>;
impl BK1RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK1RDY_A {
        match self.bits {
            false => BK1RDY_A::NOTREADY,
            true => BK1RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == BK1RDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == BK1RDY_A::READY
    }
}
impl R {
    #[doc = "Bit 0 - Data Toggle Out"]
    #[inline(always)]
    pub fn dtglout(&self) -> DTGLOUT_R {
        DTGLOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Toggle In"]
    #[inline(always)]
    pub fn dtglin(&self) -> DTGLIN_R {
        DTGLIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Current Bank"]
    #[inline(always)]
    pub fn curbk(&self) -> CURBK_R {
        CURBK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stall 0 Request"]
    #[inline(always)]
    pub fn stallrq0(&self) -> STALLRQ0_R {
        STALLRQ0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stall 1 Request"]
    #[inline(always)]
    pub fn stallrq1(&self) -> STALLRQ1_R {
        STALLRQ1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bank 0 ready"]
    #[inline(always)]
    pub fn bk0rdy(&self) -> BK0RDY_R {
        BK0RDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bank 1 ready"]
    #[inline(always)]
    pub fn bk1rdy(&self) -> BK1RDY_R {
        BK1RDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
