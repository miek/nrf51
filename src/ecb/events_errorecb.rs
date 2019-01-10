#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_ERRORECB {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `EVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTR {
    #[doc = "Event signal clear"]
    CLEAR,
    #[doc = "Event signal set"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl EVENTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            EVENTR::CLEAR => 0,
            EVENTR::SET => 1,
            EVENTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> EVENTR {
        match value {
            0 => EVENTR::CLEAR,
            1 => EVENTR::SET,
            i => EVENTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == EVENTR::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == EVENTR::SET
    }
}
#[doc = "Values that can be written to the field `EVENT`"]
pub enum EVENTW {
    #[doc = "Event signal clear"]
    CLEAR,
    #[doc = "Event signal set"]
    SET,
}
impl EVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            EVENTW::CLEAR => 0,
            EVENTW::SET => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVENTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Event signal clear"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(EVENTW::CLEAR)
    }
    #[doc = "Event signal set"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(EVENTW::SET)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31"]
    #[inline]
    pub fn event(&self) -> EVENTR {
        EVENTR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31"]
    #[inline]
    pub fn event(&mut self) -> _EVENTW {
        _EVENTW { w: self }
    }
}
