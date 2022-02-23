#[doc = "Register `I2SC_MR` reader"]
pub struct R(crate::R<I2SC_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SC_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SC_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SC_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SC_MR` writer"]
pub struct W(crate::W<I2SC_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SC_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<I2SC_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SC_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Inter-IC Sound Controller Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: I2SC_CK and I2SC_WS pin inputs used as bit clock and word select/frame synchronization."]
    SLAVE = 0,
    #[doc = "1: Bit clock and word select/frame synchronization generated by I2SC from MCK and output to I2SC_CK and I2SC_WS pins. Peripheral clock or GCLK is output as master clock on I2SC_MCK if I2SC_MR.IMCKMODE is set."]
    MASTER = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Inter-IC Sound Controller Mode"]
pub struct MODE_R(crate::FieldReader<bool, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::SLAVE,
            true => MODE_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        **self == MODE_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        **self == MODE_A::MASTER
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Inter-IC Sound Controller Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2SC_CK and I2SC_WS pin inputs used as bit clock and word select/frame synchronization."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MODE_A::SLAVE)
    }
    #[doc = "Bit clock and word select/frame synchronization generated by I2SC from MCK and output to I2SC_CK and I2SC_WS pins. Peripheral clock or GCLK is output as master clock on I2SC_MCK if I2SC_MR.IMCKMODE is set."]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MODE_A::MASTER)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Data Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATALENGTH_A {
    #[doc = "0: Data length is set to 32 bits"]
    _32_BITS = 0,
    #[doc = "1: Data length is set to 24 bits"]
    _24_BITS = 1,
    #[doc = "2: Data length is set to 20 bits"]
    _20_BITS = 2,
    #[doc = "3: Data length is set to 18 bits"]
    _18_BITS = 3,
    #[doc = "4: Data length is set to 16 bits"]
    _16_BITS = 4,
    #[doc = "5: Data length is set to 16-bit compact stereo. Left sample in bits 15:0 and right sample in bits 31:16 of same word."]
    _16_BITS_COMPACT = 5,
    #[doc = "6: Data length is set to 8 bits"]
    _8_BITS = 6,
    #[doc = "7: Data length is set to 8-bit compact stereo. Left sample in bits 7:0 and right sample in bits 15:8 of the same word."]
    _8_BITS_COMPACT = 7,
}
impl From<DATALENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATALENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATALENGTH` reader - Data Word Length"]
pub struct DATALENGTH_R(crate::FieldReader<u8, DATALENGTH_A>);
impl DATALENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATALENGTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATALENGTH_A {
        match self.bits {
            0 => DATALENGTH_A::_32_BITS,
            1 => DATALENGTH_A::_24_BITS,
            2 => DATALENGTH_A::_20_BITS,
            3 => DATALENGTH_A::_18_BITS,
            4 => DATALENGTH_A::_16_BITS,
            5 => DATALENGTH_A::_16_BITS_COMPACT,
            6 => DATALENGTH_A::_8_BITS,
            7 => DATALENGTH_A::_8_BITS_COMPACT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32_BITS`"]
    #[inline(always)]
    pub fn is_32_bits(&self) -> bool {
        **self == DATALENGTH_A::_32_BITS
    }
    #[doc = "Checks if the value of the field is `_24_BITS`"]
    #[inline(always)]
    pub fn is_24_bits(&self) -> bool {
        **self == DATALENGTH_A::_24_BITS
    }
    #[doc = "Checks if the value of the field is `_20_BITS`"]
    #[inline(always)]
    pub fn is_20_bits(&self) -> bool {
        **self == DATALENGTH_A::_20_BITS
    }
    #[doc = "Checks if the value of the field is `_18_BITS`"]
    #[inline(always)]
    pub fn is_18_bits(&self) -> bool {
        **self == DATALENGTH_A::_18_BITS
    }
    #[doc = "Checks if the value of the field is `_16_BITS`"]
    #[inline(always)]
    pub fn is_16_bits(&self) -> bool {
        **self == DATALENGTH_A::_16_BITS
    }
    #[doc = "Checks if the value of the field is `_16_BITS_COMPACT`"]
    #[inline(always)]
    pub fn is_16_bits_compact(&self) -> bool {
        **self == DATALENGTH_A::_16_BITS_COMPACT
    }
    #[doc = "Checks if the value of the field is `_8_BITS`"]
    #[inline(always)]
    pub fn is_8_bits(&self) -> bool {
        **self == DATALENGTH_A::_8_BITS
    }
    #[doc = "Checks if the value of the field is `_8_BITS_COMPACT`"]
    #[inline(always)]
    pub fn is_8_bits_compact(&self) -> bool {
        **self == DATALENGTH_A::_8_BITS_COMPACT
    }
}
impl core::ops::Deref for DATALENGTH_R {
    type Target = crate::FieldReader<u8, DATALENGTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATALENGTH` writer - Data Word Length"]
pub struct DATALENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATALENGTH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Data length is set to 32 bits"]
    #[inline(always)]
    pub fn _32_bits(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_32_BITS)
    }
    #[doc = "Data length is set to 24 bits"]
    #[inline(always)]
    pub fn _24_bits(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_24_BITS)
    }
    #[doc = "Data length is set to 20 bits"]
    #[inline(always)]
    pub fn _20_bits(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_20_BITS)
    }
    #[doc = "Data length is set to 18 bits"]
    #[inline(always)]
    pub fn _18_bits(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_18_BITS)
    }
    #[doc = "Data length is set to 16 bits"]
    #[inline(always)]
    pub fn _16_bits(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_16_BITS)
    }
    #[doc = "Data length is set to 16-bit compact stereo. Left sample in bits 15:0 and right sample in bits 31:16 of same word."]
    #[inline(always)]
    pub fn _16_bits_compact(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_16_BITS_COMPACT)
    }
    #[doc = "Data length is set to 8 bits"]
    #[inline(always)]
    pub fn _8_bits(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_8_BITS)
    }
    #[doc = "Data length is set to 8-bit compact stereo. Left sample in bits 7:0 and right sample in bits 15:8 of the same word."]
    #[inline(always)]
    pub fn _8_bits_compact(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_8_BITS_COMPACT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `RXMONO` reader - Receive Mono"]
pub struct RXMONO_R(crate::FieldReader<bool, bool>);
impl RXMONO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXMONO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMONO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMONO` writer - Receive Mono"]
pub struct RXMONO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMONO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RXDMA` reader - Single or Multiple DMA Controller Channels for Receiver"]
pub struct RXDMA_R(crate::FieldReader<bool, bool>);
impl RXDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMA` writer - Single or Multiple DMA Controller Channels for Receiver"]
pub struct RXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `RXLOOP` reader - Loopback Test Mode"]
pub struct RXLOOP_R(crate::FieldReader<bool, bool>);
impl RXLOOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXLOOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXLOOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLOOP` writer - Loopback Test Mode"]
pub struct RXLOOP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLOOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TXMONO` reader - Transmit Mono"]
pub struct TXMONO_R(crate::FieldReader<bool, bool>);
impl TXMONO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXMONO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMONO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMONO` writer - Transmit Mono"]
pub struct TXMONO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMONO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TXDMA` reader - Single or Multiple DMA Controller Channels for Transmitter"]
pub struct TXDMA_R(crate::FieldReader<bool, bool>);
impl TXDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMA` writer - Single or Multiple DMA Controller Channels for Transmitter"]
pub struct TXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TXSAME` reader - Transmit Data when Underrun"]
pub struct TXSAME_R(crate::FieldReader<bool, bool>);
impl TXSAME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXSAME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSAME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSAME` writer - Transmit Data when Underrun"]
pub struct TXSAME_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSAME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `IMCKDIV` reader - Selected Clock to I2SC Master Clock Ratio"]
pub struct IMCKDIV_R(crate::FieldReader<u8, u8>);
impl IMCKDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IMCKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMCKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMCKDIV` writer - Selected Clock to I2SC Master Clock Ratio"]
pub struct IMCKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> IMCKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Master Clock to fs Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IMCKFS_A {
    #[doc = "0: Sample frequency ratio set to 32"]
    M2SF32 = 0,
    #[doc = "1: Sample frequency ratio set to 64"]
    M2SF64 = 1,
    #[doc = "2: Sample frequency ratio set to 96"]
    M2SF96 = 2,
    #[doc = "3: Sample frequency ratio set to 128"]
    M2SF128 = 3,
    #[doc = "5: Sample frequency ratio set to 192"]
    M2SF192 = 5,
    #[doc = "7: Sample frequency ratio set to 256"]
    M2SF256 = 7,
    #[doc = "11: Sample frequency ratio set to 384"]
    M2SF384 = 11,
    #[doc = "15: Sample frequency ratio set to 512"]
    M2SF512 = 15,
    #[doc = "23: Sample frequency ratio set to 768"]
    M2SF768 = 23,
    #[doc = "31: Sample frequency ratio set to 1024"]
    M2SF1024 = 31,
    #[doc = "47: Sample frequency ratio set to 1536"]
    M2SF1536 = 47,
    #[doc = "63: Sample frequency ratio set to 2048"]
    M2SF2048 = 63,
}
impl From<IMCKFS_A> for u8 {
    #[inline(always)]
    fn from(variant: IMCKFS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IMCKFS` reader - Master Clock to fs Ratio"]
pub struct IMCKFS_R(crate::FieldReader<u8, IMCKFS_A>);
impl IMCKFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IMCKFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IMCKFS_A> {
        match self.bits {
            0 => Some(IMCKFS_A::M2SF32),
            1 => Some(IMCKFS_A::M2SF64),
            2 => Some(IMCKFS_A::M2SF96),
            3 => Some(IMCKFS_A::M2SF128),
            5 => Some(IMCKFS_A::M2SF192),
            7 => Some(IMCKFS_A::M2SF256),
            11 => Some(IMCKFS_A::M2SF384),
            15 => Some(IMCKFS_A::M2SF512),
            23 => Some(IMCKFS_A::M2SF768),
            31 => Some(IMCKFS_A::M2SF1024),
            47 => Some(IMCKFS_A::M2SF1536),
            63 => Some(IMCKFS_A::M2SF2048),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M2SF32`"]
    #[inline(always)]
    pub fn is_m2sf32(&self) -> bool {
        **self == IMCKFS_A::M2SF32
    }
    #[doc = "Checks if the value of the field is `M2SF64`"]
    #[inline(always)]
    pub fn is_m2sf64(&self) -> bool {
        **self == IMCKFS_A::M2SF64
    }
    #[doc = "Checks if the value of the field is `M2SF96`"]
    #[inline(always)]
    pub fn is_m2sf96(&self) -> bool {
        **self == IMCKFS_A::M2SF96
    }
    #[doc = "Checks if the value of the field is `M2SF128`"]
    #[inline(always)]
    pub fn is_m2sf128(&self) -> bool {
        **self == IMCKFS_A::M2SF128
    }
    #[doc = "Checks if the value of the field is `M2SF192`"]
    #[inline(always)]
    pub fn is_m2sf192(&self) -> bool {
        **self == IMCKFS_A::M2SF192
    }
    #[doc = "Checks if the value of the field is `M2SF256`"]
    #[inline(always)]
    pub fn is_m2sf256(&self) -> bool {
        **self == IMCKFS_A::M2SF256
    }
    #[doc = "Checks if the value of the field is `M2SF384`"]
    #[inline(always)]
    pub fn is_m2sf384(&self) -> bool {
        **self == IMCKFS_A::M2SF384
    }
    #[doc = "Checks if the value of the field is `M2SF512`"]
    #[inline(always)]
    pub fn is_m2sf512(&self) -> bool {
        **self == IMCKFS_A::M2SF512
    }
    #[doc = "Checks if the value of the field is `M2SF768`"]
    #[inline(always)]
    pub fn is_m2sf768(&self) -> bool {
        **self == IMCKFS_A::M2SF768
    }
    #[doc = "Checks if the value of the field is `M2SF1024`"]
    #[inline(always)]
    pub fn is_m2sf1024(&self) -> bool {
        **self == IMCKFS_A::M2SF1024
    }
    #[doc = "Checks if the value of the field is `M2SF1536`"]
    #[inline(always)]
    pub fn is_m2sf1536(&self) -> bool {
        **self == IMCKFS_A::M2SF1536
    }
    #[doc = "Checks if the value of the field is `M2SF2048`"]
    #[inline(always)]
    pub fn is_m2sf2048(&self) -> bool {
        **self == IMCKFS_A::M2SF2048
    }
}
impl core::ops::Deref for IMCKFS_R {
    type Target = crate::FieldReader<u8, IMCKFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMCKFS` writer - Master Clock to fs Ratio"]
pub struct IMCKFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IMCKFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMCKFS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sample frequency ratio set to 32"]
    #[inline(always)]
    pub fn m2sf32(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF32)
    }
    #[doc = "Sample frequency ratio set to 64"]
    #[inline(always)]
    pub fn m2sf64(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF64)
    }
    #[doc = "Sample frequency ratio set to 96"]
    #[inline(always)]
    pub fn m2sf96(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF96)
    }
    #[doc = "Sample frequency ratio set to 128"]
    #[inline(always)]
    pub fn m2sf128(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF128)
    }
    #[doc = "Sample frequency ratio set to 192"]
    #[inline(always)]
    pub fn m2sf192(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF192)
    }
    #[doc = "Sample frequency ratio set to 256"]
    #[inline(always)]
    pub fn m2sf256(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF256)
    }
    #[doc = "Sample frequency ratio set to 384"]
    #[inline(always)]
    pub fn m2sf384(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF384)
    }
    #[doc = "Sample frequency ratio set to 512"]
    #[inline(always)]
    pub fn m2sf512(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF512)
    }
    #[doc = "Sample frequency ratio set to 768"]
    #[inline(always)]
    pub fn m2sf768(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF768)
    }
    #[doc = "Sample frequency ratio set to 1024"]
    #[inline(always)]
    pub fn m2sf1024(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF1024)
    }
    #[doc = "Sample frequency ratio set to 1536"]
    #[inline(always)]
    pub fn m2sf1536(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF1536)
    }
    #[doc = "Sample frequency ratio set to 2048"]
    #[inline(always)]
    pub fn m2sf2048(self) -> &'a mut W {
        self.variant(IMCKFS_A::M2SF2048)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `IMCKMODE` reader - Master Clock Mode"]
