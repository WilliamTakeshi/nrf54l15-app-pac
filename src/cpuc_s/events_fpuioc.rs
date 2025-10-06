#[doc = "Register `EVENTS_FPUIOC` reader"]
pub type R = crate::R<EventsFpuiocSpec>;
#[doc = "Register `EVENTS_FPUIOC` writer"]
pub type W = crate::W<EventsFpuiocSpec>;
#[doc = "An invalid operation exception has occurred in the FPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsFpuioc {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsFpuioc> for bool {
    #[inline(always)]
    fn from(variant: EventsFpuioc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_FPUIOC` reader - An invalid operation exception has occurred in the FPU."]
pub type EventsFpuiocR = crate::BitReader<EventsFpuioc>;
impl EventsFpuiocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsFpuioc {
        match self.bits {
            false => EventsFpuioc::NotGenerated,
            true => EventsFpuioc::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsFpuioc::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsFpuioc::Generated
    }
}
#[doc = "Field `EVENTS_FPUIOC` writer - An invalid operation exception has occurred in the FPU."]
pub type EventsFpuiocW<'a, REG> = crate::BitWriter<'a, REG, EventsFpuioc>;
impl<'a, REG> EventsFpuiocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpuioc::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFpuioc::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - An invalid operation exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpuioc(&self) -> EventsFpuiocR {
        EventsFpuiocR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An invalid operation exception has occurred in the FPU."]
    #[inline(always)]
    pub fn events_fpuioc(&mut self) -> EventsFpuiocW<'_, EventsFpuiocSpec> {
        EventsFpuiocW::new(self, 0)
    }
}
#[doc = "An invalid operation exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpuioc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpuioc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFpuiocSpec;
impl crate::RegisterSpec for EventsFpuiocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_fpuioc::R`](R) reader structure"]
impl crate::Readable for EventsFpuiocSpec {}
#[doc = "`write(|w| ..)` method takes [`events_fpuioc::W`](W) writer structure"]
impl crate::Writable for EventsFpuiocSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_FPUIOC to value 0"]
impl crate::Resettable for EventsFpuiocSpec {}
