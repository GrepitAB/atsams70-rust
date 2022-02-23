#[doc = "Register `ISI_DMA_C_ADDR` reader"]
pub struct R(crate::R<ISI_DMA_C_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_DMA_C_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_DMA_C_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_DMA_C_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISI_DMA_C_ADDR` writer"]
pub struct W(crate::W<ISI_DMA_C_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_DMA_C_ADDR_SPEC>;
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
impl From<crate::W<ISI_DMA_C_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_DMA_C_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C_ADDR` reader - Codec Image Base Address"]
pub struct C_ADDR_R(crate::FieldReader<u32, u32>);
impl C_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        C_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_ADDR` writer - Codec Image Base Address"]
pub struct C_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> C_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Codec Image Base Address"]
    #[inline(always)]
    pub fn c_addr(&self) -> C_ADDR_R {
        C_ADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Codec Image Base Address"]
    #[inline(always)]
    pub fn c_addr(&mut self) -> C_ADDR_W {
        C_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Codec Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_c_addr](index.html) module"]
pub struct ISI_DMA_C_ADDR_SPEC;
impl crate::RegisterSpec for ISI_DMA_C_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_dma_c_addr::R](R) reader structure"]
impl crate::Readable for ISI_DMA_C_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isi_dma_c_addr::W](W) writer structure"]
impl crate::Writable for ISI_DMA_C_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_DMA_C_ADDR to value 0"]
impl crate::Resettable for ISI_DMA_C_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
