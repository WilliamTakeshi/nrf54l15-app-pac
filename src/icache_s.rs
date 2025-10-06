#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    tasks_invalidatecache: TasksInvalidatecache,
    _reserved1: [u8; 0x08],
    tasks_invalidateline: TasksInvalidateline,
    _reserved2: [u8; 0x08],
    tasks_erase: TasksErase,
    _reserved3: [u8; 0x03dc],
    status: Status,
    enable: Enable,
    mode: Mode,
    _reserved6: [u8; 0x04],
    lineaddr: Lineaddr,
    profiling: Profiling,
    debuglock: Debuglock,
    writelock: Writelock,
}
impl RegisterBlock {
    #[doc = "0x08 - Invalidate the cache."]
    #[inline(always)]
    pub const fn tasks_invalidatecache(&self) -> &TasksInvalidatecache {
        &self.tasks_invalidatecache
    }
    #[doc = "0x14 - Invalidate the line."]
    #[inline(always)]
    pub const fn tasks_invalidateline(&self) -> &TasksInvalidateline {
        &self.tasks_invalidateline
    }
    #[doc = "0x20 - Erase the cache."]
    #[inline(always)]
    pub const fn tasks_erase(&self) -> &TasksErase {
        &self.tasks_erase
    }
    #[doc = "0x400 - Status of the cache activities."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x404 - Enable cache."]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x408 - Cache mode."]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x410 - Memory address covered by the line to be maintained."]
    #[inline(always)]
    pub const fn lineaddr(&self) -> &Lineaddr {
        &self.lineaddr
    }
    #[doc = "0x414..0x430 - Unspecified"]
    #[inline(always)]
    pub const fn profiling(&self) -> &Profiling {
        &self.profiling
    }
    #[doc = "0x430 - Lock debug mode."]
    #[inline(always)]
    pub const fn debuglock(&self) -> &Debuglock {
        &self.debuglock
    }
    #[doc = "0x434 - Lock cache updates."]
    #[inline(always)]
    pub const fn writelock(&self) -> &Writelock {
        &self.writelock
    }
}
#[doc = "TASKS_INVALIDATECACHE (w) register accessor: Invalidate the cache.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_invalidatecache::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_invalidatecache`] module"]
#[doc(alias = "TASKS_INVALIDATECACHE")]
pub type TasksInvalidatecache = crate::Reg<tasks_invalidatecache::TasksInvalidatecacheSpec>;
#[doc = "Invalidate the cache."]
pub mod tasks_invalidatecache;
#[doc = "TASKS_INVALIDATELINE (w) register accessor: Invalidate the line.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_invalidateline::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_invalidateline`] module"]
#[doc(alias = "TASKS_INVALIDATELINE")]
pub type TasksInvalidateline = crate::Reg<tasks_invalidateline::TasksInvalidatelineSpec>;
#[doc = "Invalidate the line."]
pub mod tasks_invalidateline;
#[doc = "TASKS_ERASE (w) register accessor: Erase the cache.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_erase::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_erase`] module"]
#[doc(alias = "TASKS_ERASE")]
pub type TasksErase = crate::Reg<tasks_erase::TasksEraseSpec>;
#[doc = "Erase the cache."]
pub mod tasks_erase;
#[doc = "STATUS (r) register accessor: Status of the cache activities.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status of the cache activities."]
pub mod status;
#[doc = "ENABLE (rw) register accessor: Enable cache.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable cache."]
pub mod enable;
#[doc = "MODE (rw) register accessor: Cache mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Cache mode."]
pub mod mode;
#[doc = "LINEADDR (rw) register accessor: Memory address covered by the line to be maintained.\n\nYou can [`read`](crate::Reg::read) this register and get [`lineaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lineaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lineaddr`] module"]
#[doc(alias = "LINEADDR")]
pub type Lineaddr = crate::Reg<lineaddr::LineaddrSpec>;
#[doc = "Memory address covered by the line to be maintained."]
pub mod lineaddr;
#[doc = "Unspecified"]
pub use self::profiling::Profiling;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod profiling;
#[doc = "DEBUGLOCK (rw) register accessor: Lock debug mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`debuglock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debuglock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debuglock`] module"]
#[doc(alias = "DEBUGLOCK")]
pub type Debuglock = crate::Reg<debuglock::DebuglockSpec>;
#[doc = "Lock debug mode."]
pub mod debuglock;
#[doc = "WRITELOCK (rw) register accessor: Lock cache updates.\n\nYou can [`read`](crate::Reg::read) this register and get [`writelock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writelock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writelock`] module"]
#[doc(alias = "WRITELOCK")]
pub type Writelock = crate::Reg<writelock::WritelockSpec>;
#[doc = "Lock cache updates."]
pub mod writelock;
