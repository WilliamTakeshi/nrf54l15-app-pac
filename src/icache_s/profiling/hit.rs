#[doc = "Register `HIT` reader"]
pub type R = crate::R<HitSpec>;
#[doc = "Field `HITS` reader - Number of cache hits"]
pub type HitsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of cache hits"]
    #[inline(always)]
    pub fn hits(&self) -> HitsR {
        HitsR::new(self.bits)
    }
}
#[doc = "The cache hit counter for cache region.\n\nYou can [`read`](crate::Reg::read) this register and get [`hit::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HitSpec;
impl crate::RegisterSpec for HitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hit::R`](R) reader structure"]
impl crate::Readable for HitSpec {}
#[doc = "`reset()` method sets HIT to value 0"]
impl crate::Resettable for HitSpec {}