pub struct IMCKMODE_R(crate::FieldReader<bool, bool>);
impl IMCKMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IMCKMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMCKMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMCKMODE` writer - Master Clock Mode"]
pub struct IMCKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IMCKMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `IWS` reader - I2SC_WS Slot Width"]
pub struct IWS_R(crate::FieldReader<bool, bool>);
impl IWS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IWS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWS` writer - I2SC_WS Slot Width"]
pub struct IWS_W<'a> {
    w: &'a mut W,
}
impl<'a> IWS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Inter-IC Sound Controller Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Data Word Length"]
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Receive Mono"]
    #[inline(always)]
    pub fn rxmono(&self) -> RXMONO_R {
        RXMONO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Single or Multiple DMA Controller Channels for Receiver"]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Loopback Test Mode"]
    #[inline(always)]
    pub fn rxloop(&self) -> RXLOOP_R {
        RXLOOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Mono"]
    #[inline(always)]
    pub fn txmono(&self) -> TXMONO_R {
        TXMONO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Single or Multiple DMA Controller Channels for Transmitter"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit Data when Underrun"]
    #[inline(always)]
    pub fn txsame(&self) -> TXSAME_R {
        TXSAME_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Selected Clock to I2SC Master Clock Ratio"]
    #[inline(always)]
    pub fn imckdiv(&self) -> IMCKDIV_R {
        IMCKDIV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Master Clock to fs Ratio"]
    #[inline(always)]
    pub fn imckfs(&self) -> IMCKFS_R {
        IMCKFS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Master Clock Mode"]
    #[inline(always)]
    pub fn imckmode(&self) -> IMCKMODE_R {
        IMCKMODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - I2SC_WS Slot Width"]
    #[inline(always)]
    pub fn iws(&self) -> IWS_R {
        IWS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Inter-IC Sound Controller Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 2:4 - Data Word Length"]
    #[inline(always)]
    pub fn datalength(&mut self) -> DATALENGTH_W {
        DATALENGTH_W { w: self }
    }
    #[doc = "Bit 8 - Receive Mono"]
    #[inline(always)]
    pub fn rxmono(&mut self) -> RXMONO_W {
        RXMONO_W { w: self }
    }
    #[doc = "Bit 9 - Single or Multiple DMA Controller Channels for Receiver"]
    #[inline(always)]
    pub fn rxdma(&mut self) -> RXDMA_W {
        RXDMA_W { w: self }
    }
    #[doc = "Bit 10 - Loopback Test Mode"]
    #[inline(always)]
    pub fn rxloop(&mut self) -> RXLOOP_W {
        RXLOOP_W { w: self }
    }
    #[doc = "Bit 12 - Transmit Mono"]
    #[inline(always)]
    pub fn txmono(&mut self) -> TXMONO_W {
        TXMONO_W { w: self }
    }
    #[doc = "Bit 13 - Single or Multiple DMA Controller Channels for Transmitter"]
    #[inline(always)]
    pub fn txdma(&mut self) -> TXDMA_W {
        TXDMA_W { w: self }
    }
    #[doc = "Bit 14 - Transmit Data when Underrun"]
    #[inline(always)]
    pub fn txsame(&mut self) -> TXSAME_W {
        TXSAME_W { w: self }
    }
    #[doc = "Bits 16:21 - Selected Clock to I2SC Master Clock Ratio"]
    #[inline(always)]
    pub fn imckdiv(&mut self) -> IMCKDIV_W {
        IMCKDIV_W { w: self }
    }
    #[doc = "Bits 24:29 - Master Clock to fs Ratio"]
    #[inline(always)]
    pub fn imckfs(&mut self) -> IMCKFS_W {
        IMCKFS_W { w: self }
    }
    #[doc = "Bit 30 - Master Clock Mode"]
    #[inline(always)]
    pub fn imckmode(&mut self) -> IMCKMODE_W {
        IMCKMODE_W { w: self }
    }
    #[doc = "Bit 31 - I2SC_WS Slot Width"]
    #[inline(always)]
    pub fn iws(&mut self) -> IWS_W {
        IWS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_mr](index.html) module"]
pub struct I2SC_MR_SPEC;
impl crate::RegisterSpec for I2SC_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sc_mr::R](R) reader structure"]
impl crate::Readable for I2SC_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sc_mr::W](W) writer structure"]
impl crate::Writable for I2SC_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SC_MR to value 0"]
impl crate::Resettable for I2SC_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
