#[doc = "Reader of register PCLKSR"]
pub type R = crate::R<u32, super::PCLKSR>;
#[doc = "Reader of field `XOSCRDY`"]
pub type XOSCRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC32KRDY`"]
pub type XOSC32KRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `OSC32KRDY`"]
pub type OSC32KRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `OSC8MRDY`"]
pub type OSC8MRDY_R = crate::R<bool, bool>;
#[doc = "DFLL Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFLLRDY_A {
    #[doc = "0: Not ready"]
    NOTREADY,
    #[doc = "1: Ready"]
    READY,
}
impl From<DFLLRDY_A> for bool {
    #[inline(always)]
    fn from(variant: DFLLRDY_A) -> Self {
        match variant {
            DFLLRDY_A::NOTREADY => false,
            DFLLRDY_A::READY => true,
        }
    }
}
#[doc = "Reader of field `DFLLRDY`"]
pub type DFLLRDY_R = crate::R<bool, DFLLRDY_A>;
impl DFLLRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFLLRDY_A {
        match self.bits {
            false => DFLLRDY_A::NOTREADY,
            true => DFLLRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == DFLLRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == DFLLRDY_A::READY
    }
}
#[doc = "DFLL Out Of Bounds\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFLLOOB_A {
    #[doc = "0: Not out of bounds"]
    NOOOB,
    #[doc = "1: Out of bounds"]
    OOB,
}
impl From<DFLLOOB_A> for bool {
    #[inline(always)]
    fn from(variant: DFLLOOB_A) -> Self {
        match variant {
            DFLLOOB_A::NOOOB => false,
            DFLLOOB_A::OOB => true,
        }
    }
}
#[doc = "Reader of field `DFLLOOB`"]
pub type DFLLOOB_R = crate::R<bool, DFLLOOB_A>;
impl DFLLOOB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFLLOOB_A {
        match self.bits {
            false => DFLLOOB_A::NOOOB,
            true => DFLLOOB_A::OOB,
        }
    }
    #[doc = "Checks if the value of the field is `NOOOB`"]
    #[inline(always)]
    pub fn is_nooob(&self) -> bool {
        *self == DFLLOOB_A::NOOOB
    }
    #[doc = "Checks if the value of the field is `OOB`"]
    #[inline(always)]
    pub fn is_oob(&self) -> bool {
        *self == DFLLOOB_A::OOB
    }
}
#[doc = "DFLL Lock Fine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFLLLCKF_A {
    #[doc = "0: Not locked"]
    UNLOCKED,
    #[doc = "1: Locked"]
    LOCKED,
}
impl From<DFLLLCKF_A> for bool {
    #[inline(always)]
    fn from(variant: DFLLLCKF_A) -> Self {
        match variant {
            DFLLLCKF_A::UNLOCKED => false,
            DFLLLCKF_A::LOCKED => true,
        }
    }
}
#[doc = "Reader of field `DFLLLCKF`"]
pub type DFLLLCKF_R = crate::R<bool, DFLLLCKF_A>;
impl DFLLLCKF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFLLLCKF_A {
        match self.bits {
            false => DFLLLCKF_A::UNLOCKED,
            true => DFLLLCKF_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DFLLLCKF_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DFLLLCKF_A::LOCKED
    }
}
#[doc = "DFLL Lock Coarse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFLLLCKC_A {
    #[doc = "0: Not locked"]
    UNLOCKED,
    #[doc = "1: Locked"]
    LOCKED,
}
impl From<DFLLLCKC_A> for bool {
    #[inline(always)]
    fn from(variant: DFLLLCKC_A) -> Self {
        match variant {
            DFLLLCKC_A::UNLOCKED => false,
            DFLLLCKC_A::LOCKED => true,
        }
    }
}
#[doc = "Reader of field `DFLLLCKC`"]
pub type DFLLLCKC_R = crate::R<bool, DFLLLCKC_A>;
impl DFLLLCKC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFLLLCKC_A {
        match self.bits {
            false => DFLLLCKC_A::UNLOCKED,
            true => DFLLLCKC_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DFLLLCKC_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DFLLLCKC_A::LOCKED
    }
}
#[doc = "DFLL Reference Clock Stopped\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFLLRCS_A {
    #[doc = "0: Running"]
    RUNNING,
    #[doc = "1: Stopped"]
    STOPPED,
}
impl From<DFLLRCS_A> for bool {
    #[inline(always)]
    fn from(variant: DFLLRCS_A) -> Self {
        match variant {
            DFLLRCS_A::RUNNING => false,
            DFLLRCS_A::STOPPED => true,
        }
    }
}
#[doc = "Reader of field `DFLLRCS`"]
pub type DFLLRCS_R = crate::R<bool, DFLLRCS_A>;
impl DFLLRCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFLLRCS_A {
        match self.bits {
            false => DFLLRCS_A::RUNNING,
            true => DFLLRCS_A::STOPPED,
        }
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == DFLLRCS_A::RUNNING
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == DFLLRCS_A::STOPPED
    }
}
#[doc = "Reader of field `BOD33RDY`"]
pub type BOD33RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD33DET`"]
pub type BOD33DET_R = crate::R<bool, bool>;
#[doc = "Reader of field `B33SRDY`"]
pub type B33SRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLCKR`"]
pub type DPLLLCKR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLCKF`"]
pub type DPLLLCKF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLTO`"]
pub type DPLLLTO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OSC32K Ready"]
    #[inline(always)]
    pub fn osc32krdy(&self) -> OSC32KRDY_R {
        OSC32KRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OSC8M Ready"]
    #[inline(always)]
    pub fn osc8mrdy(&self) -> OSC8MRDY_R {
        OSC8MRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DFLLRDY_R {
        DFLLRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds"]
    #[inline(always)]
    pub fn dflloob(&self) -> DFLLOOB_R {
        DFLLOOB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DFLL Lock Fine"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DFLLLCKF_R {
        DFLLLCKF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFLL Lock Coarse"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DFLLLCKC_R {
        DFLLLCKC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DFLLRCS_R {
        DFLLRCS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> BOD33RDY_R {
        BOD33RDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33SRDY_R {
        B33SRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Timeout"]
    #[inline(always)]
    pub fn dplllto(&self) -> DPLLLTO_R {
        DPLLLTO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
