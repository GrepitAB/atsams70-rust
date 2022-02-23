#[doc = "Register `GMAC_TXLPITIME` reader"]
pub struct R(crate::R<GMAC_TXLPITIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_TXLPITIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_TXLPITIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_TXLPITIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPITIME` reader - Time in LPI (cleared on read)"]
pub struct LPITIME_R(crate::FieldReader<u32, u32>);
impl LPITIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LPITIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPITIME_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Time in LPI (cleared on read)"]
    #[inline(always)]
    pub fn lpitime(&self) -> LPITIME_R {
        LPITIME_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "Transmit LPI Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_txlpitime](index.html) module"]
pub struct GMAC_TXLPITIME_SPEC;
impl crate::RegisterSpec for GMAC_TXLPITIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_txlpitime::R](R) reader structure"]
impl crate::Readable for GMAC_TXLPITIME_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_TXLPITIME to value 0"]
impl crate::Resettable for GMAC_TXLPITIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
