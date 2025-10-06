#[doc = "Register `EVENTS_FPUDZC` reader"]
pub type R = crate::R<EventsFpudzcSpec>;
#[doc = "Register `EVENTS_FPUDZC` writer"]
pub type W = crate::W<EventsFpudzcSpec>;
#[doc = "A floating-point divide-by-zero exception has occurred in the FPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsFpudzc {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsFpudzc> for bool {
    #[inline(always)]
    fn from(variant: EventsFpudzc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_FPUDZC` reader - A floating-point divide-by-zero exception has occurred in the FPU."]
pub type EventsFpudzcR = crate::BitReader<EventsFpudzc>;
impl EventsFpudzcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsFpudzc {
        match self.bits {
            false => EventsFpudzc::NotGenerated,
            true => EventsFpudzc::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsFpudzc::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsFpudzc::Generated
    }
}
#[doc = "Field `EVENTS_FPUDZC` writer - A floating-point divide-by-zero exception has occurred in the FPU."]
pub type EventsFpudzcW<'a, REG> = crate::BitWriter<'a, REG, EventsFpudzc>;
impl<'a, REG> EventsFpudzcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpudzc::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpudzc::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A floating-point divide-by-zero exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpudzc(&self) -> EventsFpudzcR {
        EventsFpudzcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A floating-point divide-by-zero exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpudzc(&mut self) -> EventsFpudzcW<'_, EventsFpudzcSpec> {
        EventsFpudzcW::new(self, 0)
    }
}
#[doc = "A floating-point divide-by-zero exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpudzc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpudzc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFpudzcSpec;
impl crate::RegisterSpec for EventsFpudzcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_fpudzc::R`](R) reader structure"]
impl crate::Readable for EventsFpudzcSpec {}
#[doc = "`write(|w| ..)` method takes [`events_fpudzc::W`](W) writer structure"]
impl crate::Writable for EventsFpudzcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_FPUDZC to value 0"]
impl crate::Resettable for EventsFpudzcSpec {}
