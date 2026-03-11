#[doc = "Register `DMA_EN` writer"]
pub type W = crate::W<DmaEnSpec>;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<DmaEnSpec> {
        DmaenW::new(self, 0)
    }
}
#[doc = "CRCCU DMA Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_en::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaEnSpec;
impl crate::RegisterSpec for DmaEnSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_en::W`](W) writer structure"]
impl crate::Writable for DmaEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
