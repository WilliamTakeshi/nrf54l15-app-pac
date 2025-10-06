#[doc = "Register `READS` reader"]
pub type R = crate::R<ReadsSpec>;
#[doc = "Field `READS` reader - Number of reads for cache region."]
pub type ReadsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of reads for cache region."]
    #[inline(always)]
    pub fn reads(&self) -> ReadsR {
        ReadsR::new(self.bits)
    }
}
#[doc = "Number of reads for cache region.\n\nYou can [`read`](crate::Reg::read) this register and get [`reads::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadsSpec;
impl crate::RegisterSpec for ReadsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reads::R`](R) reader structure"]
impl crate::Readable for ReadsSpec {}
#[doc = "`reset()` method sets READS to value 0"]
impl crate::Resettable for ReadsSpec {}
