#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICM_IER {
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Proxy"]
pub struct _RHCW<'a> {
    w: &'a mut W,
}
impl<'a> _RHCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RDMW<'a> {
    w: &'a mut W,
}
impl<'a> _RDMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RBEW<'a> {
    w: &'a mut W,
}
impl<'a> _RBEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RWCW<'a> {
    w: &'a mut W,
}
impl<'a> _RWCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RECW<'a> {
    w: &'a mut W,
}
impl<'a> _RECW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RSUW<'a> {
    w: &'a mut W,
}
impl<'a> _RSUW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _URADW<'a> {
    w: &'a mut W,
}
impl<'a> _URADW<'a> {
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Enable"]
    #[inline(always)]
    pub fn rhc(&mut self) -> _RHCW {
        _RHCW { w: self }
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Enable"]
    #[inline(always)]
    pub fn rdm(&mut self) -> _RDMW {
        _RDMW { w: self }
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn rbe(&mut self) -> _RBEW {
        _RBEW { w: self }
    }
    #[doc = "Bits 12:15 - Region Wrap Condition detected Interrupt Enable"]
    #[inline(always)]
    pub fn rwc(&mut self) -> _RWCW {
        _RWCW { w: self }
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected Interrupt Enable"]
    #[inline(always)]
    pub fn rec(&mut self) -> _RECW {
        _RECW { w: self }
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Disable"]
    #[inline(always)]
    pub fn rsu(&mut self) -> _RSUW {
        _RSUW { w: self }
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Enable"]
    #[inline(always)]
    pub fn urad(&mut self) -> _URADW {
        _URADW { w: self }
    }
}
