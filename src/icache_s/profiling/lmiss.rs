#[doc = "Register `LMISS` reader"]
pub type R = crate::R<LmissSpec>;
#[doc = "Field `LMISSES` reader - Number of cache line misses"]
pub type LmissesR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of cache line misses"]
    #[inline(always)]
    pub fn lmisses(&self) -> LmissesR {
        LmissesR::new(self.bits)
    }
}
#[doc = "The cache line miss counter for cache region.\n\nYou can [`read`](crate::Reg::read) this register and get [`lmiss::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LmissSpec;
impl crate::RegisterSpec for LmissSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lmiss::R`](R) reader structure"]
impl crate::Readable for LmissSpec {}
#[doc = "`reset()` method sets LMISS to value 0"]
impl crate::Resettable for LmissSpec {}
