#[doc = "Register `TASKS_PUSH` writer"]
pub type W = crate::W<TasksPushSpec>;
#[doc = "Push key slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksPush {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksPush> for bool {
    #[inline(always)]
    fn from(variant: TasksPush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PUSH` writer - Push key slot"]
pub type TasksPushW<'a, REG> = crate::BitWriter<'a, REG, TasksPush>;
impl<'a, REG> TasksPushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksPush::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Push key slot"]
    #[inline(always)]
    pub fn tasks_push(&mut self) -> TasksPushW<'_, TasksPushSpec> {
        TasksPushW::new(self, 0)
    }
}
#[doc = "Push key slot\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_push::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPushSpec;
impl crate::RegisterSpec for TasksPushSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_push::W`](W) writer structure"]
impl crate::Writable for TasksPushSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_PUSH to value 0"]
impl crate::Resettable for TasksPushSpec {}
