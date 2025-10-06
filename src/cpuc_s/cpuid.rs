#[doc = "Register `CPUID` reader"]
pub type R = crate::R<CpuidSpec>;
#[doc = "Field `CPUID` reader - The CPU identifier."]
pub type CpuidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The CPU identifier."]
    #[inline(always)]
    pub fn cpuid(&self) -> CpuidR {
        CpuidR::new(self.bits)
    }
}
#[doc = "The identifier for the CPU in this subsystem.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuidSpec;
impl crate::RegisterSpec for CpuidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuid::R`](R) reader structure"]
impl crate::Readable for CpuidSpec {}
#[doc = "`reset()` method sets CPUID to value 0"]
impl crate::Resettable for CpuidSpec {}
