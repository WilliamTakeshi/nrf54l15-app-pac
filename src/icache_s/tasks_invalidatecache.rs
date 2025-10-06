#[doc = "Register `TASKS_INVALIDATECACHE` writer"]
pub type W = crate::W<TasksInvalidatecacheSpec>;
#[doc = "Invalidate the cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksInvalidatecache {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksInvalidatecache> for bool {
    #[inline(always)]
    fn from(variant: TasksInvalidatecache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_INVALIDATECACHE` writer - Invalidate the cache."]
pub type TasksInvalidatecacheW<'a, REG> = crate::BitWriter<'a, REG, TasksInvalidatecache>;
impl<'a, REG> TasksInvalidatecacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksInvalidatecache::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Invalidate the cache."]
    #[inline(always)]
    pub fn tasks_invalidatecache(&mut self) -> TasksInvalidatecacheW<'_, TasksInvalidatecacheSpec> {
        TasksInvalidatecacheW::new(self, 0)
    }
}
#[doc = "Invalidate the cache.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_invalidatecache::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksInvalidatecacheSpec;
impl crate::RegisterSpec for TasksInvalidatecacheSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_invalidatecache::W`](W) writer structure"]
impl crate::Writable for TasksInvalidatecacheSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_INVALIDATECACHE to value 0"]
impl crate::Resettable for TasksInvalidatecacheSpec {}
