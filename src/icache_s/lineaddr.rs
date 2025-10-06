#[doc = "Register `LINEADDR` reader"]
pub type R = crate::R<LineaddrSpec>;
#[doc = "Register `LINEADDR` writer"]
pub type W = crate::W<LineaddrSpec>;
#[doc = "Field `ADDR` reader - Address."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, LineaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Memory address covered by the line to be maintained.\n\nYou can [`read`](crate::Reg::read) this register and get [`lineaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lineaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LineaddrSpec;
impl crate::RegisterSpec for LineaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lineaddr::R`](R) reader structure"]
impl crate::Readable for LineaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`lineaddr::W`](W) writer structure"]
impl crate::Writable for LineaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINEADDR to value 0"]
impl crate::Resettable for LineaddrSpec {}
