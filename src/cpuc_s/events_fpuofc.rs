#[doc = "Register `EVENTS_FPUOFC` reader"]
pub type R = crate::R<EventsFpuofcSpec>;
#[doc = "Register `EVENTS_FPUOFC` writer"]
pub type W = crate::W<EventsFpuofcSpec>;
#[doc = "A floating-point overflow exception has occurred in the FPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsFpuofc {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsFpuofc> for bool {
    #[inline(always)]
    fn from(variant: EventsFpuofc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_FPUOFC` reader - A floating-point overflow exception has occurred in the FPU."]
pub type EventsFpuofcR = crate::BitReader<EventsFpuofc>;
impl EventsFpuofcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsFpuofc {
        match self.bits {
            false => EventsFpuofc::NotGenerated,
            true => EventsFpuofc::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsFpuofc::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsFpuofc::Generated
    }
}
#[doc = "Field `EVENTS_FPUOFC` writer - A floating-point overflow exception has occurred in the FPU."]
pub type EventsFpuofcW<'a, REG> = crate::BitWriter<'a, REG, EventsFpuofc>;
impl<'a, REG> EventsFpuofcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpuofc::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpuofc::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A floating-point overflow exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpuofc(&self) -> EventsFpuofcR {
        EventsFpuofcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A floating-point overflow exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpuofc(&mut self) -> EventsFpuofcW<'_, EventsFpuofcSpec> {
        EventsFpuofcW::new(self, 0)
    }
}
#[doc = "A floating-point overflow exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpuofc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpuofc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFpuofcSpec;
impl crate::RegisterSpec for EventsFpuofcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_fpuofc::R`](R) reader structure"]
impl crate::Readable for EventsFpuofcSpec {}
#[doc = "`write(|w| ..)` method takes [`events_fpuofc::W`](W) writer structure"]
impl crate::Writable for EventsFpuofcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_FPUOFC to value 0"]
impl crate::Resettable for EventsFpuofcSpec {}
