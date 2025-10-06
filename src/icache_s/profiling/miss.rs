#[doc = "Register `MISS` reader"]
pub type R = crate::R<MissSpec>;
#[doc = "Field `MISSES` reader - Number of cache misses"]
pub type MissesR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of cache misses"]
    #[inline(always)]
    pub fn misses(&self) -> MissesR {
        MissesR::new(self.bits)
    }
}
#[doc = "The cache miss counter for cache region.\n\nYou can [`read`](crate::Reg::read) this register and get [`miss::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MissSpec;
impl crate::RegisterSpec for MissSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miss::R`](R) reader structure"]
impl crate::Readable for MissSpec {}
#[doc = "`reset()` method sets MISS to value 0"]
impl crate::Resettable for MissSpec {}
