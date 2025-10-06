#[doc = "Register `WRITES` reader"]
pub type R = crate::R<WritesSpec>;
#[doc = "Field `WRITES` reader - Number of writes for cache region."]
pub type WritesR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of writes for cache region."]
    #[inline(always)]
    pub fn writes(&self) -> WritesR {
        WritesR::new(self.bits)
    }
}
#[doc = "Number of writes for cache region.\n\nYou can [`read`](crate::Reg::read) this register and get [`writes::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WritesSpec;
impl crate::RegisterSpec for WritesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writes::R`](R) reader structure"]
impl crate::Readable for WritesSpec {}
#[doc = "`reset()` method sets WRITES to value 0"]
impl crate::Resettable for WritesSpec {}
